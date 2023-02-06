use proc_macro::TokenStream;
use syn::{DeriveInput, Fields, parse_macro_input};

pub fn json_saving(item: TokenStream) -> TokenStream {
    //Get abstract syntax tree
    let ast = parse_macro_input!(item as DeriveInput);
    //Function header
    let mut output = String::from(
        format!("impl interfaces::saving::JsonSaveable for {} {{fn save(&self) -> json::JsonValue {{let mut output = json::JsonValue::Object(json::object::Object::new());",
                ast.ident));

    //Get all fields from the struct
    let fields =
        if let syn::Data::Struct(
            syn::DataStruct {
                fields: Fields::Named(ref fields),
                ..
            }) = ast.data
        {
            fields
        } else {
            panic!("Derive macro only supports Structs!")
        };

    for field in fields.named.iter() {
        let field_name = field.ident.as_ref().unwrap().to_string();
        output += format!("output.insert(\"{}\", interfaces::saving::JsonSaveable::save(&self.{})).unwrap();", field_name, field_name).as_str();
    }

    output += "return output;}}";
    output.parse().unwrap()
}