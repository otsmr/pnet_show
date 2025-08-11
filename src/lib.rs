extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Show)]
pub fn debug_print_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let name_packet = format_ident!("{}Packet", name); // Create the new name

    let fields = if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            fields.named
        } else {
            panic!("DebugPrint can only be derived for structs with named fields");
        }
    } else {
        panic!("DebugPrint can only be derived for structs");
    };

    let field_names: Vec<_> = fields
        .iter()
        .filter(|f| f.ident.as_ref().is_none_or(|ident| ident != "payload")) // Ignore the "payload" field
        .map(|f| {
            let ident = &f.ident;
            quote! { stringify!(#ident) } // Convert field names to strings
        })
        .collect();

    let field_accessors: Vec<_> = fields
        .iter()
        .filter(|f| f.ident.as_ref().is_none_or(|ident| ident != "payload")) // Ignore the "payload" field
        .map(|f| {
            let ident = &f.ident;
            format_ident!("get_{}", ident.as_ref().unwrap())
        })
        .collect();

    let expanded = quote! {
        impl<'a> #name_packet<'a> {
            fn show(&self) -> String {
                let mut result = String::new();
                result.push_str(&format!("###[ {} ]###\n", stringify!(#name)));
                #(result.push_str(&format!(" {}: {:?}\n", #field_names, &self.#field_accessors()));)*
                result
            }
        }
    };

    TokenStream::from(expanded)
}
