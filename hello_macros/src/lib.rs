// Concept: Procedural macros.
//
// A proc macro is a function that runs *at compile time*, taking a
// stream of tokens as input and producing a new stream of tokens that
// gets spliced into the caller's source before the rest of the
// compiler ever sees it. `#[derive(Describe)]` below is a *derive
// macro*: applying it to a struct causes Rust to hand this crate the
// struct's token stream, we parse that into a syntax tree with
// `syn`, walk its fields, and use `quote!` to generate a brand-new
// `impl Describe for <Struct>` block as tokens, which the compiler
// then compiles as if we had hand-written it.
//
// Proc macros must live in their own crate with `proc-macro = true`
// in Cargo.toml, because they run as compiler plugins during the
// *build* of whatever crate uses them - they execute on the host
// machine, not the target.

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

/// Generates `impl Describe for T`, printing every named field as
/// `field_name: value` using each field's own `Debug` implementation
/// - equivalent in spirit to `#[derive(Debug)]`, hand-rolled to show
/// how the built-in derives actually work under the hood.
#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    // Parse the incoming tokens into a syntax tree describing exactly
    // one struct/enum/union declaration.
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let struct_name_str = struct_name.to_string();

    let Data::Struct(data_struct) = &ast.data else {
        return syn::Error::new_spanned(&ast, "Describe can only be derived for structs")
            .to_compile_error()
            .into();
    };
    let Fields::Named(named_fields) = &data_struct.fields else {
        return syn::Error::new_spanned(&ast, "Describe requires named fields")
            .to_compile_error()
            .into();
    };

    // For each field, generate a `format!("field: {:?}", self.field)`
    // fragment. This runs once, at compile time, over the struct's
    // *definition* - there is no runtime cost or reflection involved.
    let field_fragments = named_fields.named.iter().map(|field| {
        let field_ident = field.ident.as_ref().expect("named field");
        let field_name_str = field_ident.to_string();
        quote! {
            format!(concat!(#field_name_str, ": {:?}"), self.#field_ident)
        }
    });

    let expanded = quote! {
        impl Describe for #struct_name {
            fn describe(&self) -> String {
                let fields: Vec<String> = vec![#(#field_fragments),*];
                format!("{} {{ {} }}", #struct_name_str, fields.join(", "))
            }
        }
    };

    expanded.into()
}
