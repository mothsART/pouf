use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt;

use clap::ArgMatches;

use askama_parser::node::{Lit, Loop, Target, Whitespace};
use askama_parser::{Expr, Node};

use super::address::Address;
use super::automotive::Automotive;
use super::barecode::BareCode;
use super::color::Color;
use super::coordinates::Coordinate;
use super::currency::Currency;
use super::filesystem::FileSystem;
use super::http::Http;
use super::internet::Internet;
use super::job::Job;
use super::people::People;
use super::phone::Phone;
use super::timezone::Timezone;

use super::template_trait::TemplateObject;

#[derive(Clone, Copy, PartialEq)]
pub enum AstLevel {
    Top,
    Nested,
}

#[derive(Clone, Copy)]
enum DisplayWrap {
    Unwrapped,
}

#[derive(Debug, Clone)]
pub struct ParsedError {
    pub msg: Cow<'static, str>,
}

impl ParsedError {
    fn new<S: Into<Cow<'static, str>>>(s: S) -> Self {
        Self { msg: s.into() }
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) enum WhitespaceHandling {
    /// The default behaviour. It will leave the whitespace characters "as is".
    #[default]
    Preserve,
}

impl From<WhitespaceHandling> for Whitespace {
    fn from(ws: WhitespaceHandling) -> Self {
        match ws {
            WhitespaceHandling::Preserve => Whitespace::Preserve,
        }
    }
}

struct LoopObject {
    address: Address,
    automotive: Automotive,
    barecode: BareCode,
    color: Color,
    coordinates: Coordinate,
    currency: Currency,
    filesystem: FileSystem,
    http: Http,
    internet: Internet,
    job: Job,
    people: People,
    phone: Phone,
    timezone: Timezone,
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
    pub fn new(template_m: &ArgMatches) -> Generator<'_> {
        Generator {
            next_ws: None,
            skip_ws: WhitespaceHandling::Preserve,
            buf: Buffer::new(0),
            last_loop_var: None,
            last_loop_object: LoopObject {
                address: Address::create(template_m),
                automotive: Automotive::create(template_m),
                barecode: BareCode::create(template_m),
                color: Color::create(template_m),
                coordinates: Coordinate::create(template_m),
                currency: Currency::create(template_m),
                filesystem: FileSystem::create(template_m),
                http: Http::create(template_m),
                internet: Internet::create(template_m),
                job: Job::create(template_m),
                people: People::create(template_m),
                phone: Phone::create(template_m),
                timezone: Timezone::create(template_m),
            },
            template_m,
        }
    }

    pub fn handle(&mut self, nodes: &'a [Node<'_>], level: AstLevel) -> Result<usize, ParsedError> {
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
                Node::Loop(ref loop_block) => {
                    self.write_loop(loop_block)?;
                }
                _ => {}
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
        let Lit { lws, val, rws } = *lit;
        if !lws.is_empty() {
            match self.skip_ws {
                WhitespaceHandling::Preserve => {
                    self.buf.write(lws);
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

    fn write_loop(&mut self, loop_block: &'a Loop<'_>) -> Result<usize, ParsedError> {
        if let Target::Name(val) = loop_block.var {
            self.last_loop_var = Some(val);
        }

        if let Expr::Call(r#box, args) = &loop_block.iter {
            if let Expr::Var(value) = r#box.borrow() {
                if *value == "peoples" && !args.is_empty() {
                    if let askama_parser::Expr::NumLit(val) = args[0] {
                        if let Ok(len_of_element) = val.parse::<i32>() {
                            for _n in 0..len_of_element {
                                //self.last_loop_object.people = People::create(self.template_m);
                                self.handle(&loop_block.body, AstLevel::Nested)?;
                            }
                        }
                    }
                }
                if *value == "barecodes" && !args.is_empty() {
                    if let askama_parser::Expr::NumLit(val) = args[0] {
                        if let Ok(len_of_element) = val.parse::<i32>() {
                            for _n in 0..len_of_element {
                                self.handle(&loop_block.body, AstLevel::Nested)?;
                            }
                        }
                    }
                }
            }
        }

        Ok(0)
    }

    fn dispatch(&mut self, attrs: &mut [&'a str]) -> Result<DisplayWrap, ParsedError> {
        if let Some(name) = attrs.last() {
            if let Some(parent_name) = attrs.get(attrs.len().saturating_sub(2)) {
                match *parent_name {
                    "address" => {
                        return write_object(&self.last_loop_object.address, attrs, &mut self.buf);
                    }
                    "automotive" => {
                        return write_object(
                            &self.last_loop_object.automotive,
                            attrs,
                            &mut self.buf,
                        );
                    }
                    "barecode" => {
                        return write_object(&self.last_loop_object.barecode, attrs, &mut self.buf);
                    }
                    "color" => {
                        return write_object(&self.last_loop_object.color, attrs, &mut self.buf);
                    }
                    "coordinates" => {
                        return write_object(
                            &self.last_loop_object.coordinates,
                            attrs,
                            &mut self.buf,
                        );
                    }
                    "currency" => {
                        return write_object(&self.last_loop_object.currency, attrs, &mut self.buf);
                    }
                    "filesystem" => {
                        return write_object(
                            &self.last_loop_object.filesystem,
                            attrs,
                            &mut self.buf,
                        );
                    }
                    "http" => {
                        return write_object(&self.last_loop_object.http, attrs, &mut self.buf);
                    }
                    "internet" => {
                        return write_object(&self.last_loop_object.internet, attrs, &mut self.buf);
                    }
                    "job" => {
                        return write_object(&self.last_loop_object.job, attrs, &mut self.buf);
                    }
                    "people" => {
                        return write_object(&self.last_loop_object.people, attrs, &mut self.buf);
                    }
                    "phone" => {
                        return write_object(&self.last_loop_object.phone, attrs, &mut self.buf);
                    }
                    "timezone" => {
                        return write_object(&self.last_loop_object.timezone, attrs, &mut self.buf);
                    }
                    _ => {
                        let attrs_str = attrs.join(".");
                        return Err(format!("\"{name}\" in \"{attrs_str}\" doesn't exist.").into());
                    }
                }
            }
        }
        Ok(DisplayWrap::Unwrapped)
    }

    fn write_expr(
        &mut self,
        val: &'a Expr<'a>,
        attrs: &mut Vec<&'a str>,
    ) -> Result<DisplayWrap, ParsedError> {
        match val {
            Expr::Attr(attr, name) => match attr.borrow() {
                Expr::Var(parent_name) => {
                    attrs.insert(0, *name);
                    attrs.insert(0, *parent_name);
                    Ok(self.dispatch(attrs)?)
                }
                _val => {
                    attrs.insert(0, *name);
                    self.write_expr(_val, attrs)
                }
            },
            _ => Ok(DisplayWrap::Unwrapped),
        }
    }
}

fn write_object<T: TemplateObject>(
    object: &T,
    attrs: &mut [&str],
    buf: &mut Buffer,
) -> Result<DisplayWrap, ParsedError> {
    if let Some(name) = attrs.last() {
        if let Some(property_value) = object.get_property(name) {
            if let Some(value) = property_value.downcast_ref::<String>() {
                buf.write(value);
            } else {
                return Ok(DisplayWrap::Unwrapped);
            }
        }
    }
    Ok(DisplayWrap::Unwrapped)
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
