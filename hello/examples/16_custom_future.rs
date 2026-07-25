// Concept: Custom `Future` and `Poll` logic - building a tiny
// executor by hand, with no async runtime dependency.
//
// `async fn` is sugar over a hand-writable trait:
//
//     trait Future {
//         type Output;
//         fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
//     }
//
// A `Future` does nothing on its own - it is *polled*. Each poll
// either returns `Poll::Ready(value)` (done) or `Poll::Pending` (not
// yet), and in the `Pending` case the future is responsible for
// arranging a later wake-up by stashing the `Waker` from the given
// `Context` somewhere and calling `.wake()` on it once progress is
// possible again. An executor's whole job is: poll a future; if
// `Pending`, go do something else and wait to be told (via the
// waker) when to poll it again - this example implements exactly
// that loop, with a background thread standing in for "the OS timer
// interrupt that would drive a real async runtime".

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::Duration;

/// A future that becomes `Ready` after a background thread sleeps for
/// `duration` - a hand-written stand-in for `tokio::time::sleep`.
struct Delay {
    done: Arc<AtomicBool>,
    started: bool,
    duration: Duration,
}

impl Delay {
    fn new(duration: Duration) -> Self {
        Delay {
            done: Arc::new(AtomicBool::new(false)),
            started: false,
            duration,
        }
    }
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.done.load(Ordering::Acquire) {
            return Poll::Ready("delay complete");
        }

        if !self.started {
            self.started = true;
            let done = Arc::clone(&self.done);
            let waker: Waker = cx.waker().clone();
            let duration = self.duration;
            // Stands in for a real runtime's timer wheel: some other
            // part of the system will, at an unpredictable future
            // point, make progress possible and must remember to
            // call `wake()` - that's the entire contract a Future
            // relies on when it returns `Pending`.
            thread::spawn(move || {
                thread::sleep(duration);
                done.store(true, Ordering::Release);
                waker.wake();
            });
        }
        // Not ready yet: we've arranged (above) for `wake()` to be
        // called once the background thread finishes, so it's sound
        // to return `Pending` and let the executor move on.
        Poll::Pending
    }
}

/// A schedulable unit of work: a boxed future plus a way to
/// re-enqueue itself onto the ready queue when woken.
struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = &'static str> + Send>>>,
    ready_queue: SyncSender<Arc<Task>>,
}

// `Wake` is std's bridge from "an Arc<T> that knows how to
// reschedule itself" to a real `Waker` the futures API understands.
impl Wake for Task {
    fn wake(self: Arc<Self>) {
        // Waking a task just means: put it back on the executor's
        // ready queue so it gets polled again.
        let _ = self.ready_queue.send(self.clone());
    }
}

/// A deliberately minimal single-threaded executor: pull a task off
/// the queue, poll it, and rely on `Task::wake` to put it back on the
/// queue when it can make progress again.
struct MiniExecutor {
    ready_queue: Receiver<Arc<Task>>,
    spawner: SyncSender<Arc<Task>>,
}

impl MiniExecutor {
    fn new() -> Self {
        let (spawner, ready_queue) = sync_channel(16);
        MiniExecutor { ready_queue, spawner }
    }

    fn spawn(&self, future: impl Future<Output = &'static str> + Send + 'static) {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            ready_queue: self.spawner.clone(),
        });
        self.spawner.send(task).unwrap();
    }

    fn run(&self) {
        // Drop our own sender so `recv()` returns `Err` (queue
        // closed) once every in-flight task has finished and no
        // pending wake-ups remain, letting the loop terminate.
        while let Ok(task) = self.ready_queue.recv_timeout(Duration::from_millis(200)) {
            let waker: Waker = task.clone().into();
            let mut cx = Context::from_waker(&waker);
            let mut future_slot = task.future.lock().unwrap();
            if let Poll::Ready(value) = future_slot.as_mut().poll(&mut cx) {
                println!("task completed with: {value}");
            }
        }
    }
}

fn main() {
    let executor = MiniExecutor::new();
    executor.spawn(Delay::new(Duration::from_millis(30)));
    executor.spawn(Delay::new(Duration::from_millis(10)));
    println!("both futures spawned; executor now polls until each is Ready");
    executor.run();
    println!("executor drained the queue - all futures resolved");
}
