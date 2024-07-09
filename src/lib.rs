extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Ident, Token};

struct SafeFormatInput {
    format_string: Expr,
    args: Vec<(Ident, Expr)>,
}

impl Parse for SafeFormatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let format_string: Expr = input.parse()?;
        input.parse::<Token![,]>()?;

        let mut args = Vec::new();
        while !input.is_empty() {
            let name: Ident = input.parse()?;
            input.parse::<Token![=]>()?;
            let expr: Expr = input.parse()?;
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

    // Collect the argument names and their corresponding expressions
    let mut format_args = Vec::new();
    let mut replace_statements = Vec::new();

    for (name, expr) in &args {
        let placeholder = format!("{{{}}}", name);
        replace_statements.push(quote! {
            format_tokens = format_tokens.replace(#placeholder, "{}");
        });
        format_args.push(expr);
    }

    let output = quote! {
        {
            let mut format_tokens = #format_string.to_string();
            #(#replace_statements)*

            let mut formatted_string = String::new();
            let mut split_iter = format_tokens.split("{}");
            if let Some(first) = split_iter.next() {
                formatted_string.push_str(first);
            }
            let mut arg_iter = vec![#(#format_args),*].into_iter();
            while let Some(part) = split_iter.next() {
                if let Some(arg) = arg_iter.next() {
                    formatted_string.push_str(&arg.to_string());
                }
                formatted_string.push_str(part);
            }
            formatted_string
        }
    };

    TokenStream::from(output)
}
