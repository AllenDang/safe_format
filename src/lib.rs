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

/// A procedural macro that formats strings using named parameters.
///
/// # Overview
///
/// The `safe_format` macro allows you to create formatted strings using named parameters,
/// similar to the standard `format!` macro. It safely ignores any extra parameters that
/// are not used in the format string, providing a flexible and convenient way to handle
/// string formatting in Rust.
///
/// # Returns
///
/// Returns a `String` containing the formatted output with the named parameters
/// substituted into the format string.
///
/// # Errors
///
/// This macro does not produce any runtime errors if extra parameters are provided
/// that are not used in the format string. These extra parameters are simply ignored.
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
