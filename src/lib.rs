extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, LitStr, Token};

struct SafeFormatInput {
    format_string: LitStr,
    args: Vec<(proc_macro2::Ident, syn::Expr)>,
}

impl Parse for SafeFormatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let format_string: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;

        let mut args = Vec::new();
        while !input.is_empty() {
            let name: proc_macro2::Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let expr: syn::Expr = input.parse()?;
            args.push((name, expr));

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(SafeFormatInput {
            format_string,
            args,
        })
    }
}

#[proc_macro]
pub fn safe_format(input: TokenStream) -> TokenStream {
    let SafeFormatInput {
        format_string,
        args,
    } = parse_macro_input!(input as SafeFormatInput);

    let mut format_tokens = format_string.value();
    let mut format_args = Vec::new();

    for (name, expr) in args {
        let placeholder = format!("{{{}}}", name);
        if format_tokens.contains(&placeholder) {
            format_tokens = format_tokens.replace(&placeholder, "{}");
            format_args.push(quote! { #expr });
        }
    }

    let output = quote! {
        format!(#format_tokens, #(#format_args),*)
    };

    TokenStream::from(output)
}
