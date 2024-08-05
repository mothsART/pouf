use std::fmt;
use std::borrow::Cow;
use std::borrow::Borrow;

use clap::ArgMatches;

use askama_parser::{Node, Expr};
use askama_parser::node::{Lit, Loop, Whitespace, Target};

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
    fn new<S: Into<Cow<'static, str>>>(s: S) -> Self {
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
        Self::new(s)
    }
}

impl From<String> for ParsedError {
    #[inline]
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum WhitespaceHandling {
    /// The default behaviour. It will leave the whitespace characters "as is".
    Preserve,
}

impl From<WhitespaceHandling> for Whitespace {
    fn from(ws: WhitespaceHandling) -> Self {
        match ws {
            WhitespaceHandling::Preserve => Whitespace::Preserve,
        }
    }
}

impl Default for WhitespaceHandling {
    fn default() -> Self {
        WhitespaceHandling::Preserve
    }
}

struct LoopObject {
    people: People,
    job: Job,
    coordinates: Coordinate,
}

pub struct Generator<'a> {
    // Suffix whitespace from the previous literal. Will be flushed to the
    // output buffer unless suppressed by whitespace suppression on the next
    // non-literal.
    next_ws: Option<&'a str>,
    // Whitespace suppression from the previous non-literal. Will be used to
    // determine whether to flush prefix whitespace from the next literal.
    skip_ws: WhitespaceHandling,
    buf: Buffer,
    last_loop_var: Option<&'a str>,
    last_loop_object: LoopObject,
    template_m: &'a ArgMatches,
}

impl<'a> Generator<'a> {
    pub fn new<'n>(template_m: &'n ArgMatches) -> Generator<'n> {
        Generator {
            next_ws: None,
            skip_ws: WhitespaceHandling::Preserve,
            buf: Buffer::new(0),
            last_loop_var: None,
            last_loop_object: LoopObject {
                people: People::create(template_m),
                job: Job::create(template_m),
                coordinates: Coordinate::create(&template_m),
            },
            template_m: template_m
        }
    }

    pub fn handle(
        &mut self,
        nodes: &'a [Node<'_>],
        level: AstLevel,
    ) -> Result<usize, ParsedError> {
        let size_hint = 0;
        for n in nodes {
            match *n {
                Node::Lit(ref lit) => {
                    self.visit_lit(lit);
                }
                Node::Expr(_ws, ref val) => {
                    let mut _vec: Vec<&str> = vec![];
                    let _ = self.write_expr(val, &mut _vec)?;
                }
                Node::If(ref _i) => {
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
                WhitespaceHandling::Preserve => {
                    self.buf.write(lws);
                }
                /*
                WhitespaceHandling::Suppress => {}
                _ if val.is_empty() => {
                    assert!(rws.is_empty());
                    self.next_ws = Some(lws);
                }
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
                */
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
                                        for _n in 0..len_of_element {
                                            self.last_loop_object.people = People::create(&self.template_m);
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
    
    fn write_people(&mut self, val: &'a Expr<'a>, name: &str, property: &str, attrs: &mut Vec<&'a str>) -> Result<DisplayWrap, ParsedError> {
        if let Some(last_loop_var) = self.last_loop_var {
            if last_loop_var != name {
                return Ok(DisplayWrap::Unwrapped);
                //return Err(format!("\"{name}.{property}\" doesn't exist. Did you mean \"{last_loop_var}.{property}\" ?").into());
            }
            if let Some(property_value) = self.last_loop_object.people.get_property(property) {
                if let Some(name) = property_value.downcast_ref::<String>() {
                    self.buf.write(&name);
                } else {
                    return Ok(DisplayWrap::Unwrapped);
                    //return Err(format!("\"{property}\" is an object on \"{name}.{property}\". You must use properties.").into());
                }
            } else {
                match val {
                    Expr::Attr(attr, name_one) => {
                        match attr.borrow() {
                            Expr::Var(parent_name) => {
                                println!("&> {parent_name}");
                                return Ok(DisplayWrap::Unwrapped);
                            }
                            _val_two => {
                                return Ok(DisplayWrap::Unwrapped);
                            }
                        }
                    }
                    _ => { return Ok(DisplayWrap::Unwrapped); }
                }
            }
            return Ok(DisplayWrap::Unwrapped);
        }
        Ok(DisplayWrap::Unwrapped)
    }

    fn write_job(&mut self, attrs: &mut Vec<&'a str>) -> Result<DisplayWrap, ParsedError> {
        if let Some(name) = attrs.last() {
            if let Some(parent_name) = attrs.get(attrs.len().checked_sub(2).unwrap_or(0)) {
                if let Some(property_value) = self.last_loop_object.job.get_property(name) {
                    if let Some(name) = property_value.downcast_ref::<String>() {
                        self.buf.write(&name);
                    } else {
                        return Ok(DisplayWrap::Unwrapped);
                        //return Err(format!("\"{property}\" is an object on \"{name}.{property}\". You must use properties.").into());
                    }
                }
            }       
        }
        Ok(DisplayWrap::Unwrapped)
    }

    fn write_coordinates(&mut self, attrs: &mut Vec<&'a str>) -> Result<DisplayWrap, ParsedError> {
        if let Some(name) = attrs.last() {
            if let Some(parent_name) = attrs.get(attrs.len().checked_sub(2).unwrap_or(0)) {
                if let Some(property_value) = self.last_loop_object.coordinates.get_property(name) {
                    if let Some(name) = property_value.downcast_ref::<String>() {
                        self.buf.write(&name);
                    } else {
                        return Ok(DisplayWrap::Unwrapped);
                        //return Err(format!("\"{property}\" is an object on \"{name}.{property}\". You must use properties.").into());
                    }
                }
            }       
        }
        Ok(DisplayWrap::Unwrapped)
    }

    fn write_phone(&mut self, attrs: &mut Vec<&'a str>) -> Result<DisplayWrap, ParsedError> {
        Ok(DisplayWrap::Unwrapped)
    }

    fn dispatch(&mut self, val: &'a Expr<'a>, attrs: &mut Vec<&'a str>) -> Result<DisplayWrap, ParsedError> {
        if let Some(name) = attrs.last() {
            if let Some(parent_name) = attrs.get(attrs.len().checked_sub(2).unwrap_or(0)) {
                match *parent_name {
                    "people" => { return Ok(self.write_people(val, parent_name, &name, attrs)?); },
                    "job" => { return Ok(self.write_job(attrs)?); },
                    "phone" => { return Ok(self.write_phone(attrs)?); },
                    "coordinates" => { return Ok(self.write_coordinates(attrs)?); },
                    _ => {
                        let attrs_str = attrs.join(".");
                        return Err(format!("\"{name}\" in \"{attrs_str}\" doesn't exist.").into())
                    }
                }
            }
        }
        return Ok(DisplayWrap::Unwrapped);
    }

    fn write_expr(&mut self, val: &'a Expr<'a>, attrs: &mut Vec<&'a str>) -> Result<DisplayWrap, ParsedError> {
        match val {
            Expr::Attr(attr, name) => {
                match attr.borrow() {
                    Expr::Var(parent_name) => {
                        attrs.insert(0, *name);
                        attrs.insert(0, *parent_name);
                        return Ok(self.dispatch(val, attrs)?);
                    }
                    _val => {
                        attrs.insert(0, *name);
                        return Ok(self.write_expr(_val, attrs)?);
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

use crate::template::people::People;

use super::address::Coordinate;
use super::job::Job;
impl Buffer {
    fn new(indent: u8) -> Self {
        Self {
            buf: String::new(),
            indent,
            start: true,
            discard: false,
        }
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
}
