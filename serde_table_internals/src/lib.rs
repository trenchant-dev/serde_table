use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    spanned::Spanned,
    Expr, Ident, Result, Token,
};

// For literal data tables (quoted strings only)
struct WsData {
    rows: Vec<Vec<Expr>>,
}

// For tables with expressions (supports unquoted identifiers)
struct WsDataExpr {
    rows: Vec<Vec<Expr>>,
}

impl Parse for WsData {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut rows = Vec::new();
        let mut current_row = Vec::new();
        let mut last_line = 1;

        while !input.is_empty() {
            // Parse each item as an expression
            let expr = input.parse::<Expr>()?;

            // Check if we're on a new line by comparing spans
            let span = expr.span();
            let line = span.start().line;

            if line > last_line && !current_row.is_empty() {
                rows.push(current_row);
                current_row = Vec::new();
            }

            current_row.push(expr);
            last_line = line;

            // Skip any whitespace
            while input.peek(syn::Token![_]) {
                let _ = input.parse::<syn::Token![_]>()?;
            }
        }

        if !current_row.is_empty() {
            rows.push(current_row);
        }

        Ok(WsData { rows })
    }
}

impl Parse for WsDataExpr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut rows = Vec::new();
        let mut current_row = Vec::new();
        let mut last_line = 1;

        while !input.is_empty() {
            let expr = if input.peek(Ident) 
                && !input.peek2(syn::token::Paren)    // fn()
                && !input.peek2(syn::token::Bracket)  // arr[]
                && !input.peek2(syn::token::Brace)    // T{}
                && !input.peek2(syn::Token![.])       // obj.field
                && !input.peek2(syn::Token![::])      // path::to
            {
                // Only convert simple bare identifiers into string literals
                let ident = input.parse::<Ident>()?;
                let ident_str = ident.to_string();
                syn::parse_str::<Expr>(&format!("\"{}\"", ident_str))?
            } else {
                input.parse::<Expr>()?
            };

            let span = expr.span();
            let line = span.start().line;

            if line > last_line && !current_row.is_empty() {
                rows.push(current_row);
                current_row = Vec::new();
            }

            current_row.push(expr);
            last_line = line;

            while input.peek(Token![_]) {
                let _ = input.parse::<Token![_]>()?;
            }
        }

        if !current_row.is_empty() {
            rows.push(current_row);
        }

        Ok(WsDataExpr { rows })
    }
}

#[proc_macro]
pub fn serde_table(input: TokenStream) -> TokenStream {
    let serde_table = parse_macro_input!(input as WsData);
    generate_output(serde_table.rows)
}

#[proc_macro]
pub fn serde_table_expr(input: TokenStream) -> TokenStream {
    let serde_table = parse_macro_input!(input as WsDataExpr);
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
        ::serde_table::StringRows::from_rows(vec![#(#row_expressions),*]).parse()
    }}
    .into()
}
