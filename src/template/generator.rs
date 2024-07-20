use std::fmt;
use std::collections::HashMap;
use std::mem;
use std::{cmp, hash, borrow::Cow};
use std::borrow::Borrow;

use proc_macro2::TokenStream;
use proc_macro2::Span;

use askama_parser::{Node, Expr};
use askama_parser::node::{Lit, Let, Loop, Comment, Whitespace, Ws, Target};

#[derive(Clone, Copy)]
enum DisplayWrap {
    Wrapped,
    Unwrapped,
}

#[derive(Debug)]
enum Writable<'a> {
    Lit(&'a str),
    Expr(&'a Expr<'a>),
}

#[derive(Debug, Clone)]
struct ParsedError {
    msg: Cow<'static, str>,
    span: Span,
}

impl ParsedError {
    fn new<S: Into<Cow<'static, str>>>(s: S, span: Span) -> Self {
        Self {
            msg: s.into(),
            span,
        }
    }

    fn into_compile_error(self) -> TokenStream {
        syn::Error::new(self.span, self.msg)
            .to_compile_error()
            .into()
    }
}

impl std::error::Error for ParsedError {}

impl fmt::Display for ParsedError {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.msg)
    }
}

impl From<&'static str> for ParsedError {
    #[inline]
    fn from(s: &'static str) -> Self {
        Self::new(s, Span::call_site())
    }
}

impl From<String> for ParsedError {
    #[inline]
    fn from(s: String) -> Self {
        Self::new(s, Span::call_site())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize))]
#[cfg_attr(feature = "serde", serde(field_identifier, rename_all = "lowercase"))]
pub(crate) enum WhitespaceHandling {
    /// The default behaviour. It will leave the whitespace characters "as is".
    Preserve,
    /// It'll remove all the whitespace characters before and after the jinja block.
    Suppress,
    /// It'll remove all the whitespace characters except one before and after the jinja blocks.
    /// If there is a newline character, the preserved character in the trimmed characters, it will
    /// the one preserved.
    Minimize,
}

impl From<WhitespaceHandling> for Whitespace {
    fn from(ws: WhitespaceHandling) -> Self {
        match ws {
            WhitespaceHandling::Suppress => Whitespace::Suppress,
            WhitespaceHandling::Preserve => Whitespace::Preserve,
            WhitespaceHandling::Minimize => Whitespace::Minimize,
        }
    }
}

impl Default for WhitespaceHandling {
    fn default() -> Self {
        WhitespaceHandling::Preserve
    }
}

pub struct Generator<'a> {
    // Variables accessible directly from the current scope (not redirected to context)
    locals: MapChain<'a, &'a str, LocalMeta>,

    // Suffix whitespace from the previous literal. Will be flushed to the
    // output buffer unless suppressed by whitespace suppression on the next
    // non-literal.
    next_ws: Option<&'a str>,
    // Whitespace suppression from the previous non-literal. Will be used to
    // determine whether to flush prefix whitespace from the next literal.
    skip_ws: WhitespaceHandling,
    whitespace: WhitespaceHandling,
    buf: Buffer,

    last_loop_var: Option<&'a str>
}

