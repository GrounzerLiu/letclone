//! # letclone
//!
//! A procedural macro for convenient variable cloning in Rust.
//!
//! ## Overview
//!
//! `letclone` provides a [`clone!`] macro that simplifies the common pattern of
//! cloning variables into new bindings. Instead of writing verbose `let` statements
//! with `.clone()` calls, you can use the concise `clone!` macro.
//!
//! The macro is especially useful when working with closures that need to capture
//! cloned values, as it reduces boilerplate code significantly.
//!
//! ## Notes
//!
//! `clone!` transparently unwraps `syn::Expr::Group` and continues processing the
//! inner expression. This matters mostly for tokens produced by macro expansion,
//! where `Group` is an implementation detail rather than a user-written Rust
//! expression.
//!
//! This is different from a source-level parenthesized expression such as `(a)`
//! or `(a + b)`, which parses as `syn::Expr::Paren` and is not treated as a
//! supported clone target.
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```rust
//! use letclone::clone;
//!
//! let original = String::from("hello");
//! clone!(original);
//! // Equivalent to: let original = original.clone();
//! ```
//!
//! ### Field Access
//!
//! ```rust
//! use letclone::clone;
//!
//! struct Person {
//!     name: String,
//! }
//!
//! let person = Person { name: String::from("Alice") };
//! clone!(person.name);
//! // Equivalent to: let name = person.name.clone();
//! assert_eq!(name, "Alice");
//! ```
//!
//! ### Mutable Bindings
//!
//! ```rust
//! use letclone::clone;
//!
//! let original = String::from("hello");
//! clone!(mut original);
//! original.push_str(" world");
//! assert_eq!(original, "hello world");
//! ```
//!
//! ### Multiple Expressions
//!
//! ```rust
//! use letclone::clone;
//!
//! let a = String::from("a");
//! let b = String::from("b");
//! clone!(a, b);
//! // Equivalent to:
//! // let a = a.clone();
//! // let b = b.clone();
//! ```
//!
//! ### Usage in Closures
//!
//! ```rust
//! use letclone::clone;
//!
//! let name = String::from("Alice");
//! let scores = vec![85, 90, 95];
//!
//! let closure = {
//!     clone!(name, scores);
//!     move || {
//!         println!("Name: {}, Scores: {:?}", name, scores);
//!     }
//! };
//! ```

use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{Expr, ExprGroup, Token};

/// Represents a cloneable expression with optional `mut` modifier
struct CloneExpr {
    mutability: Option<Token![mut]>,
    inner: Expr,
}

impl Parse for CloneExpr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mutability = if input.peek(Token![mut]) {
            Some(input.parse()?)
        } else {
            None
        };
        let inner: Expr = input.parse()
            .map_err(|e| syn::Error::new(e.span(), "expected a valid expression: field access (a.b), tuple index access (a.0), method call (a.method()), or path (var)"))?;
        Ok(CloneExpr { mutability, inner })
    }
}

impl ToTokens for CloneExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(quote! { let });
        if let Some(m) = &self.mutability {
            tokens.extend(quote! { #m });
        }
        let inner = &self.inner;
        extend(inner, tokens);
    }
}

fn expr_variant_description(expr: &Expr) -> &'static str {
    match expr {
        Expr::Array(_) => "array expression",
        Expr::Assign(_) => "assignment expression",
        Expr::Async(_) => "async block",
        Expr::Await(_) => "await expression",
        Expr::Binary(_) => "binary expression",
        Expr::Block(_) => "block expression",
        Expr::Break(_) => "break expression",
        Expr::Call(_) => "function call expression",
        Expr::Cast(_) => "cast expression",
        Expr::Closure(_) => "closure expression",
        Expr::Const(_) => "const block",
        Expr::Continue(_) => "continue expression",
        Expr::Field(_) => "field access expression",
        Expr::ForLoop(_) => "for loop expression",
        Expr::Group(_) => "grouped expression",
        Expr::If(_) => "if expression",
        Expr::Index(_) => "index expression",
        Expr::Infer(_) => "inferred expression",
        Expr::Let(_) => "let expression",
        Expr::Lit(_) => "literal expression",
        Expr::Loop(_) => "loop expression",
        Expr::Macro(_) => "macro expression",
        Expr::Match(_) => "match expression",
        Expr::MethodCall(_) => "method call expression",
        Expr::Paren(_) => "parenthesized expression",
        Expr::Path(_) => "path expression",
        Expr::Range(_) => "range expression",
        Expr::RawAddr(_) => "raw address expression",
        Expr::Reference(_) => "reference expression",
        Expr::Repeat(_) => "array repeat expression",
        Expr::Return(_) => "return expression",
        Expr::Struct(_) => "struct literal expression",
        Expr::Try(_) => "try expression",
        Expr::TryBlock(_) => "try block",
        Expr::Tuple(_) => "tuple expression",
        Expr::Unary(_) => "unary expression",
        Expr::Unsafe(_) => "unsafe block",
        Expr::Verbatim(_) => "verbatim expression",
        Expr::While(_) => "while expression",
        Expr::Yield(_) => "yield expression",
        _ => "expression",
    }
}

