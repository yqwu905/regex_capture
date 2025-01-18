//! This macro implement `FromStr` trait for your struct with given regex expression.
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DataStruct, DeriveInput, Fields, Ident, LitStr};

#[derive(Debug)]
enum StructField {
    Raw(String),
    Func(String, String),
}

#[proc_macro_derive(RegexCapture, attributes(converter))]
pub fn my_derive(_input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(_input as DeriveInput);

    // parse regex attribute #[converter(regex = "")]
    let mut regex = String::new();
    for attr in input.attrs {
        if attr.path().is_ident("converter") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("regex") {
                    let value = meta.value()?;
                    let regex_lit: LitStr = value.parse()?;
                    regex = regex_lit.value();
                    return Ok(());
                };
                Err(meta.error("No regex found"))
            })
            .unwrap();
        }
    }

    if let Data::Struct(DataStruct {
        fields: Fields::Named(ref fields),
        ..
    }) = input.data
    {
        let mut fields_token: Vec<TokenStream2> = Vec::new();
        for f in fields.named.iter() {
            if let Some(field_name) = f.ident.clone() {
                let mut field = StructField::Raw(field_name.to_string());
                for attr in f.attrs.clone() {
                    if attr.path().is_ident("converter") {
                        let _ = attr.parse_nested_meta(|meta| {
                            if meta.path.is_ident("func") {
                                let value = meta.value()?;
                                let s: Ident = value.parse()?;
                                field = StructField::Func(field_name.to_string(), s.to_string());
                                Ok(())
                            } else {
                                Err(meta.error("Unsupported attribute"))
                            }
                        });
                    }
                }
                let reg_cap_get = format!(
                    "reg_cap.name(\"{}\").ok_or(\"Failed to matched regex group {}\")?.as_str()",
                    field_name, field_name
                );
                let field_token: TokenStream2 = match field {
                    StructField::Raw(ref field) => format!("{}: {}.parse()?,", field, reg_cap_get)
                        .parse()
                        .unwrap(),
                    StructField::Func(ref field, ref func) => {
                        format!("{}: {}({})?,", field, func, reg_cap_get)
                            .parse()
                            .unwrap()
                    }
                };
                fields_token.push(field_token);
            }
        }

        let name = &input.ident;
        let rex_name = Ident::new(format!("_{}_REX", name).as_str(), Span::call_site());
        let tokens = quote! {

            static #rex_name : LazyLock<Regex> = LazyLock::new(|| Regex::new(#regex).unwrap());
            impl FromStr for #name {
                type Err = Box<dyn Error>;

                fn from_str(src: &str) -> Result<Self, Self::Err> {
                    let reg_cap = #rex_name.captures(src).ok_or("#name failed to match")?;
                    Ok(#name {
                        #(#fields_token)*
                    })
                }
            }

        };

        tokens.into()
    } else {
        panic!("RegexCapture macro can only be use with struct!");
    }
}
