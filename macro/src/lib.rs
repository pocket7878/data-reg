use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{braced, parenthesized, parse_macro_input, Result};

#[derive(Debug)]
struct RegexMacroInput {
    tt: proc_macro2::TokenStream,
}

impl RegexMacroInput {
    fn parse_any(input: ParseStream) -> Result<proc_macro2::TokenStream> {
        if input.parse::<syn::Token![.]>().is_ok() {
            Ok(syn::parse_quote!(Regex::any()))
        } else {
            Err(syn::Error::new(input.span(), "expected '.' "))
        }
    }

    // parse {#<ident>} or {#<closure>} syntax to Regex::statisfy
    fn parse_satisfy(input: ParseStream) -> Result<proc_macro2::TokenStream> {
        if input.peek(syn::token::Brace) {
            let braced_content;
            braced!(braced_content in input);
            if let Ok(fn_name) = braced_content.parse::<syn::Ident>() {
                Ok(syn::parse_quote!(Regex::satisfy(#fn_name)))
            } else if let Ok(closure) = braced_content.parse::<syn::ExprClosure>() {
                Ok(syn::parse_quote!(Regex::satisfy(#closure)))
            } else {
                Err(syn::Error::new(
                    input.span(),
                    "expected closure or function name",
                ))
            }
        } else {
            Err(syn::Error::new(input.span(), "expected brace"))
        }
    }

    // parse ?, +, * optional meta characters.
    fn parse_optional_meta_character(
        input: ParseStream,
        base_regex_expr: proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream {
        if input.parse::<syn::token::Question>().is_ok() {
            syn::parse_quote!(Regex::zero_or_one(#base_regex_expr))
        } else if input.parse::<syn::token::Star>().is_ok() {
            syn::parse_quote!(Regex::repeat0(#base_regex_expr))
        } else if input.parse::<syn::Token![+]>().is_ok() {
            syn::parse_quote!(Regex::repeat1(#base_regex_expr))
        } else {
            base_regex_expr
        }
    }

    fn parse_atom(input: ParseStream) -> Result<proc_macro2::TokenStream> {
        if let Ok(any_regex) = Self::parse_any(input) {
            Ok(any_regex)
        } else if let Ok(satisfy_regex) = Self::parse_satisfy(input) {
            Ok(satisfy_regex)
        } else if input.peek(syn::token::Paren) {
            let parend_content;
            parenthesized!(parend_content in input);
            match Self::parse_expr(&parend_content) {
                Ok(expr) => Ok(syn::parse_quote!(Regex::group(#expr))),
                Err(error) => Err(error),
            }
        } else {
            Err(syn::Error::new(
                input.span(),
                "expreted '.' or {#<ident>} or {#<closure>} or (#<regex>)",
            ))
        }
    }

    fn parse_factor(input: ParseStream) -> Result<proc_macro2::TokenStream> {
        match Self::parse_atom(input) {
            Ok(atom) => Ok(Self::parse_optional_meta_character(input, atom)),
            Err(error) => Err(error),
        }
    }

    fn parse_term(input: ParseStream) -> Result<proc_macro2::TokenStream> {
        let mut factors = vec![];

        let factor = Self::parse_factor(input)?;
        factors.push(factor);

        while let Ok(f) = Self::parse_factor(input) {
            factors.push(f);
        }

        if factors.len() == 1 {
            Ok(factors[0].clone())
        } else {
            let mut reg = factors[0].clone();
            for f in factors.iter().skip(1) {
                reg = syn::parse_quote!(Regex::concat(#reg, #f));
            }
            Ok(reg)
        }
    }

    fn parse_expr(input: ParseStream) -> Result<proc_macro2::TokenStream> {
        let term = Self::parse_term(input)?;
        let mut terms = vec![];
        terms.push(term);

        while input.parse::<syn::Token![|]>().is_ok() {
            terms.push(Self::parse_term(input)?);
        }

        let mut reg = terms[0].clone();
        for f in terms.iter().skip(1) {
            reg = syn::parse_quote!(Regex::or(#reg, #f));
        }
        Ok(reg)
    }
}

impl Parse for RegexMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        match Self::parse_expr(input) {
            Ok(tt) => {
                if input.is_empty() {
                    Ok(Self { tt })
                } else {
                    Err(syn::Error::new(input.span(), "unexpected tokens"))
                }
            }
            Err(error) => Err(error),
        }
    }
}

impl From<RegexMacroInput> for proc_macro2::TokenStream {
    fn from(input: RegexMacroInput) -> Self {
        input.tt
    }
}

/// Procedual macro for building vec_reg regex expressions.
///
/// - `{#fn_name}` is a syntax for `Regex::satisfy(fn_name)`.
/// - `{|x| x % 2 == 0}` is a syntax for `Regex::satisfy(|x| x % 2 == 0)`.
/// - `.` is a syntax for `Regex::any()`.
/// - `R|S` is a syntax for `Regex::or(R, S)`.
/// - `RS` is a syntax for `Regex::concat(R, S)`.
/// - `R*` is a syntax for `Regex::repeat0(R)`.
/// - `R+` is a syntax for `Regex::repeat1(R)`.
/// - `R?` is a syntax for `Regex::zero_or_one(R)`.
/// - `(R)` is a syntax for `Regex::group(R)`.
///
#[proc_macro]
pub fn vec_reg(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RegexMacroInput);
    let output: proc_macro2::TokenStream = input.into();

    output.into()
}