fn extend(expr: &Expr, tokens: &mut proc_macro2::TokenStream) {
    match expr {
        Expr::Field(syn::ExprField {
            base,
            member: syn::Member::Named(field_name),
            ..
        }) => {
            tokens.extend(quote! {
                #field_name = #base.#field_name.clone();
            });
        }
        Expr::Field(syn::ExprField {
            base,
            member: syn::Member::Unnamed(index),
            ..
        }) => {
            let index_num = index.index;
            let ident = syn::Ident::new(&format!("field_{}", index_num), index.span());
            tokens.extend(quote! {
                #ident = #base.#index.clone();
            });
        }
        Expr::MethodCall(expr_method_call) => {
            let method = &expr_method_call.method;
            tokens.extend(quote! {
                #method = #expr.clone();
            });
        }
        Expr::Path(syn::ExprPath { path, .. }) => {
            let ident = &path.segments.last().unwrap().ident;
            tokens.extend(quote! {
                #ident = #expr.clone();
            });
        }
        Expr::Group(ExprGroup { expr, .. }) => {
            extend(&expr, tokens);
        }
        _ => {
            panic!(
                "clone! macro does not support {}. Supported forms: field access (`a.b`), tuple index access (`a.0`), method call (`a.method()`), or path (`var`).",
                expr_variant_description(expr)
            );
        }
    }
}

/// Represents a list of clone expressions
struct CloneExprList {
    exprs: Vec<CloneExpr>,
}

impl Parse for CloneExprList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut exprs = Vec::new();
        while !input.is_empty() {
            let expr: CloneExpr = input.parse().map_err(|e| {
                syn::Error::new(e.span(), format!("failed to parse clone expression: {}", e))
            })?;
            exprs.push(expr);
            if input.peek(Token![,]) {
                let _comma: Token![,] = input.parse()?;
            } else {
                break;
            }
        }
        if exprs.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "clone! macro requires at least one expression",
            ));
        }
        Ok(CloneExprList { exprs })
    }
}

impl ToTokens for CloneExprList {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for expr in &self.exprs {
            expr.to_tokens(tokens);
        }
    }
}

/// Generates `let var = expr.clone();` statements for one or more expressions
///
/// # Supported expression types
/// - Field access: `clone!(obj.field)` -> `let field = obj.field.clone();`
/// - Nested field access: `clone!(a.b.c)` -> `let c = a.b.c.clone();`
/// - Tuple index: `clone!(tuple.0)` -> `let field_0 = tuple.0.clone();`
/// - Nested tuple index: `clone!(obj.tuple.0)` -> `let field_0 = obj.tuple.0.clone();`
/// - Method call: `clone!(obj.method())` -> `let method = obj.method().clone();`
/// - Nested method call: `clone!(a.b.method())` -> `let method = a.b.method().clone();`
/// - Path/variable: `clone!(var)` -> `let var = var.clone();`
///
/// # Using `mut` modifier
/// - `clone!(mut obj.field)` -> `let mut field = obj.field.clone();`
/// - `clone!(mut tuple.0)` -> `let mut field_0 = tuple.0.clone();`
/// - `clone!(mut a.b.c)` -> `let mut c = a.b.c.clone();`
///
/// # Multiple expressions
/// - `clone!(a, b.field, mut c)` -> generates multiple let statements
///
/// # Usage in closures
/// The macro is particularly useful for cloning values before moving them into closures:
/// ```
/// use letclone::clone;
///
/// let name = String::from("Alice");
/// let scores = vec![85, 90, 95];
///
/// let closure = {
///     clone!(name, scores);
///     move || {
///         println!("Name: {}, Scores: {:?}", name, scores);
///     }
/// };
/// ```
#[proc_macro]
pub fn clone(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let expr_list = syn::parse_macro_input!(input as CloneExprList);
    let mut tokens = proc_macro2::TokenStream::new();
    expr_list.to_tokens(&mut tokens);
    proc_macro::TokenStream::from(tokens)
}
