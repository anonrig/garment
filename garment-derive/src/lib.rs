use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Diagnostic, attributes(diagnostic, label, help, related))]
pub fn derive_diagnostic(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let quote = quote! {
        impl garment::Diagnostic for #ident {
        }
    };

    proc_macro::TokenStream::from(quote)
}
