//! Derive macro for the `Topicable` trait.

// TODO support tuple-structs

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Expr, ExprLit, Fields, Lit, Meta, MetaNameValue, parse_macro_input};

/// Derives `Topicable` for a named-field struct.
///
/// Fields annotated with `#[key]` are collected into a generated `<Name>Key`
/// struct that implements `CdrBounds`. Structs with no `#[key]` fields use
/// [`()`](primitive@unit) as their key type and must implement [`Default`].
///
/// An optional `#[dds_type_name = "..."]` attribute overrides the DDS type
/// name used for topic matching. Without it, the Rust type name is used.
///
/// # Examples
///
/// ```ignore
/// #[derive(cyclonedds::Topicable, serde::Serialize, serde::Deserialize, Default, Clone, Debug)]
/// #[dds_type_name = "MySensor"]
/// pub struct Sensor {
///     #[key]
///     pub id: u32,
///     pub value: f32,
/// }
///
/// // NOTE: this results in an additional type being generated with the name
/// // `SensorKey { pub id: u32 }`
/// ```
///
/// # Panics
///
/// Panics at compile time if applied to an enum, a union, a tuple struct, or
/// if `#[dds_type_name]` is not a string literal.
#[proc_macro_derive(Topicable, attributes(key, dds_type_name))]
pub fn derive_topicable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident.clone();
    let vis = input.vis.clone();

    // Extract custom topic name if present
    let mut dds_type_name_literal: Option<String> = None;
    for attr in &input.attrs {
        if attr.path().is_ident("dds_type_name") {
            if let Meta::NameValue(MetaNameValue {
                value:
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(lit_str),
                        ..
                    }),
                ..
            }) = &attr.meta
            {
                dds_type_name_literal = Some(lit_str.value());
            } else {
                panic!("#[dds_type_name = \"...\"] must be a string literal");
            }
        }
    }

    let data_struct = match &input.data {
        Data::Struct(s) => s,
        _ => panic!("#[derive(Topicable)] only works on structs"),
    };

    let mut key_fields = Vec::new();

    if let Fields::Named(fields_named) = &data_struct.fields {
        for field in fields_named.named.iter() {
            let field_name = field.ident.clone().unwrap();
            let field_ty = &field.ty;

            if field.attrs.iter().any(|a| a.path().is_ident("key")) {
                key_fields.push((field_name, field_ty));
            }
        }
    } else {
        panic!("#[derive(Topicable)] only supports named fields");
    }

    // Generate code
    let expanded = if key_fields.is_empty() {
        if let Some(dds_type_name) = dds_type_name_literal {
            quote! {
                impl ::cyclonedds::Topicable for #name {
                    type Key = ();

                    fn from_key(_: &Self::Key) -> Self {
                        Self::default()
                    }

                    fn as_key(&self) -> Self::Key {
                        ()
                    }

                    fn dds_type_name() -> impl AsRef<str> {
                       #dds_type_name
                    }
                }
            }
        } else {
            quote! {
                impl ::cyclonedds::Topicable for #name {
                    type Key = ();

                    fn from_key(_: &Self::Key) -> Self {
                        Self::default()
                    }

                    fn as_key(&self) -> Self::Key {
                        ()
                    }
                }
            }
        }
    } else {
        let key_name = format_ident!("{}Key", name);
        let key_field_defs = key_fields.iter().map(|(n, t)| quote! { pub #n: #t });
        let key_field_inits = key_fields
            .iter()
            .map(|(n, _)| quote! { #n: self.#n.clone() });
        let key_field_from_key = key_fields
            .iter()
            .map(|(n, _)| quote! { #n: key.#n.clone() });
        let key_size_sum = key_fields.iter().map(|(_, t)| {
            quote! {
                <#t as ::cyclonedds::cdr_bounds::CdrBounds>::max_serialized_cdr_size()
            }
        });
        let key_alignment_max = key_fields.iter().map(|(_, t)| {
            quote! {
                <#t as ::cyclonedds::cdr_bounds::CdrBounds>::alignment()
            }
        });

        if let Some(dds_type_name) = dds_type_name_literal {
            quote! {
                #[derive(Default, serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Hash)]
                #vis struct #key_name {
                    #(#key_field_defs),*
                }

                impl ::cyclonedds::cdr_bounds::CdrBounds for #key_name {
                    fn max_serialized_cdr_size() -> ::cyclonedds::cdr_bounds::CdrSize {
                        #(#key_size_sum)+*
                    }

                    fn alignment() -> usize {
                        0 #(.max(#key_alignment_max))*
                    }
                }

                impl ::cyclonedds::Topicable for #name {
                    type Key = #key_name;

                    fn from_key(key: &Self::Key) -> Self {
                        Self {
                            #(#key_field_from_key),*,
                            ..Default::default()
                        }
                    }

                    fn as_key(&self) -> Self::Key {
                        Self::Key {
                            #(#key_field_inits),*
                        }
                    }

                    fn dds_type_name() -> impl AsRef<str> {
                       #dds_type_name
                    }
                }
            }
        } else {
            quote! {
                #[derive(Default, serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Hash)]
                #vis struct #key_name {
                    #(#key_field_defs),*
                }

                impl ::cyclonedds::cdr_bounds::CdrBounds for #key_name {
                    fn max_serialized_cdr_size() -> ::cyclonedds::cdr_bounds::CdrSize {
                        #(#key_size_sum)+*
                    }

                    fn alignment() -> usize {
                        0 #(.max(#key_alignment_max))*
                    }
                }

                impl ::cyclonedds::Topicable for #name {
                    type Key = #key_name;

                    fn from_key(key: &Self::Key) -> Self {
                        Self {
                            #(#key_field_from_key),*,
                            ..Default::default()
                        }
                    }

                    fn as_key(&self) -> Self::Key {
                        Self::Key {
                            #(#key_field_inits),*
                        }
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}
