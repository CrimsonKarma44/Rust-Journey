extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Item};
use syn::{parse_macro_input, ItemFn, ItemStruct};
// custom derive macro
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree+
    
    // that we can manipulate
    // let ast = syn::parse(input).unwrap();
    let ast = parse_macro_input!(input as ItemStruct);
    
    // Build the trait implementation
    impl_hello_macro(&ast)
}


// fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
fn impl_hello_macro(ast: &ItemStruct) -> TokenStream {
    let name = &ast.ident;
    let expanded = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };
    
    expanded.into()
}

// attribute like macro

#[proc_macro_attribute]
pub fn my_logger(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. Parse the item (the function this is attached to)
    let input_fn = parse_macro_input!(item as ItemFn);
    let name = &input_fn.sig.ident;
    let block = &input_fn.block;
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;

    // 2. Generate new code (we are "wrapping" the original function)
    let expanded = quote! {
        #vis #sig {
            println!("Entering function: {}", stringify!(#name));
            let result = (|| #block)(); // Execute the original body
            println!("Exiting function: {}", stringify!(#name));
            result
        }
    };

    // 3. Return the new code as a TokenStream
    TokenStream::from(expanded)
}


// Function like macro
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // 1. Convert the tokens to a string to see what the user wrote
    // Note: In a real macro like 'sqlx', you would parse this 
    // more carefully to check for syntax errors at compile-time.
    let sql_query = input.to_string();

    // 2. Do something based on the query
    // Let's imagine we generate a mock database call
    let expanded = quote! {
        {
            println!("Executing SQL: {}", #sql_query);
            // In a real app, this might return a Vec of data
            format!("Result from: {}", #sql_query)
        }
    };

    expanded.into()
}