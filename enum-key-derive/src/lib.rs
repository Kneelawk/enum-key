use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput};

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(keyable_enum))]
struct Opts {
    key_enum: Option<Ident>,
}

/// Derive macro for deriving the `KeyableEnum` trait on enums.
#[proc_macro_derive(KeyableEnum, attributes(keyable_enum))]
pub fn derive_keyable_enum(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input);

    let opts = Opts::from_derive_input(&derive_input).expect("Invalid keyable_enum options");

    let data_enum = match derive_input.data {
        Data::Enum(e) => e,
        _ => {
            panic!("KeyableEnum can only be implemented on enums.");
        }
    };

    let enum_ident = derive_input.ident;
    let key_enum_ident = opts
        .key_enum
        .as_ref()
        .map(|key_enum| format_ident!("{}", key_enum))
        .unwrap_or_else(|| format_ident!("{}Key", &enum_ident));

    let variant_cases: Vec<_> = data_enum
        .variants
        .iter()
        .map(|var| {
            let var_name = &var.ident;
            let fields = &var.fields;
            let field_holder = if fields.is_empty() {
                quote! {}
            } else {
                let first_field = fields.iter().next().unwrap();
                if first_field.ident.is_some() {
                    quote! {{ .. }}
                } else {
                    quote! {( .. )}
                }
            };

            quote! {Self::#var_name #field_holder => Self::Key::#var_name}
        })
        .collect();

    let variants: Vec<_> = data_enum
        .variants
        .iter()
        .map(|var| var.ident.clone())
        .collect();

    let out = quote! {
        impl ::enum_key::KeyableEnum for #enum_ident {
            type Key = #key_enum_ident;

            fn get_enum_key(&self) -> Self::Key {
                match self {
                    #(#variant_cases),*
                }
            }
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum #key_enum_ident {
            #(#variants),*
        }

        impl ::enum_key::EnumKey for #key_enum_ident {
            type Enum = #enum_ident;

            const VALUES: &'static [Self] = &[#(Self::#variants),*];
        }
    };

    out.into()
}
