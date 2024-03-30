use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(FormatRender)]
pub fn format_render(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_format_render_macro(&ast)
}

fn impl_format_render_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl FormatRender for #name {
			fn pretty(&self) -> String  { format!("{}: {}", self.id, self.name) }
			fn id_only(&self) -> String { format!("{}", self.id) }
			fn csv(&self) -> String    { format!("{},\"{}\"", self.id, self.name) }
        }
    };
    gen.into()
}
