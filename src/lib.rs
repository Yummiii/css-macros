use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use regex::Regex;
use std::{collections::HashMap, fs::read_to_string};
use syn::{parse_macro_input, Ident, ItemStruct, meta::parser};

use crate::args::CssAttributes;

extern crate proc_macro;

mod args;

#[proc_macro_attribute]
pub fn css(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut atribs = CssAttributes::default();
    let parser = parser(|x| atribs.parse(x));
    parse_macro_input!(args with parser);

    let ast = parse_macro_input!(item as ItemStruct);

    let attrs = &ast.attrs;
    let name = &ast.ident;
    let vis = &ast.vis;

    let css = read_to_string(atribs.path.unwrap().value()).unwrap();
    let classes = extract_classes(css);

    let (fields, initializer): (Vec<_>, Vec<_>) = classes
        .iter()
        .map(|prop| {
            let field_name = Ident::new(prop.0, Span::call_site());
            let a = prop.1;

            let field = quote! {
                pub #field_name: stylist::StyleSource,
            };

            let init = quote! {
                #field_name: stylist::css!(#a),
            };

            (field, init)
        })
        .unzip();

    quote! {
        #(#attrs)*
        #vis struct #name {
            #(#fields)*
        }

        impl Default for #name {
            fn default() -> Self {
                Self {
                    #(#initializer)*
                }
            }
        }
    }
    .into()
}

fn extract_classes(css_content: String) -> HashMap<String, String> {
    let re = Regex::new(r#"\.([^\s\{\}]+)\s*\{([^}]*)\}"#).unwrap();
    let mut class_rules_map = HashMap::new();

    for cap in re.captures_iter(&css_content) {
        if let (Some(class_name), Some(class_rules)) = (cap.get(1), cap.get(2)) {
            class_rules_map.insert(
                class_name.as_str().to_string(),
                class_rules.as_str().trim().to_string(),
            );
        }
    }

    class_rules_map
}
