extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(DriverMarker)]
pub fn driver_marker_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_driver_marker_macro(&ast)
}

fn impl_driver_marker_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let f = format!("{}", name);
    let gen = quote! {
        impl DriverMarker for #name {
            fn get_name(&self) -> &str {
                #f
            }

            fn add_to(&self, commands: &mut Commands, entity: Entity){
                commands
                    .entity(entity)
                    .insert(#name);
            }

            fn remove_from(&self, commands: &mut Commands, entity: Entity){
                commands
                    .entity(entity)
                    .remove::<#name>();
            }
        }
    };
    gen.into()
}