use darling::{util::PathList, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{parse_macro_input, AttributeArgs, Fields, FieldsNamed, Ident, ItemStruct};

fn ident(s: String) -> Ident {
    ident_str(s.as_str())
}

fn ident_str(s: &str) -> Ident {
    Ident::new(s, proc_macro2::Span::call_site())
}

// macro for generating getters
#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    // getting attribute fields as Vec<String>
    let a_fields = match PathList::from_list(&parse_macro_input!(attr as AttributeArgs)) {
        Ok(v) => v.to_strings(),
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    // cloning TokenStream for further return
    let mut ts = item.clone();
    // Getting the syn AST
    let item_struct = parse_macro_input!(item as ItemStruct);

    // Getting name of struct
    let name = &item_struct.ident;

    // Getting type of fields
    let tymap = {
        match item_struct.fields {
            Fields::Named(FieldsNamed {
                brace_token: _,
                ref named,
            }) => {
                let mut tymap = HashMap::new();

                for f in named.iter() {
                    if let Some(ref ident) = f.ident {
                        tymap.insert(ident.to_string(), &f.ty);
                    }
                }

                tymap
            }
            _ => {
                return quote! {
                    compile_error!("The fields should be named");
                }
                .into()
            }
        }
    };

    // adding impls for getters
    for field in a_fields.iter() {
        // name of the field and ref getter
        let fieldname = ident_str(field.as_str());
        // get_mut name
        let fnname_mut = ident(format!("{}_mut", field));
        // typename
        let ty = tymap.get(field);
        // code gen
        let res: TokenStream = quote! {
            impl #name {
                pub fn #fieldname(&self) -> &#ty {
                    &self.#fieldname
                }

                pub fn #fnname_mut(&mut self) -> &mut #ty {
                    &mut self.#fieldname
                }
            }
        }
        .into();
        ts.extend(res.into_iter());
    }

    ts
}

// macro for generating setters
#[proc_macro_attribute]
pub fn set(attr: TokenStream, item: TokenStream) -> TokenStream {
    // getting attribute fields as Vec<String>
    let a_fields = match PathList::from_list(&parse_macro_input!(attr as AttributeArgs)) {
        Ok(v) => v.to_strings(),
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    // cloning TokenStream for further return
    let mut ts = item.clone();
    // Getting the syn AST
    let item_struct = parse_macro_input!(item as ItemStruct);

    // Getting name of struct
    let name = &item_struct.ident;

    // Getting type of fields
    let tymap = {
        match item_struct.fields {
            Fields::Named(FieldsNamed {
                brace_token: _,
                ref named,
            }) => {
                let mut tymap = HashMap::new();

                for f in named.iter() {
                    if let Some(ref ident) = f.ident {
                        tymap.insert(ident.to_string(), &f.ty);
                    }
                }

                tymap
            }
            _ => {
                return quote! {
                    compile_error!("The fields should be named");
                }
                .into()
            }
        }
    };

    // adding impls for setters
    for field in a_fields.iter() {
        // name of the field
        let fieldname = ident_str(field.as_str());
        // set name
        let fnname = ident(format!("set_{}", field));
        // typename
        let ty = tymap.get(field);
        // code gen
        let res: TokenStream = quote! {
            impl #name {
                pub fn #fnname(&mut self, #fieldname: #ty) {
                    self.#fieldname = #fieldname;
                }
            }
        }
        .into();
        ts.extend(res.into_iter());
    }

    ts
}
