use proc_macro::TokenStream;
use syn::{DeriveInput, Fields, parse_macro_input, PathSegment, Type};
use syn::punctuated::Punctuated;
use syn::token::Colon2;

// This doesn't work with types that have generics, so you'll have to do it manually in the load function
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
                fields: Fields::Named(ref fields),
                ..
            }) = ast.data
        {
            fields
        } else {
            panic!("Only support Struct")
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
            }

            //Load field from json
            output += format!("input.{} = {}", field_name.clone(),
                              match_type(field_name, &field.ty)).as_str();

            //Close if
            if !required {
                output += "}"
            }
        }
    }

    output += "return input;}";
    output.parse().unwrap()
}

fn match_type(field_name: String, field_type: &Type) -> String {
    return match field_type {
        Type::Path(type_path) => load_path(field_name, combine(&type_path.path.segments).as_str()),
        Type::Tuple(type_tuple) => {
            let mut output = String::new();
            let mut i = 0;
            for element in &type_tuple.elems {
                let field_name = format!("{}.{}", field_name, i);
                output += &(match_type(field_name, element) + ",");
                i += 1;
            }
            format!("({})", &output[0..output.len()-1])
        }
        //Bare functions should be loaded already
        Type::BareFn(_) => format!("input.{}", field_name),
        //Panic if something else is here
        Type::Array(_) => panic!("Unhandled array on field {}, did you mean to ignore it?", field_name),
        Type::Group(_) => panic!("Unhandled group on field {}, did you mean to ignore it?", field_name),
        Type::ImplTrait(_) => panic!("Unhandled impl trait on field {}, did you mean to ignore it?", field_name),
        Type::Infer(_) => panic!("Unhandled infer on field {}, did you mean to ignore it?", field_name),
        Type::Macro(_) => panic!("Unhandled macro on field {}, did you mean to ignore it?", field_name),
        Type::Never(_) => panic!("Unhandled never on field {}, did you mean to ignore it?", field_name),
        Type::Paren(_) => panic!("Unhandled array on field {}, did you mean to ignore it?", field_name),
        Type::Ptr(_) => panic!("Unhandled array on field {}, did you mean to ignore it?", field_name),
        Type::Reference(_) => panic!("Unhandled array on field {}, did you mean to ignore it?", field_name),
        Type::Slice(_) => panic!("Unhandled slice on field {}, did you mean to ignore it?", field_name),
        Type::TraitObject(_) => panic!("Unhandled trait object on field {}, did you mean to ignore it?", field_name),
        Type::Verbatim(_) => panic!("Unhandled verbatim on field {}, did you mean to ignore it?", field_name),
        _ => panic!("Unknown type for field {}", field_name),
    };
}

fn combine(segments: &Punctuated<PathSegment, Colon2>) -> String {
    let mut output = String::new();
    for segment in segments {
        output += (segment.ident.to_string() + "::").as_str();
    }

    return String::from(&output[0..output.len() - 2]);
}

fn load_path(field_name: String, found_type: &str) -> String {
    if field_name == "keys" {
        panic!("{}", found_type);
    }

    //Add load depending on the value type
    return match found_type {
        "u8" => format!("value[\"{}\"].as_u8().expect(\"No field {}\")", field_name, field_name),
        "u16" => format!("value[\"{}\"].as_u16().expect(\"No field {}\")", field_name, field_name),
        "u64" => format!("value[\"{}\"].as_u64().expect(\"No field {}\")", field_name, field_name),
        "&str" => format!("value[\"{}\"].as_str().expect(\"No field {}\")", field_name, field_name),
        "String" => format!("String::from(value[\"{}\"].as_str().expect(\"No field {}\"))", field_name, field_name),
        "HashMap" => format!("value[\"{}\"].as_u64().expect(\"No field {}\")", field_name, field_name),
        "Duration" => format!("Duration::from_nanos(value[\"{}\"].as_u64().expect(\"No field {}\"))", field_name, field_name),
        _ => format!("{}::load(&value[\"{}\"])", found_type, found_type)
    };
}