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

            fn node_type(&self)->NodeType {
                self.node_type
            }
            
            fn add_child(&mut self, elem: Box<dyn Any>) {
                self.childs.push(elem);
            }

            fn find_child_by_line(&mut self, line: u32) {
                
            }

            // fn find_child_by_name<T: 'static + NamedNode>(&mut self, name: &str)->Option<&T>{
            //     self.childs.iter().find(|x| {
            //         x.downcast_ref::<T>().map_or(None,|y| {
            //             println!("{}",NamedNode::name(y));
            //             if NamedNode::name(y) == name {
            //                 Some(1)
            //             } else {
            //                 None
            //             }
            //         }).is_some()
            //     }).map_or(None,|z| {
            //         z.downcast_ref::<T>()
            //     })
            // }
        }
    };

    gen.into()
}

#[proc_macro_derive(NamedNode)]
pub fn name_node_derive(input:TokenStream) -> TokenStream{
    let ast:DeriveInput = syn::parse(input).unwrap();
    impl_name_node_derive(&ast)
}

fn impl_name_node_derive(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    
    let gen = quote! {
        impl NamedNode for #name {
            fn node_name(&self)->String {
                self.name.clone()
            }
        }
    };

    gen.into()
}

#[proc_macro_derive(AddressNode)]
pub fn address_node_derive(input:TokenStream) -> TokenStream{
    let ast:DeriveInput = syn::parse(input).unwrap();
    impl_address_node_derive(&ast)
}

fn impl_address_node_derive(ast: &DeriveInput) -> TokenStream {
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
        fields.named.push(get_field_def(quote! { pub node_type:NodeType}));
        fields.named.push(get_field_def(quote! { pub start_line:u32}));
        fields.named.push(get_field_def(quote! { pub end_line:u32}));
        fields.named.push(get_field_def(quote! { pub childs: Vec<Box<dyn Any>>}));
        // we can't note hierichy here.
        // fields.named.push(get_field_def(quote! { pub childs_arena: Vec<NodeId>}));
        // fields.named.push(get_field_def(quote! { pub parent_arena: Vec<NodeId>}));
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
        fields.named.push(get_field_def(quote! { pub name:String }));
    }

    return quote! {
        #item_struct
    }
    .into();

}

#[proc_macro_attribute]
pub fn add_addressnode_field(_args: TokenStream, input:TokenStream) -> TokenStream{
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(_args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(get_field_def(quote! { pub address:u32}));
        fields.named.push(get_field_def(quote! { pub address_ext:i32}));
        fields.named.push(get_field_def(quote! { pub calib_access:Option<CalibrationAccess>}));
        fields.named.push(get_field_def(quote! { pub description:String}));
        fields.named.push(get_field_def(quote! { pub max_refresh_rate:i32}));
        fields.named.push(get_field_def(quote! { pub max_refresh_unit:Option<ScalingUnits>}));
        fields.named.push(get_field_def(quote! { pub model_link:String}));
        fields.named.push(get_field_def(quote! { pub symbol_link:String}));
        fields.named.push(get_field_def(quote! { pub symbol_offset:i32}));
        fields.named.push(get_field_def(quote! { pub ref_instance:Option<NodeId>}));
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