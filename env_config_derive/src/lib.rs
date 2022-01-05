// Based on https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs @ 8a2a0d6
use std::sync::Mutex;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use proc_macro_error::{abort, proc_macro_error};

use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote, AttributeArgs, Data, DeriveInput, Field, Fields, GenericParam, Generics, Ident, Meta, NestedMeta, Lit, PathArguments, Type};

struct Config {
    prefix: Option<String>,
}

impl Config {
    fn set_prefix(&mut self, lit: &Lit) {
        match lit {
            Lit::Str(litstr) => {
                self.prefix = Some(litstr.value());
            },
            _ => abort! { lit, "attribute arg prefix only accepts a literal string value" },
        }
    }
}

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config { prefix: None });
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn env_config(args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);

    for arg in args.iter() {
        match arg {
            NestedMeta::Meta(meta) => {
                match meta {
                    Meta::NameValue(metanamevalue) => {
                        match metanamevalue.path.get_ident() {
                            Some(ident) => {
                                match ident.to_string().as_str() {
                                    "prefix" => CONFIG.lock().unwrap().set_prefix(&metanamevalue.lit),
                                    _ => abort! { ident, "unknown attribute arg, valid args are: prefix" },
                                }
                            },
                            None => abort! { metanamevalue.path.get_ident(), "attribute arg without a name, valid args are: prefix" },
                        }
                    },
                    _ => abort! { meta, "attribute only supports these named args: prefix" },
                }
            },
            _ => abort! { arg, "attribute only supports these named args: prefix" },
        }
    }

    item
}

#[proc_macro_error]
#[proc_macro_derive(EnvConfig)]
pub fn derive_env_config(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let from_env_impl = from_env(&name, &input.data);

    let expanded = quote! {
        impl #impl_generics env_config::FromEnv for #name #ty_generics #where_clause {
            fn from_env() -> Result<#name, env_config::EnvError> {
                #from_env_impl
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(env_config::FromEnvVar));
        }
    }
    generics
}

fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(typepath) if typepath.qself.is_none() => {
            match typepath.path.segments.last() {
                Some(pathsegment) => {
                    pathsegment.ident == "Option" && matches!(pathsegment.arguments, PathArguments::AngleBracketed(_))
                },
                None => false,
            }
        },
        _ => false
    }
}

fn env_var_name(f: &Field) -> String {
    let mut env_var_name = "".to_string();
    match &CONFIG.lock().unwrap().prefix {
        Some(prefix) => {
            env_var_name.push_str(&prefix);
            env_var_name.push_str("_");
        },
        None => (),
    }
    env_var_name.push_str(&f.ident.as_ref().unwrap().to_string().to_uppercase());
    env_var_name
}

fn from_env_var_to_option(f: &Field) -> TokenStream {
    let name = &f.ident;
    let ty = &f.ty;
    let env_var_name = env_var_name(f);
    quote_spanned! {f.span() =>
        let #name: #ty = env_config::FromEnvVar::from_env_var(#env_var_name).map_err(|e| env_config::EnvError::InvalidValue(e))?;
    }
}

fn from_env_var_to_mandatory(f: &Field) -> TokenStream {
    let name = &f.ident;
    let ty = &f.ty;
    let env_var_name = env_var_name(f);
    quote! {
        let #name: #ty = match env_config::FromEnvVar::from_env_var(#env_var_name).map_err(|e| env_config::EnvError::InvalidValue(e))? {
            Some(value) => value,
            None => Err(env_config::EnvError::MandatoryEnvVarNotSet { env_var_name: #env_var_name.into() })?,
        };
    }
}

fn from_env(name: &Ident, data: &Data) -> TokenStream {
    match *data {
        Data::Struct(ref data) => {
            match data.fields {
                Fields::Named(ref fields) => {
                    let lets = fields.named.iter().map(|f| {
                        match is_option(&f.ty) {
                            true => from_env_var_to_option(f),
                            false => from_env_var_to_mandatory(f),
                        }
                    });
                    let struct_field_names = fields.named.iter().map(|f| {
                        &f.ident
                    });
                    quote! {
                        #( #lets )*
                        Ok(#name { #( #struct_field_names ),* })
                    }
                }
                // can't do tuple structs as no way to tell which env var to look at
                Fields::Unnamed(_) => abort! { data.fields, "cannot be derived for tuple structs" },
                // nothing to set on a unit struct
                Fields::Unit => abort! { data.fields, "cannot be derived for unit structs" }
            }
        },
        // can't do enum as no way to tell how to choose or set a value
        Data::Enum(_) => abort! { name, "cannot be derived for enums" },
        // union might be possible but it would be kinda strange to use them, leave for now
        Data::Union(_) => abort! { name, "cannot be derived for unions" },
    }
}
