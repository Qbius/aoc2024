extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr, ReturnType, Type, PathArguments, GenericArgument};

#[proc_macro_attribute]
pub fn lines(_: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);

    let fn_name = &function.sig.ident;
    let details_name = syn::Ident::new(&format!("{}_details", fn_name.to_string()), function.sig.ident.span());

    let visibility = &function.vis;
    let inputs = &function.sig.inputs;
    let return_type = &function.sig.output;
    let block = &function.block;

    let output = quote! {
        #visibility fn #fn_name(input: &str) #return_type {
            let lines: Vec<String> = input.split('\n').map(|line| line.trim().to_string()).collect();
            #details_name(lines)
        }

        fn #details_name(#inputs) #return_type #block
    };

    output.into()
}

#[proc_macro_attribute]
pub fn grid(_: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);

    let fn_name = &function.sig.ident;
    let details_name = syn::Ident::new(&format!("{}_details", fn_name.to_string()), function.sig.ident.span());

    let visibility = &function.vis;
    let inputs = &function.sig.inputs;
    let return_type = &function.sig.output;
    let block = &function.block;

    let output = quote! {
        #visibility fn #fn_name(input: &str) #return_type {
            let grid: std::collections::HashMap<(usize, usize), char> = input.split('\n').enumerate().flat_map(move |(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c))).collect();
            #details_name(grid)
        }

        fn #details_name(#inputs) #return_type #block
    };

    output.into()
}

fn extract_type(function: ItemFn) -> Type {
    if let ReturnType::Type(_, return_type) = &function.sig.output {
        if let Type::Path(type_path) = return_type.as_ref() {
            if let Some(segment) = type_path.path.segments.last() {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(arg) = args.args.first() {
                        if let GenericArgument::Type(typ) = arg {
                            return typ.clone();
                        }
                    }
                }
            }
        }
    }
    
    panic!("Function must return Option<T>.");
}

#[proc_macro_attribute]
pub fn unwrap(_: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);

    let fn_name = &function.sig.ident;
    let details_name = syn::Ident::new(&format!("{}_details", fn_name.to_string()), function.sig.ident.span());

    let visibility = &function.vis;
    let inputs = &function.sig.inputs;
    let block = &function.block;

    let return_type = extract_type(function.clone());
    let output = quote! {
        #visibility fn #fn_name(#inputs) -> #return_type {
            #details_name(#(#inputs)).expect(format!("Something went wrong in \"{}\"", #fn_name.to_string()))
        }
        
        fn #details_name(#inputs) -> Option<#return_type> #block
    };

    output.into()
}

#[proc_macro_attribute]
pub fn regex(attr: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);
    let re_pattern = parse_macro_input!(attr as LitStr);

    let fn_name = &function.sig.ident;
    let details_name = syn::Ident::new(&format!("{}_details", fn_name.to_string()), function.sig.ident.span());

    let visibility = &function.vis;
    let inputs = &function.sig.inputs;
    let return_type = &function.sig.output;
    let block = &function.block;

    let output = quote! {
        #visibility fn #fn_name(input: &str) #return_type {
            let lines: Vec<String> = input.split('\n').map(String::from).collect();
            let re = regex::Regex::new(#re_pattern).expect(format!("{} is an invalid regex pattern!", #re_pattern));
            #details_name(lines)
        }

        fn #details_name(#inputs) #return_type #block
    };

    output.into()
}