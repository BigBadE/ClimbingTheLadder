extern crate proc_macro;

use proc_macro::TokenStream;

mod json_loading;
mod json_saving;

#[proc_macro_derive(JsonLoadable, attributes(ignore_field, require_field))]
pub fn json_load(item: TokenStream) -> TokenStream {
    return json_loading::json_loading(item);
}

#[proc_macro_derive(JsonSaveable)]
pub fn json_save(item: TokenStream) -> TokenStream {
    return json_saving::json_saving(item);
}
