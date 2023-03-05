use proc_macro::TokenStream;

extern crate proc_macro;

mod driver_marker;

#[proc_macro_derive(DriverMarker)]
pub fn driver_marker_derive(input: TokenStream) -> TokenStream {
    driver_marker::driver_marker_derive(input)
}