impl<'a> Generator<'a> {
    pub fn new<'n>(
        locals: MapChain<'n, &'n str, LocalMeta>,
    ) -> Generator<'n> {
        Generator {
            locals,
            next_ws: None,
            skip_ws: WhitespaceHandling::Preserve,
            whitespace: WhitespaceHandling::Preserve,
            buf: Buffer::new(0),
            last_loop_var: None
        }
    }

    pub fn handle(
        &mut self,
        nodes: &'a [Node<'_>],
    ) {
        for n in nodes {
            match *n {
                Node::Lit(ref lit) => {
                    let _ = self.buf.writeln(lit.val);
                }
                Node::Comment(ref comment) => {
                    println!("comment\n");
                    self.write_comment(comment);
                }
                Node::Expr(ws, ref val) => {
                    let _ = self.write_expr(ws, val);
                }
                Node::Let(ref l) => {
                    println!("let\n");
                    self.write_let(l);
                }
                Node::If(ref i) => {
                    println!("if\n");
                    //size_hint += self.write_if(ctx, buf, i)?;
                }
                Node::Match(ref m) => {
                    println!("match\n");
                    //size_hint += self.write_match(ctx, buf, m)?;
                }
                Node::Loop(ref loop_block) => {
                    self.write_loop(loop_block);
                }
                Node::BlockDef(ref b) => {
                    println!("block\n");
                    //size_hint += self.write_block(buf, Some(b.name), Ws(b.ws1.0, b.ws2.1))?;
                }
                Node::Include(ref i) => {
                    println!("include\n");
                    //size_hint += self.handle_include(ctx, buf, i)?;
                }
                Node::Call(ref call) => {
                    println!("call\n");
                    //size_hint += self.write_call(ctx, buf, call)?;
                }
                Node::Macro(ref m) => {
                    /*
                    if level != AstLevel::Top {
                        return Err("macro blocks only allowed at the top level".into());
                    }
                    self.flush_ws(m.ws1);
                    self.prepare_ws(m.ws2);
                    */
                }
                Node::Raw(ref raw) => {
                    println!("raw\n");
                    /*
                    self.handle_ws(raw.ws1);
                    self.handle_ws(raw.ws2);
                    */
                }
                Node::Import(ref i) => {
                    /*
                    if level != AstLevel::Top {
                        return Err("import blocks only allowed at the top level".into());
                    }
                    self.handle_ws(i.ws);
                    */
                }
                Node::Extends(_) => {
                    /*
                    if level != AstLevel::Top {
                        return Err("extend blocks only allowed at the top level".into());
                    }
                    // No whitespace handling: child template top-level is not used,
                    // except for the blocks defined in it.
                    */
                }
                Node::Break(ws) => {
                    println!("break\n");
                    /*
                    self.handle_ws(ws);
                    self.write_buf_writable(buf)?;
                    self.buf.writeln("break;")?;
                    */
                }
                Node::Continue(ws) => {
                    /*
                    self.handle_ws(ws);
                    self.write_buf_writable(buf)?;
                    self.buf.writeln("continue;")?;
                    */
                }
                _ => {
                    println!("find\n");
                }
            }
        }
    }

    pub fn render(self) {
        println!("{}", self.buf.buf);
    }

    fn write_loop(
        &mut self,
        loop_block: &'a Loop<'_>,
    ) {
        print!("&& {:?}\n", loop_block.var);
        match loop_block.var {
            Target::Name(val) => {
                self.last_loop_var = Some(val);
            }
            _ => {}
        }

        self.handle_ws(loop_block.ws1);
        self.locals.push();

        match &loop_block.iter {
            Expr::Call(r#box, args) => {
                match r#box.borrow() {
                    Expr::Var(value) => {
                        if *value == "peoples" && args.len() >= 1 {
                            match args[0] {
                                askama_parser::Expr::NumLit(val) => {
                                    if let Ok(len_of_element) = val.parse::<i32>() {
                                        print!(">>> {} <=> {:?}\n", value, len_of_element);
                                        for n in 0..len_of_element {
                                            self.handle(&loop_block.body);
                                        }
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        self.last_loop_var = None;
    }

    fn visit_expr(
        &mut self,
        buf: &mut Buffer,
        expr: &Expr<'_>,
    ) -> Result<DisplayWrap, ParsedError> {
        Ok(match *expr {
            Expr::BoolLit(s) => self.visit_bool_lit(buf, s),
            Expr::NumLit(s) => self.visit_num_lit(buf, s),
            Expr::StrLit(s) => self.visit_str_lit(buf, s),
            Expr::CharLit(s) => self.visit_char_lit(buf, s),
            Expr::Var(s) => self.visit_var(buf, s),
            Expr::Path(ref path) => self.visit_path(buf, path),
            Expr::Array(ref elements) => self.visit_array(buf, elements)?,
            Expr::Attr(ref obj, name) => self.visit_attr(buf, obj, name)?,
            Expr::Index(ref obj, ref key) => self.visit_index(buf, obj, key)?,
            Expr::Filter(name, ref args) => self.visit_filter(buf, name, args)?,
            Expr::Unary(op, ref inner) => self.visit_unary(buf, op, inner)?,
            Expr::BinOp(op, ref left, ref right) => self.visit_binop(buf, op, left, right)?,
            Expr::Range(op, ref left, ref right) => {
                self.visit_range(buf, op, left.as_deref(), right.as_deref())?
            }
            Expr::Group(ref inner) => self.visit_group(buf, inner)?,
            Expr::Call(ref obj, ref args) => self.visit_call(buf, obj, args)?,
            //Expr::RustMacro(ref path, args) => self.visit_rust_macro(buf, path, args),
            //Expr::Try(ref expr) => self.visit_try(buf, expr.as_ref())?,
            //Expr::Tuple(ref exprs) => self.visit_tuple(buf, exprs)?,
            _ => DisplayWrap::Unwrapped
        })
    }

    fn visit_filter(
        &mut self,
        buf: &mut Buffer,
        mut name: &str,
        args: &[Expr<'_>],
    ) -> Result<DisplayWrap, ParsedError> {
        Ok(DisplayWrap::Unwrapped)
    }

    fn visit_attr(
        &mut self,
        buf: &mut Buffer,
        obj: &Expr<'_>,
        attr: &str,
    ) -> Result<DisplayWrap, ParsedError> {
        if let Expr::Var(name) = *obj {
            if name == "loop" {
                if attr == "index" {
                    self.buf.write("(_loop_item.index + 1)");
                    return Ok(DisplayWrap::Unwrapped);
                } else if attr == "index0" {
                    self.buf.write("_loop_item.index");
                    return Ok(DisplayWrap::Unwrapped);
                } else if attr == "first" {
                    self.buf.write("_loop_item.first");
                    return Ok(DisplayWrap::Unwrapped);
                } else if attr == "last" {
                    self.buf.write("_loop_item.last");
                    return Ok(DisplayWrap::Unwrapped);
                } /*else {
                    return Err("unknown loop variable".into());
                }*/
            }
        }
        self.visit_expr(buf, obj)?;
        self.buf.write(&format!(".{}", normalize_identifier(attr)));
        Ok(DisplayWrap::Unwrapped)
    }

    fn visit_index(
        &mut self,
        buf: &mut Buffer,
        obj: &Expr<'_>,
        key: &Expr<'_>,
    ) -> Result<DisplayWrap, ParsedError> {
        self.buf.write("&");
        self.visit_expr(buf, obj)?;
        println!("yo\n");
        self.buf.write("[");
        self.visit_expr(buf, key)?;
        self.buf.write("]");
        Ok(DisplayWrap::Unwrapped)
    }

    fn visit_unary(
        &mut self,
        buf: &mut Buffer,
        op: &str,
        inner: &Expr<'_>,
    ) -> Result<DisplayWrap, ParsedError> {
        self.buf.write(op);
        self.visit_expr(buf, inner)?;
        Ok(DisplayWrap::Unwrapped)
    }

    fn visit_range(
        &mut self,
        buf: &mut Buffer,
        op: &str,
        left: Option<&Expr<'_>>,
        right: Option<&Expr<'_>>,
    ) -> Result<DisplayWrap, ParsedError> {
        if let Some(left) = left {
            self.visit_expr(buf, left)?;
        }
        self.buf.write(op);
        if let Some(right) = right {
            self.visit_expr(buf, right)?;
        }
        Ok(DisplayWrap::Unwrapped)
    }

    fn visit_binop(
        &mut self,
        buf: &mut Buffer,
        op: &str,
        left: &Expr<'_>,
        right: &Expr<'_>,
    ) -> Result<DisplayWrap, ParsedError> {
        self.visit_expr(buf, left)?;
        self.buf.write(&format!(" {op} "));
        self.visit_expr(buf, right)?;
        Ok(DisplayWrap::Unwrapped)
    }

    fn visit_group(
        &mut self,
        buf: &mut Buffer,
        inner: &Expr<'_>,
    ) -> Result<DisplayWrap, ParsedError> {
        self.buf.write("(");
        self.visit_expr(buf, inner)?;
        self.buf.write(")");
        Ok(DisplayWrap::Unwrapped)
    }

    fn visit_tuple(
        &mut self,
        buf: &mut Buffer,
        exprs: &[Expr<'_>],
    ) -> Result<DisplayWrap, ParsedError> {
        self.buf.write("(");
        for (index, expr) in exprs.iter().enumerate() {
            if index > 0 {
                self.buf.write(" ");
            }
            self.visit_expr(buf, expr)?;
            self.buf.write(",");
        }
        self.buf.write(")");
        Ok(DisplayWrap::Unwrapped)
    }

    fn visit_array(
        &mut self,
        buf: &mut Buffer,
        elements: &[Expr<'_>],
    ) -> Result<DisplayWrap, ParsedError> {
        println!("yo\n");
        self.buf.write("[");
        for (i, el) in elements.iter().enumerate() {
            if i > 0 {
                self.buf.write(", ");
            }
            self.visit_expr(buf, el)?;
        }
        self.buf.write("]");
        Ok(DisplayWrap::Unwrapped)
    }

    fn visit_path(&mut self, buf: &mut Buffer, path: &[&str]) -> DisplayWrap {
        for (i, part) in path.iter().enumerate() {
            if i > 0 {
                self.buf.write("::");
            }
            self.buf.write(part);
        }
        DisplayWrap::Unwrapped
    }

    fn visit_var(&mut self, buf: &mut Buffer, s: &str) -> DisplayWrap {
        if s == "self" {
            self.buf.write(s);
            return DisplayWrap::Unwrapped;
        }

        self.buf.write(normalize_identifier(&self.locals.resolve_or_self(s)));
        DisplayWrap::Unwrapped
    }

    fn visit_bool_lit(&mut self, buf: &mut Buffer, s: &str) -> DisplayWrap {
        self.buf.write(s);
        DisplayWrap::Unwrapped
    }

    fn visit_str_lit(&mut self, buf: &mut Buffer, s: &str) -> DisplayWrap {
        self.buf.write(&format!("\"{s}\""));
        DisplayWrap::Unwrapped
    }

    fn visit_char_lit(&mut self, buf: &mut Buffer, s: &str) -> DisplayWrap {
        self.buf.write(&format!("'{s}'"));
        DisplayWrap::Unwrapped
    }

    fn visit_num_lit(&mut self, buf: &mut Buffer, s: &str) -> DisplayWrap {
        self.buf.write(s);
        DisplayWrap::Unwrapped
    }
    
    fn write_expr(&mut self, ws: Ws, val: &'a Expr<'a>) -> Result<DisplayWrap, ParsedError> {
        match val {
            Expr::Attr(attrOne, nameOne) => {
                match attrOne.borrow() {
                    Expr::Var(name) => {
                        if let Some(last_loop_var) = self.last_loop_var {
                            if last_loop_var != *name {
                                return Err("\"{name}.{nameOne}\" doesn't exist. Did you mean \"{last_loop_var}.{nameOne}\" ?".into());
                            }
                            return Ok(DisplayWrap::Unwrapped);
                        }
                        print!("{name}.{nameOne}\n");
                        Ok(DisplayWrap::Unwrapped)
                    }
                    _val => {
                        print!(">prout {:?}<\n", _val);
                        Ok(DisplayWrap::Unwrapped)
                    }
                }
            }
            _ => {
                Ok(DisplayWrap::Unwrapped)
            }
        }
        //self.handle_ws(ws);
    }

    fn write_let(&mut self, l: &'a Let<'_>) {
        self.handle_ws(l.ws);

        let Some(val) = &l.val else {
            let _ = self.buf.write("let ");
            //self.visit_target(false, true, &l.var);
            let _ = self.buf.writeln(";");
            return;
        };
    }

    fn visit_call(
        &mut self,
        buf: &mut Buffer,
        left: &Expr<'_>,
        args: &[Expr<'_>],
    ) -> Result<DisplayWrap, ParsedError> {
        match left {
            Expr::Attr(left, method) if **left == Expr::Var("loop") => match *method {
                "cycle" => match args {
                    [arg] => {
                        if matches!(arg, Expr::Array(arr) if arr.is_empty()) {
                            return Err("loop.cycle(…) cannot use an empty array".into());
                        }
                        /*
                        self.buf.write("({");
                        self.buf.write("let _cycle = &(");
                        self.visit_expr(buf, arg)?;
                        self.buf.writeln(");")?;
                        self.buf.writeln("let _len = _cycle.len();")?;
                        self.buf.writeln("if _len == 0 {")?;
                        self.buf.writeln("return ::core::result::Result::Err(::askama::Error::Fmt(::core::fmt::Error));")?;
                        self.buf.writeln("}")?;
                        self.buf.writeln("_cycle[_loop_item.index % _len]")?;
                        self.buf.writeln("})")?;
                        */
                    }
                    _ => return Err("loop.cycle(…) expects exactly one argument".into()),
                },
                s => return Err(format!("unknown loop method: {s:?}").into()),
            },
            left => {                
                match left {
                    Expr::Var(name) => match self.locals.resolve(name) {
                        Some(resolved) => self.buf.write(&resolved),
                        None => self.buf.write(&format!("(&self.{})", normalize_identifier(name))),
                    },
                    left => {
                        self.visit_expr(buf, left)?;
                    }
                }

                self.buf.write("(");
                self._visit_args(buf, args)?;
                self.buf.write(")");
            }
        }
        Ok(DisplayWrap::Unwrapped)
    }

    fn _visit_args(&mut self, buf: &mut Buffer, args: &[Expr<'_>]) -> Result<(), ParsedError> {
        if args.is_empty() {
            return Ok(());
        }

        for (i, arg) in args.iter().enumerate() {
            if i > 0 {
                self.buf.write(", ");
            }

            let borrow = !is_copyable(arg);
            if borrow {
                self.buf.write("&(");
            }

            match arg {
                Expr::Call(left, _) if !matches!(left.as_ref(), Expr::Path(_)) => {
                    //self.buf.writeln("{")?;
                    self.visit_expr(buf, arg)?;
                    //self.buf.writeln("}")?;
                }
                _ => {
                    self.visit_expr(buf, arg)?;
                }
            }

            if borrow {
                self.buf.write(")");
            }
        }
        Ok(())
    }

    fn should_trim_ws(&self, ws: Option<Whitespace>) -> WhitespaceHandling {
        match ws {
            Some(Whitespace::Suppress) => WhitespaceHandling::Suppress,
            Some(Whitespace::Preserve) => WhitespaceHandling::Preserve,
            Some(Whitespace::Minimize) => WhitespaceHandling::Minimize,
            None => self.whitespace,
        }
    }

    fn write_comment(&mut self, comment: &'a Comment<'_>) {
        self.handle_ws(comment.ws);
    }

   // Combines `flush_ws()` and `prepare_ws()` to handle both trailing whitespace from the
    // preceding literal and leading whitespace from the succeeding literal.
    fn handle_ws(&mut self, ws: Ws) {
        self.flush_ws(ws);
        self.prepare_ws(ws);
    }

    // If the previous literal left some trailing whitespace in `next_ws` and the
    // prefix whitespace suppressor from the given argument, flush that whitespace.
    // In either case, `next_ws` is reset to `None` (no trailing whitespace).
    fn flush_ws(&mut self, ws: Ws) {
        if self.next_ws.is_none() {
            return;
        }

        // If `whitespace` is set to `suppress`, we keep the whitespace characters only if there is
        // a `+` character.
        /*
        match self.should_trim_ws(ws.0) {
            WhitespaceHandling::Preserve => {
                let val = self.next_ws.unwrap();
                if !val.is_empty() {
                    self.buf_writable.push(Writable::Lit(val));
                }
            }
            WhitespaceHandling::Minimize => {
                let val = self.next_ws.unwrap();
                if !val.is_empty() {
                    self.buf_writable
                        .push(Writable::Lit(match val.contains('\n') {
                            true => "\n",
                            false => " ",
                        }));
                }
            }
            WhitespaceHandling::Suppress => {}
            
        }
        */
        self.next_ws = None;
    }

    // Sets `skip_ws` to match the suffix whitespace suppressor from the given
    // argument, to determine whether to suppress leading whitespace from the
    // next literal.
    fn prepare_ws(&mut self, ws: Ws) {
        self.skip_ws = self.should_trim_ws(ws.1);
    }
}

struct Buffer {
    // The buffer to generate the code into
    buf: String,
    // The current level of indentation (in spaces)
    indent: u8,
    // Whether the output buffer is currently at the start of a line
    start: bool,
}

use core::fmt::Error;
impl Buffer {
    fn new(indent: u8) -> Self {
        Self {
            buf: String::new(),
            indent,
            start: true,
        }
    }

    fn writeln(&mut self, s: &str) -> Result<(), Error> {
        if !s.is_empty() {
            self.write(s);
        }
        self.buf.push('\n');

        self.start = true;
        Ok(())
    }

    fn write(&mut self, s: &str) {
        if self.start {
            for _ in 0..(self.indent * 4) {
                self.buf.push(' ');
            }
            self.start = false;
        }
        self.buf.push_str(s);
    }

    fn indent(&mut self) {
        self.indent += 1;
    }
}

#[derive(Clone, Default)]
pub(crate) struct LocalMeta {
    refs: Option<String>,
    initialized: bool,
}

impl LocalMeta {
    fn initialized() -> Self {
        Self {
            refs: None,
            initialized: true,
        }
    }

    fn with_ref(refs: String) -> Self {
        Self {
            refs: Some(refs),
            initialized: true,
        }
    }
}

#[derive(Debug)]
pub(crate) struct MapChain<'a, K, V>
where
    K: cmp::Eq + hash::Hash,
{
    parent: Option<&'a MapChain<'a, K, V>>,
    scopes: Vec<HashMap<K, V>>,
}

impl<'a, K: 'a, V: 'a> MapChain<'a, K, V>
where
    K: cmp::Eq + hash::Hash,
{
    fn with_parent<'p>(parent: &'p MapChain<'_, K, V>) -> MapChain<'p, K, V> {
        MapChain {
            parent: Some(parent),
            scopes: vec![HashMap::new()],
        }
    }

    /// Iterates the scopes in reverse and returns `Some(LocalMeta)`
    /// from the first scope where `key` exists.
    fn get(&self, key: &K) -> Option<&V> {
        let mut scopes = self.scopes.iter().rev();
        scopes
            .find_map(|set| set.get(key))
            .or_else(|| self.parent.and_then(|set| set.get(key)))
    }

    fn is_current_empty(&self) -> bool {
        self.scopes.last().unwrap().is_empty()
    }

    fn insert(&mut self, key: K, val: V) {
        self.scopes.last_mut().unwrap().insert(key, val);

        // Note that if `insert` returns `Some` then it implies
        // an identifier is reused. For e.g. `{% macro f(a, a) %}`
        // and `{% let (a, a) = ... %}` then this results in a
        // generated template, which when compiled fails with the
        // compile error "identifier `a` used more than once".
    }

    fn insert_with_default(&mut self, key: K)
    where
        V: Default,
    {
        self.insert(key, V::default());
    }

    fn push(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop(&mut self) {
        self.scopes.pop().unwrap();
        assert!(!self.scopes.is_empty());
    }
}

impl MapChain<'_, &str, LocalMeta> {
    fn resolve(&self, name: &str) -> Option<String> {
        let name = normalize_identifier(name);
        self.get(&name).map(|meta| match &meta.refs {
            Some(expr) => expr.clone(),
            None => name.to_string(),
        })
    }

    fn resolve_or_self(&self, name: &str) -> String {
        let name = normalize_identifier(name);
        self.resolve(name).unwrap_or_else(|| format!("self.{name}"))
    }
}

impl<'a, K: Eq + hash::Hash, V> Default for MapChain<'a, K, V> {
    fn default() -> Self {
        Self {
            parent: None,
            scopes: vec![HashMap::new()],
        }
    }
}

// Identifiers to be replaced with raw identifiers, so as to avoid
// collisions between template syntax and Rust's syntax. In particular
// [Rust keywords](https://doc.rust-lang.org/reference/keywords.html)
// should be replaced, since they're not reserved words in Askama
// syntax but have a high probability of causing problems in the
// generated code.
//
// This list excludes the Rust keywords *self*, *Self*, and *super*
// because they are not allowed to be raw identifiers, and *loop*
// because it's used something like a keyword in the template
// language.
static USE_RAW: [(&str, &str); 47] = [
    ("as", "r#as"),
    ("break", "r#break"),
    ("const", "r#const"),
    ("continue", "r#continue"),
    ("crate", "r#crate"),
    ("else", "r#else"),
    ("enum", "r#enum"),
    ("extern", "r#extern"),
    ("false", "r#false"),
    ("fn", "r#fn"),
    ("for", "r#for"),
    ("if", "r#if"),
    ("impl", "r#impl"),
    ("in", "r#in"),
    ("let", "r#let"),
    ("match", "r#match"),
    ("mod", "r#mod"),
    ("move", "r#move"),
    ("mut", "r#mut"),
    ("pub", "r#pub"),
    ("ref", "r#ref"),
    ("return", "r#return"),
    ("static", "r#static"),
    ("struct", "r#struct"),
    ("trait", "r#trait"),
    ("true", "r#true"),
    ("type", "r#type"),
    ("unsafe", "r#unsafe"),
    ("use", "r#use"),
    ("where", "r#where"),
    ("while", "r#while"),
    ("async", "r#async"),
    ("await", "r#await"),
    ("dyn", "r#dyn"),
    ("abstract", "r#abstract"),
    ("become", "r#become"),
    ("box", "r#box"),
    ("do", "r#do"),
    ("final", "r#final"),
    ("macro", "r#macro"),
    ("override", "r#override"),
    ("priv", "r#priv"),
    ("typeof", "r#typeof"),
    ("unsized", "r#unsized"),
    ("virtual", "r#virtual"),
    ("yield", "r#yield"),
    ("try", "r#try"),
];

fn normalize_identifier(ident: &str) -> &str {
    if let Some(word) = USE_RAW.iter().find(|x| x.0 == ident) {
        word.1
    } else {
        ident
    }
}

// This is used by the code generator to decide whether a named filter is part of
// Askama or should refer to a local `filters` module. It should contain all the
// filters shipped with Askama, even the optional ones (since optional inclusion
// in the const vector based on features seems impossible right now).
const BUILT_IN_FILTERS: &[&str] = &[
    "abs",
    "capitalize",
    "center",
    "e",
    "escape",
    "filesizeformat",
    "fmt",
    "format",
    "indent",
    "into_f64",
    "into_isize",
    "join",
    "linebreaks",
    "linebreaksbr",
    "paragraphbreaks",
    "lower",
    "lowercase",
    "safe",
    "trim",
    "truncate",
    "upper",
    "uppercase",
    "urlencode",
    "urlencode_strict",
    "wordcount",
    // optional features, reserve the names anyway:
    "json",
    "markdown",
    "yaml",
];

/// Returns `true` if enough assumptions can be made,
/// to determine that `self` is copyable.
fn is_copyable(expr: &Expr<'_>) -> bool {
    is_copyable_within_op(expr, false)
}

fn is_copyable_within_op(expr: &Expr<'_>, within_op: bool) -> bool {
    use Expr::*;
    match expr {
        BoolLit(_) | NumLit(_) | StrLit(_) | CharLit(_) => true,
        Unary(.., expr) => is_copyable_within_op(expr, true),
        BinOp(_, lhs, rhs) => is_copyable_within_op(lhs, true) && is_copyable_within_op(rhs, true),
        Range(..) => true,
        // The result of a call likely doesn't need to be borrowed,
        // as in that case the call is more likely to return a
        // reference in the first place then.
        Call(..) | Path(..) => true,
        // If the `expr` is within a `Unary` or `BinOp` then
        // an assumption can be made that the operand is copy.
        // If not, then the value is moved and adding `.clone()`
        // will solve that issue. However, if the operand is
        // implicitly borrowed, then it's likely not even possible
        // to get the template to compile.
        _ => within_op && is_attr_self(expr),
    }
}

/// Returns `true` if this is an `Attr` where the `obj` is `"self"`.
pub(crate) fn is_attr_self(expr: &Expr<'_>) -> bool {
    match expr {
        Expr::Attr(obj, _) if matches!(obj.as_ref(), Expr::Var("self")) => true,
        Expr::Attr(obj, _) if matches!(obj.as_ref(), Expr::Attr(..)) => is_attr_self(expr),
        _ => false,
    }
}
