use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream};
use syn::{braced, bracketed, parenthesized, parse_macro_input, Result};

#[derive(Debug)]
struct RegexMacroInput {
    tt: proc_macro2::TokenStream,
}

impl RegexMacroInput {
    fn parse_any(input: ParseStream) -> Result<proc_macro2::TokenStream> {
        if let Err(err) = input.parse::<syn::Token![.]>() {
            return Err(err);
        }
        Ok(syn::parse_quote!(vec_reg_common::Regex::any()))
    }

    // parse [#<ident>] or [#<closure>] syntax to Regex::statisfy
    fn parse_satisfy(input: ParseStream) -> Result<proc_macro2::TokenStream> {
        if !input.peek(syn::token::Bracket) {
            return Err(syn::Error::new(input.span(), "expected brace"));
        }

        let braced_content;
        bracketed!(braced_content in input);
        let inverse = braced_content.parse::<syn::Token![^]>().is_ok();
        if let Ok(fn_name) = braced_content.parse::<syn::Ident>() {
            if inverse {
                Ok(syn::parse_quote!(vec_reg_common::Regex::not_satisfy(#fn_name)))
            } else {
                Ok(syn::parse_quote!(vec_reg_common::Regex::satisfy(#fn_name)))
            }
        } else if let Ok(closure) = braced_content.parse::<syn::ExprClosure>() {
            if inverse {
                Ok(syn::parse_quote!(vec_reg_common::Regex::not_satisfy(#closure)))
            } else {
                Ok(syn::parse_quote!(vec_reg_common::Regex::satisfy(#closure)))
            }
        } else {
            Err(syn::Error::new(
                input.span(),
                "expected closure or function name",
            ))
        }
    }

    // parse ?, +, *, {n}, {n,}, {n,m} meta characters.
    fn parse_optional_meta_character(
        input: ParseStream,
        base_regex_expr: proc_macro2::TokenStream,
    ) -> Result<proc_macro2::TokenStream> {
        if input.parse::<syn::token::Question>().is_ok() {
            let greedy = input.parse::<syn::token::Question>().is_err();
            Ok(syn::parse_quote!(vec_reg_common::Regex::zero_or_one(#base_regex_expr, #greedy)))
        } else if input.parse::<syn::token::Star>().is_ok() {
            let greedy = input.parse::<syn::token::Question>().is_err();
            Ok(syn::parse_quote!(vec_reg_common::Regex::repeat0(#base_regex_expr, #greedy)))
        } else if input.parse::<syn::Token![+]>().is_ok() {
            let greedy = input.parse::<syn::token::Question>().is_err();
            Ok(syn::parse_quote!(vec_reg_common::Regex::repeat1(#base_regex_expr, #greedy)))
        } else if input.peek(syn::token::Brace) {
            let braced_content;
            braced!(braced_content in input);
            let greedy = input.parse::<syn::token::Question>().is_err();
            if let Ok(n_lit) = braced_content.parse::<syn::LitInt>() {
                if braced_content.parse::<syn::token::Comma>().is_err() {
                    Ok(syn::parse_quote!(vec_reg_common::Regex::repeat_n(#base_regex_expr, #n_lit)))
                } else if let Ok(m_lit) = braced_content.parse::<syn::LitInt>() {
                    Ok(
                        syn::parse_quote!(vec_reg_common::Regex::repeat_min_max(#base_regex_expr, #n_lit, #m_lit, #greedy)),
                    )
                } else if braced_content.is_empty() {
                    Ok(
                        syn::parse_quote!(vec_reg_common::Regex::repeat_n_or_more(#base_regex_expr, #n_lit, #greedy)),
                    )
                } else {
                    Err(syn::Error::new(
                        braced_content.span(),
                        "expected integer literal or empty",
                    ))
                }
            } else {
                Err(syn::Error::new(input.span(), "expected integer literal"))
            }
        } else {
            Ok(base_regex_expr)
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
            let mut capturing = true;
            let mut name = None;
            if parend_content.parse::<syn::token::Question>().is_ok() {
                if parend_content.parse::<syn::token::Colon>().is_ok() {
                    capturing = false;
                } else if parend_content.peek(syn::Ident) {
                    let parsed_ident = parend_content.parse::<syn::Ident>()?;
                    if parsed_ident.to_string().as_str() != "P" {
                        return Err(syn::Error::new(parsed_ident.span(), "expected 'P'"));
                    }
                    if let Err(err) = parend_content.parse::<syn::Token![<]>() {
                        return Err(err);
                    }
                    match parend_content.parse::<syn::LitStr>() {
                        Ok(group_name) => {
                            name = Some(group_name);
                        }
                        Err(err) => return Err(err),
                    }
                    if let Err(err) = parend_content.parse::<syn::Token![>]>() {
                        return Err(err);
                    }
                }
            }
            match Self::parse_expr(&parend_content) {
                Ok(expr) => {
                    if capturing && name.is_none() {
                        Ok(syn::parse_quote!(vec_reg_common::Regex::group(#expr)))
                    } else if capturing && name.is_some() {
                        let name = name.unwrap();
                        Ok(syn::parse_quote!(vec_reg_common::Regex::named_group(#name, #expr)))
                    } else {
                        Ok(syn::parse_quote!(vec_reg_common::Regex::non_capturing_group(#expr)))
                    }
                }
                Err(error) => Err(error),
            }
        } else {
            Err(syn::Error::new(
                input.span(),
                "expected '.', {#<ident>}, {#<closure>}, (#<regex>), (?:#<regex>) or (?P<\"name\">#<regex>)",
            ))
        }
    }

    fn parse_factor(input: ParseStream) -> Result<proc_macro2::TokenStream> {
        if input.parse::<syn::token::Caret>().is_ok() {
            Ok(syn::parse_quote!(vec_reg_common::Regex::begin()))
        } else if input.parse::<syn::token::Dollar>().is_ok() {
            Ok(syn::parse_quote!(vec_reg_common::Regex::end()))
        } else {
            match Self::parse_atom(input) {
                Ok(atom) => Self::parse_optional_meta_character(input, atom),
                Err(error) => Err(error),
            }
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
                reg = syn::parse_quote!(vec_reg_common::Regex::concat(#reg, #f));
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
            reg = syn::parse_quote!(vec_reg_common::Regex::or(#reg, #f));
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

#[proc_macro]
pub fn vec_reg(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as RegexMacroInput);
    let output: proc_macro2::TokenStream = input.into();

    output.into()
}
