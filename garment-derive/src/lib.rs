use quote::quote;

#[proc_macro_derive(Diagnostic, attributes(diagnostic))]
pub fn derive_diagnostic(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    quote!().into()
}
