extern crate proc_macro;
mod window_info;
#[proc_macro_derive(FurinaWindowInfo, attributes(window_info))]
pub fn furina_window_info(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    window_info::window_info(input)
}
