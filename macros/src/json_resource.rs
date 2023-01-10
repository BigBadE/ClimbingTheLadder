use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input, PathSegment, Type};
use syn::punctuated::Punctuated;
use syn::token::Colon2;

pub fn json_implement(item: TokenStream) -> TokenStream {
    //Get abstract syntax tree
    let ast = parse_macro_input!(item as DeriveInput);
    //Function header
    let mut output = String::from(
        format!("pub fn __load_{}(mut input: {}, value: &json::JsonValue) -> {} {{", ast.ident, ast.ident, ast.ident));

    //Get all fields from the struct
    let fields =
        if let syn::Data::Struct(
            syn::DataStruct {
                fields: syn::Fields::Named(ref fields),
                ..
            }) = ast.data
        {
            fields
        } else {
            panic!("Only support Struct")
        };

    for field in fields.named.iter() {
        match &field.ty {
            Type::Path(type_path) => {
                let segments = &type_path.path.segments;
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
                    }

                    //Load field from json
                    output += load_path(field_name, combine(segments).as_str()).as_str();

                    //Close if
                    if !required {
                        output += "}"
                    }
                }
            }
            //Panic if something else is here
            _ => panic!("Unknown type on field {}, did you mean to ignore it?", field.ident.as_ref().unwrap()),
        }
    }

    output += "return input;}";
    output.parse().unwrap()
}

fn combine(segments: &Punctuated<PathSegment, Colon2>) -> String {
    let mut output = String::new();
    for segment in segments {
        output += (segment.ident.to_string() + "::").as_str();
    }

    return String::from(&output[0..output.len() - 2]);
}

fn load_path(field_name: String, found_type: &str) -> String {
    //Add load depending on the value type
    return match found_type {
        "u8" => format!("input.{} = value[\"{}\"].as_u8().expect(\"No field {}\")", field_name, field_name, field_name),
        "u16" => format!("input.{} = value[\"{}\"].as_u16().expect(\"No field {}\")", field_name, field_name, field_name),
        "Duration" => format!("input.{} = Duration::from_nanos(value[\"{}\"].as_u64().expect(\"No field {}\"))", field_name, field_name, field_name),
        _ => format!("input.{} = {}::load(&value[\"{}\"])", field_name, found_type, found_type)
    };
}