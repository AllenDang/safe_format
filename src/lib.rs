extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, ExprLit, Lit, Token};

struct SafeFormatInput {
    format_string: Expr,
    args: Vec<(proc_macro2::Ident, syn::Expr)>,
}

impl Parse for SafeFormatInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let format_string: Expr = input.parse()?;
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

    let mut format_string_literal = None;
    let mut format_expr = None;

    // Check if format_string is a string literal or an expression
    if let Expr::Lit(ExprLit {
        lit: Lit::Str(lit_str),
        ..
    }) = &format_string
    {
        format_string_literal = Some(lit_str.value());
    } else {
        format_expr = Some(&format_string);
    }

    let mut format_tokens = String::new();
    let mut format_args = Vec::new();

    if let Some(literal) = format_string_literal {
        format_tokens = literal;

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
    } else if let Some(expr) = format_expr {
        // Handle the case where the format string is an expression
        for (name, expr) in &args {
            let placeholder = format!("{{{}}}", name);
            format_args.push(quote! {
                if let Some(pos) = format_tokens.find(#placeholder) {
                    format_tokens.replace_range(pos..pos + #placeholder.len(), "{}");
                }
            });
            format_args.push(quote! { #expr });
        }

        let output = quote! {
            {
                let mut format_tokens = #expr.to_string();
                #(#format_args)*
                format!(format_tokens, #(#format_args),*)
            }
        };

        TokenStream::from(output)
    } else {
        TokenStream::from(quote! {
            compile_error!("First argument must be a string literal or a variable of type String.");
        })
    }
}
