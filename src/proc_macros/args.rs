use {
    syn::{
        parse::{Parse, ParseStream, Result},
        Expr, ExprClosure, LitStr, Token,
    },
};

/// Wrapping of the two arguments given to one of the
/// `regex_is_match`, `regex_find`, or `regex_captures`
/// macros
pub(crate) struct RexValArgs {
    pub regex_str: LitStr,
    pub value: Expr, // this expression is (or produces) the text to search or check
}
impl Parse for RexValArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let regex_str = input.parse::<LitStr>()?;
        input.parse::<Token![,]>()?;
        let value = input.parse::<Expr>()?;
        let _ = input.parse::<Token![,]>(); // allow a trailing comma
        Ok(RexValArgs { regex_str, value })
    }
}

/// Wrapping of the three arguments given to the
/// `regex_replace_all` macro
pub(crate) struct RexValFunArgs {
    pub regex_str: LitStr,
    pub value: Expr,
    pub fun: ExprClosure,
}
impl Parse for RexValFunArgs {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let regex_str = input.parse::<LitStr>()?;
        input.parse::<Token![,]>()?;
        let value = input.parse::<Expr>()?;
        input.parse::<Token![,]>()?;
        let fun = input.parse::<ExprClosure>()?;
        let _ = input.parse::<Token![,]>(); // allow a trailing comma
        Ok(RexValFunArgs {
            regex_str,
            value,
            fun,
        })
    }
}

