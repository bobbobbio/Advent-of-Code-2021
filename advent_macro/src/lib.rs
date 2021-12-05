extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use std::matches;
use syn::spanned::Spanned as _;
use syn::*;

fn part_inner(input: TokenStream, part_number: usize) -> Result<File> {
    let func: ItemFn = parse(input)?;

    let func_name = func.sig.ident.clone();
    let args = func.sig.inputs.clone();

    if args.len() != 1 {
        return Err(Error::new(args.span(), "too many inputs"));
    }
    let parsed_type = if let FnArg::Typed(pat_type) = args.first().unwrap() {
        (&*pat_type.ty).clone()
    } else {
        return Err(Error::new(args.span(), "invalid input type"));
    };

    fn type_is_named(p: &TypePath, name: &str) -> bool {
        let segment = p.path.segments.last();
        if let Some(segment) = segment {
            segment.ident.to_string() == name
        } else {
            false
        }
    }

    let is_vec = |p| type_is_named(p, "Vec");
    let is_string = |p| type_is_named(p, "String");

    let parse_func: TypePath = if matches!(&parsed_type, Type::Path(p) if is_vec(p)) {
        parse_quote!(::advent::parse::parse_lines)
    } else if matches!(&parsed_type, Type::Path(p) if is_string(p)) {
        parse_quote!(::std::convert::TryFrom::try_from)
    } else {
        parse_quote!(::std::str::FromStr::from_str)
    };

    let tramp = Ident::new(&format!("_run_part_{}", part_number), Span::call_site());

    Ok(parse_quote! {
        #func
        fn #tramp(input: &str) -> ::advent::parse::Result<()> {
            let p: #parsed_type = #parse_func(input)?;
            let result = #func_name(p);
            println!("Part {}: {}", #part_number, result);
            Ok(())
        }
    })
}

#[proc_macro_attribute]
pub fn part_one(_attr: TokenStream, input: TokenStream) -> TokenStream {
    match part_inner(input, 1) {
        Ok(v) => quote!(#v).into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn part_two(_attr: TokenStream, input: TokenStream) -> TokenStream {
    match part_inner(input, 2) {
        Ok(v) => quote!(#v).into(),
        Err(e) => e.into_compile_error().into(),
    }
}

#[proc_macro]
pub fn harness(_attr: TokenStream) -> TokenStream {
    quote! {
        fn main() -> ::advent::parse::Result<()> {
            use ::std::io::Read as _;
            let mut input = ::std::string::String::new();
            ::std::io::stdin().lock().read_to_string(&mut input)?;

            _run_part_1(&input)?;
            _run_part_2(&input)?;

            Ok(())
        }
    }
    .into()
}
