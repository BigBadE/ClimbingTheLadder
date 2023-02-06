use proc_macro::TokenStream;
use syn::{DeriveInput, Fields, parse_macro_input, PathSegment};
use syn::punctuated::Punctuated;
use syn::token::Colon2;

pub fn json_loading(item: TokenStream) -> TokenStream {
    //Get abstract syntax tree
    let ast = parse_macro_input!(item as DeriveInput);
    //Function header
    let mut output = String::from(
        format!("impl interfaces::loading::JsonLoadable for {} {{fn load(value: &json::JsonValue) -> Result<Self, anyhow::Error> {{let mut output = Self::default();",
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
        //Get attributes on field
        let mut ignore = false;
        let mut required = false;

        for attribute in &field.attrs {
            //Combine segments to get the attribute
            match combine(&attribute.path.segments).as_str() {
                "ignore_field" => ignore = true,
                "require_field" => required = true,
                _ => {}
            }
        }

        if !ignore {
            let field_name = field.ident.as_ref().unwrap().to_string();

            //If it's not required, check if it exists
            if !required {
                output += format!("if value.has_key(\"{}\") {{", field_name).as_str();
            } else {
                output += format!("if !value.has_key(\"{}\") {{ return Err(anyhow::Error::msg(format!(\"Missing required field {}: {{:?}}\", value))) }}", field_name, field_name).as_str();
            }

            //Load field from json
            output += format!("output.{} = interfaces::loading::JsonLoadable::load(&value[\"{}\"])?;", field_name.clone(),
                              field_name.clone()).as_str();

            //Close if
            if !required {
                output += "}"
            }
        }
    }

    output += "return Ok(output);}}";
    output.parse().unwrap()
}

fn combine(segments: &Punctuated<PathSegment, Colon2>) -> String {
    let mut output = String::new();
    for segment in segments {
        output += (segment.ident.to_string() + "::").as_str();
    }

    return String::from(&output[0..output.len() - 2]);
}