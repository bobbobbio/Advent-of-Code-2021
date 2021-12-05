extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use std::matches;
use syn::*;

fn verify_signature(sig: &Signature) -> Result<()> {
    let as_expected = matches!(sig, Signature {
        constness: None,
        asyncness: None,
        unsafety: None,
        abi: None,
        generics: Generics {
            lt_token: None,
            gt_token: None,
            where_clause: None,
            ..
        },
        inputs,
        variadic: None,
        output: ReturnType::Type(_, ret_type),
        ..
    } if inputs.is_empty() && matches!(&**ret_type, Type::Infer(_)));

    if !as_expected {
        Err(Error::new(
            sig.ident.span(),
            "function signature wrong for into_parser",
        ))
    } else {
        Ok(())
    }
}

fn into_parser_inner(input: TokenStream) -> Result<ItemFn> {
    let input: ItemFn = parse(input)?;
    verify_signature(&input.sig)?;
    let vis = &input.vis;

    let name = input.sig.ident;
    let block = input.block;
    Ok(parse_quote! {
        #vis fn #name<Input>() -> impl ::combine::Parser<Input, Output = Self>
        where
            Input: ::combine::Stream<Token = char>,
        #block
    })
}

#[proc_macro_attribute]
pub fn into_parser(_attr: TokenStream, input: TokenStream) -> TokenStream {
    match into_parser_inner(input) {
        Ok(v) => quote!(#v).into(),
        Err(e) => e.into_compile_error().into(),
    }
}
