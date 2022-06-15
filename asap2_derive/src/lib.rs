use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput, parse_macro_input,ItemStruct, parse::{self, Parser}, Field};
use syn::__private::TokenStream2;

/// Auto generate field and impls that `Node` trait needs
#[proc_macro_derive(Node)]
pub fn node_derive(input:TokenStream) -> TokenStream{
    let ast:DeriveInput = syn::parse(input).unwrap();

    impl_node_derive(&ast)
}

fn impl_node_derive(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    
    let gen = quote! {
        impl Node for #name {
            fn start_line(&self)->u32 {
                self.start_line
            }
        
            fn end_line(&self)->u32 {
                self.end_line
            }
        
            fn node_type(&self)->NodeType {
                self.node_type
            }
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn add_node_field(_args: TokenStream, input:TokenStream) -> TokenStream{
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(_args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(get_field_def(quote! { node_type:NodeType}));
        fields.named.push(get_field_def(quote! { start_line:u32}));
        fields.named.push(get_field_def(quote! { end_line:u32}));
    }

    return quote! {
        #item_struct
    }
    .into();

}

#[proc_macro_attribute]
pub fn add_namenode_field(_args: TokenStream, input:TokenStream) -> TokenStream{
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(_args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(get_field_def(quote! { name:String }));
    }

    return quote! {
        #item_struct
    }
    .into();

}

fn get_field_def(tok:TokenStream2)-> Field{
    syn::Field::parse_named
    .parse2(tok)
    .unwrap()
}