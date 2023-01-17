extern crate proc_macro;

use proc_macro::TokenStream;

mod json_resource;

#[proc_macro_derive(JsonResource, attributes(ignore_field, require_field))]
pub fn json_implement(item: TokenStream) -> TokenStream {
    return json_resource::json_implement(item);
}
