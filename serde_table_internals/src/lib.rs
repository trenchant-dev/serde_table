use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    Expr, Ident, Result, Token,
};

// For literal data tables (quoted strings only)
struct ExprMacroInput {
    rows: Vec<Vec<Expr>>,
}

// For tables with expressions (supports unquoted identifiers)
struct UnquotedMacroInput {
    rows: Vec<Vec<Expr>>,
}

struct ParsedRows {
    rows: Vec<Vec<Expr>>,
}

fn parse_rows(
    input: ParseStream,
    parse_expr: impl Fn(ParseStream) -> Result<Expr>,
) -> Result<ParsedRows> {
    let mut rows = Vec::new();
    let mut current_row = Vec::new();
    let mut current_line = input.span().start().line;

    while !input.is_empty() {
        let next_span = input.span().start();

        // Check if we're on a new line
        if next_span.line > current_line && !current_row.is_empty() {
            rows.push(current_row);
            current_row = Vec::new();
        }
        current_line = next_span.line;

        // Parse the next expression using provided parser
        let expr = parse_expr(input)?;
        current_row.push(expr);

        // Skip any whitespace
        while input.peek(Token![_]) {
            let _ = input.parse::<Token![_]>()?;
        }
    }

    if !current_row.is_empty() {
        rows.push(current_row);
    }

    Ok(ParsedRows { rows })
}

impl Parse for ExprMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed = parse_rows(input, |input| input.parse::<Expr>())?;
        Ok(ExprMacroInput { rows: parsed.rows })
    }
}

impl Parse for UnquotedMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed = parse_rows(input, |input| {
            if input.peek(Ident)
                && !input.peek2(syn::token::Paren)
                && !input.peek2(syn::token::Bracket)
                && !input.peek2(syn::token::Brace)
                && !input.peek2(syn::Token![.])
                && !input.peek2(syn::Token![::])
            {
                let ident = input.parse::<Ident>()?;
                syn::parse_str::<Expr>(&format!("\"{}\"", ident.to_string()))
            } else {
                input.parse::<Expr>()
            }
        })?;
        Ok(UnquotedMacroInput { rows: parsed.rows })
    }
}

#[proc_macro]
pub fn serde_table_expr(input: TokenStream) -> TokenStream {
    let serde_table = parse_macro_input!(input as ExprMacroInput);
    generate_output(serde_table.rows)
}

#[proc_macro]
pub fn serde_table(input: TokenStream) -> TokenStream {
    let serde_table = parse_macro_input!(input as UnquotedMacroInput);
    generate_output(serde_table.rows)
}

// Helper function to avoid code duplication
fn generate_output(rows: Vec<Vec<Expr>>) -> TokenStream {
    let row_expressions = rows.iter().map(|row| {
        let exprs = row.iter();
        quote! {
            vec![#(#exprs.to_string()),*]
        }
    });

    quote! {{
        ::serde_table::parse([#(#row_expressions),*])
    }}
    .into()
}
