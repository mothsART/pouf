use std::fmt;
use std::collections::HashMap;
use std::{cmp, hash, borrow::Cow};
use std::borrow::Borrow;

use proc_macro2::TokenStream;
use proc_macro2::Span;

use askama_parser::{Node, Expr};
use askama_parser::node::{Lit, Loop, Whitespace, Ws, Target};

#[derive(Clone, Copy, PartialEq)]
pub enum AstLevel {
    Top,
    Block,
    Nested,
}

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
pub struct ParsedError {
    pub msg: Cow<'static, str>,
}

impl ParsedError {
    fn new<S: Into<Cow<'static, str>>>(s: S, span: Span) -> Self {
        Self {
            msg: s.into(),
        }
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
    pub fn new<'n>() -> Generator<'n> {
        Generator {
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
        level: AstLevel,
    ) -> Result<usize, ParsedError> {
        let mut size_hint = 0;
        for n in nodes {
            match *n {
                Node::Lit(ref lit) => {
                    self.visit_lit(lit);
                }
                Node::Expr(ws, ref val) => {
                    let _ = self.write_expr(ws, val)?;
                }
                Node::If(ref i) => {
                    println!("if\n");
                    //size_hint += self.write_if(ctx, buf, i)?;
                }
                Node::Loop(ref loop_block) => {
                    self.write_loop(loop_block)?;
                }
                _ => {
                    println!("find\n");
                }
            }
        }
        if AstLevel::Top == level {
            println!("cool\n");
            //self.next_ws = Some(self.skip_ws.into());
        }
        Ok(size_hint)
    }

    pub fn render(self) -> String {
        self.buf.buf
    }

    fn visit_lit(&mut self, lit: &'a Lit<'_>) {
        //assert!(self.next_ws.is_none());
        let Lit { lws, val, rws } = *lit;
        if !lws.is_empty() {
            match self.skip_ws {
                WhitespaceHandling::Suppress => {}
                _ if val.is_empty() => {
                    assert!(rws.is_empty());
                    self.next_ws = Some(lws);
                }
                WhitespaceHandling::Preserve => {
                    self.buf.write(lws);
                },
                WhitespaceHandling::Minimize => {
                    match lws.contains('\n') {
                        true => {
                            self.buf.write("\n");
                        }
                        false => {
                            self.buf.write(" ");
                        }
                    };
                }
            }
        }

        if !val.is_empty() {
            self.skip_ws = WhitespaceHandling::Preserve;
            self.buf.write(val);
        }

        if !rws.is_empty() {
            self.next_ws = Some(rws);
        }
    }

    fn write_loop(
        &mut self,
        loop_block: &'a Loop<'_>,
    ) -> Result<usize, ParsedError> {
        match loop_block.var {
            Target::Name(val) => {
                self.last_loop_var = Some(val);
            }
            _ => {}
        }

        match &loop_block.iter {
            Expr::Call(r#box, args) => {
                match r#box.borrow() {
                    Expr::Var(value) => {
                        if *value == "peoples" && args.len() >= 1 {
                            match args[0] {
                                askama_parser::Expr::NumLit(val) => {
                                    if let Ok(len_of_element) = val.parse::<i32>() {
                                        for n in 0..len_of_element {
                                            self.handle(&loop_block.body, AstLevel::Nested)?;
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

        Ok(0)
    }
    
    fn write_expr(&mut self, ws: Ws, val: &'a Expr<'a>) -> Result<DisplayWrap, ParsedError> {
        match val {
            Expr::Attr(attr_one, name_one) => {
                match attr_one.borrow() {
                    Expr::Var(name) => {
                        if let Some(last_loop_var) = self.last_loop_var {
                            if last_loop_var != *name {
                                return Err(format!("\"{name}.{name_one}\" doesn't exist. Did you mean \"{last_loop_var}.{name_one}\" ?").into());
                            }
                            self.buf.write("let ");
                            return Ok(DisplayWrap::Unwrapped);
                        }
                        print!("{name}.{name_one}\n");
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
    }
}

struct Buffer {
    // The buffer to generate the code into
    buf: String,
    // The current level of indentation (in spaces)
    indent: u8,
    // Whether the output buffer is currently at the start of a line
    start: bool,
    discard: bool,
}

use core::fmt::Error;
impl Buffer {
    fn new(indent: u8) -> Self {
        Self {
            buf: String::new(),
            indent,
            start: true,
            discard: false,
        }
    }

    fn writeln(&mut self, s: &str) -> Result<(), Error> {
        if self.discard {
            return Ok(());
        }
        if !s.is_empty() {
            print!("salut {s}\n");
            self.write(s);
        }
        self.buf.push('\n');

        self.start = true;
        Ok(())
    }

    fn write(&mut self, s: &str) {
        if self.discard {
            return;
        }
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
