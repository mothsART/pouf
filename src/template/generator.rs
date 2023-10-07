use std::collections::HashMap;
use std::mem;

use askama_parser::{Node, Expr};
use askama_parser::node::{Lit, Let, Loop, Comment, Whitespace, Ws};

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
    // buffer for writable
    buf_writable: Vec<Writable<'a>>,
    whitespace: WhitespaceHandling,
}

impl<'a> Generator<'a> {
    pub fn new() -> Generator<'a> {
        Generator {
            next_ws: None,
            skip_ws: WhitespaceHandling::Preserve,
            buf_writable: vec![],
            whitespace: WhitespaceHandling::Preserve,
        }
    }

    pub fn handle(
        &mut self,
        nodes: &'a [Node<'_>],
    ) {
        let mut buf = Buffer::new(0);
        for n in nodes {
            match *n {
                Node::Lit(ref lit) => {
                    self.visit_lit(lit);
                }
                Node::Comment(ref comment) => {
                    self.write_comment(comment);
                }
                Node::Expr(ws, ref val) => {
                    self.write_expr(ws, val);
                }
                Node::Let(ref l) => {
                    self.write_let(&mut buf, l);
                }
                Node::If(ref i) => {
                    //size_hint += self.write_if(ctx, buf, i)?;
                }
                Node::Match(ref m) => {
                    //size_hint += self.write_match(ctx, buf, m)?;
                }
                Node::Loop(ref loop_block) => {
                    self.write_loop(&mut buf, loop_block);
                }
                Node::BlockDef(ref b) => {
                    //size_hint += self.write_block(buf, Some(b.name), Ws(b.ws1.0, b.ws2.1))?;
                }
                Node::Include(ref i) => {
                    //size_hint += self.handle_include(ctx, buf, i)?;
                }
                Node::Call(ref call) => {
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
                    /*
                    self.handle_ws(raw.ws1);
                    self.visit_lit(&raw.lit);
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
                    /*
                    self.handle_ws(ws);
                    self.write_buf_writable(buf)?;
                    buf.writeln("break;")?;
                    */
                }
                Node::Continue(ws) => {
                    /*
                    self.handle_ws(ws);
                    self.write_buf_writable(buf)?;
                    buf.writeln("continue;")?;
                    */
                }
            }
        }
        
        self.write_buf_writable(&mut buf);
        println!("{}", &buf.buf);
    }

    pub fn render(self) {
        println!("{:?}", self.buf_writable);
    }

    fn write_loop(
        &mut self,
        buf: &mut Buffer,
        loop_block: &'a Loop<'_>,
    ) {
        self.handle_ws(loop_block.ws1);
        //self.locals.push();

        let expr_code = self.visit_expr_root(&loop_block.iter);

        /*
        let flushed = self.write_buf_writable(buf)?;
        buf.writeln("{")?;
        buf.writeln("let mut _did_loop = false;")?;
        match loop_block.iter {
            Expr::Range(_, _, _) => buf.writeln(&format!("let _iter = {expr_code};")),
            Expr::Array(..) => buf.writeln(&format!("let _iter = {expr_code}.iter();")),
            // If `iter` is a call then we assume it's something that returns
            // an iterator. If not then the user can explicitly add the needed
            // call without issues.
            Expr::Call(..) | Expr::Index(..) => {
                buf.writeln(&format!("let _iter = ({expr_code}).into_iter();"))
            }
            // If accessing `self` then it most likely needs to be
            // borrowed, to prevent an attempt of moving.
            _ if expr_code.starts_with("self.") => {
                buf.writeln(&format!("let _iter = (&{expr_code}).into_iter();"))
            }
            // If accessing a field then it most likely needs to be
            // borrowed, to prevent an attempt of moving.
            Expr::Attr(..) => buf.writeln(&format!("let _iter = (&{expr_code}).into_iter();")),
            // Otherwise, we borrow `iter` assuming that it implements `IntoIterator`.
            _ => buf.writeln(&format!("let _iter = ({expr_code}).into_iter();")),
        }?;
        if let Some(cond) = &loop_block.cond {
            self.locals.push();
            buf.write("let _iter = _iter.filter(|");
            self.visit_target(buf, true, true, &loop_block.var);
            buf.write("| -> bool {");
            self.visit_expr(buf, cond)?;
            buf.writeln("});")?;
            self.locals.pop();
        }

        self.locals.push();
        buf.write("for (");
        self.visit_target(buf, true, true, &loop_block.var);
        buf.writeln(", _loop_item) in ::askama::helpers::TemplateLoop::new(_iter) {")?;

        buf.writeln("_did_loop = true;")?;
        let mut size_hint1 = self.handle(ctx, &loop_block.body, buf, AstLevel::Nested)?;
        self.handle_ws(loop_block.ws2);
        size_hint1 += self.write_buf_writable(buf)?;
        self.locals.pop();
        buf.writeln("}")?;

        buf.writeln("if !_did_loop {")?;
        self.locals.push();
        let mut size_hint2 = self.handle(ctx, &loop_block.else_nodes, buf, AstLevel::Nested)?;
        self.handle_ws(loop_block.ws3);
        size_hint2 += self.write_buf_writable(buf)?;
        self.locals.pop();
        buf.writeln("}")?;

        buf.writeln("}")?;

        Ok(flushed + ((size_hint1 * 3) + size_hint2) / 2)
        */
    }

    fn visit_expr_root(&mut self, expr: &Expr<'_>) {
        let mut buf = Buffer::new(0);
        self.visit_expr(&mut buf, expr);
    }

    fn visit_expr(
        &mut self,
        buf: &mut Buffer,
        expr: &Expr<'_>,
    ) {
        match *expr {
            Expr::BoolLit(s) => self.visit_bool_lit(buf, s),
            Expr::NumLit(s) => println!("ggfhfhfjh"), //self.visit_num_lit(buf, s),
            /*
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
            */
            Expr::Call(ref obj, ref args) => self.visit_call(buf, obj, args),
            /*
            Expr::RustMacro(ref path, args) => self.visit_rust_macro(buf, path, args),
            Expr::Try(ref expr) => self.visit_try(buf, expr.as_ref())?,
            Expr::Tuple(ref exprs) => self.visit_tuple(buf, exprs)?,
            */
            _ => { println!("pfff {:?}", expr); }
        }
    }

    fn visit_call(
        &mut self,
        buf: &mut Buffer,
        left: &Expr<'_>,
        args: &[Expr<'_>],
    ) {
        match left {
            Expr::Attr(left, method) if **left == Expr::Var("loop") => match *method {
                "cycle" => match args {
                    [arg] => {
                        /*
                        if matches!(arg, Expr::Array(arr) if arr.is_empty()) {
                            return Err("loop.cycle(…) cannot use an empty array".into());
                        }
                        buf.write("({");
                        buf.write("let _cycle = &(");
                        self.visit_expr(buf, arg)?;
                        buf.writeln(");")?;
                        buf.writeln("let _len = _cycle.len();")?;
                        buf.writeln("if _len == 0 {")?;
                        buf.writeln("return ::core::result::Result::Err(::askama::Error::Fmt(::core::fmt::Error));")?;
                        buf.writeln("}")?;
                        buf.writeln("_cycle[_loop_item.index % _len]")?;
                        buf.writeln("})")?;
                        */
                    }
                    _ => {}//return Err("loop.cycle(…) expects exactly one argument".into()),
                },
                s => return,
            },
            left => {
                /*
                match left {
                    Expr::Var(name) => match self.locals.resolve(name) {
                        Some(resolved) => buf.write(&resolved),
                        None => buf.write(&format!("(&self.{})", normalize_identifier(name))),
                    },
                    left => {
                        self.visit_expr(buf, left);
                    }
                }
                */
                buf.write("(");
                //self._visit_args(buf, args);
                buf.write(")");
            }
        }
        //Ok(DisplayWrap::Unwrapped)
    }

    fn visit_bool_lit(&mut self, buf: &mut Buffer, s: &str) {
        buf.write(s);
    }

    fn write_expr(&mut self, ws: Ws, s: &'a Expr<'a>) {
        self.handle_ws(ws);
        self.buf_writable.push(Writable::Expr(s));
    }

    fn write_let(&mut self, buf: &mut Buffer, l: &'a Let<'_>) {
        self.handle_ws(l.ws);

        let Some(val) = &l.val else {
            self.write_buf_writable(buf);
            buf.write("let ");
            //self.visit_target(buf, false, true, &l.var);
            buf.writeln(";");
            return;
        };

        /*
        let mut expr_buf = Buffer::new(0);
        self.visit_expr(&mut expr_buf, val)?;

        let shadowed = self.is_shadowing_variable(&l.var)?;
        if shadowed {
            // Need to flush the buffer if the variable is being shadowed,
            // to ensure the old variable is used.
            self.write_buf_writable(buf)?;
        }
        if shadowed
            || !matches!(l.var, Target::Name(_))
            || matches!(&l.var, Target::Name(name) if self.locals.get(name).is_none())
        {
            buf.write("let ");
        }

        self.visit_target(buf, true, true, &l.var);
        buf.writeln(&format!(" = {};", &expr_buf.buf))
        */
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
        self.next_ws = None;
    }

    // Sets `skip_ws` to match the suffix whitespace suppressor from the given
    // argument, to determine whether to suppress leading whitespace from the
    // next literal.
    fn prepare_ws(&mut self, ws: Ws) {
        self.skip_ws = self.should_trim_ws(ws.1);
    }

    fn write_buf_writable(&mut self, buf: &mut Buffer) {
        if self.buf_writable.is_empty() {
            return;
        }

        if self
            .buf_writable
            .iter()
            .all(|w| matches!(w, Writable::Lit(_)))
        {
            let mut buf_lit = Buffer::new(0);
            for s in mem::take(&mut self.buf_writable) {
                if let Writable::Lit(s) = s {
                    buf_lit.write(s);
                };
            }
            let _ = buf.writeln(&format!("{}", &buf_lit.buf));
            return;
        }

        let mut size_hint = 0;
        let mut buf_format = Buffer::new(0);
        let mut buf_expr = Buffer::new(buf.indent + 1);
        println!("hehe");
        /*
        let mut expr_cache = HashMap::with_capacity(self.buf_writable.len());
        for s in mem::take(&mut self.buf_writable) {
            match s {
                Writable::Lit(s) => {
                    buf_format.write(&s.replace('{', "{{").replace('}', "}}"));
                    size_hint += s.len();
                }
                Writable::Expr(s) => {
                    use self::DisplayWrap::*;
                    let mut expr_buf = Buffer::new(0);
                    let wrapped = self.visit_expr(&mut expr_buf, s)?;
                    let expression = match wrapped {
                        Wrapped => expr_buf.buf,
                        Unwrapped => format!(
                            "::askama::MarkupDisplay::new_unsafe(&({}), {})",
                            expr_buf.buf, self.input.escaper
                        ),
                    };

                    let id = match expr_cache.entry(expression.clone()) {
                        Entry::Occupied(e) if is_cacheable(s) => *e.get(),
                        e => {
                            let id = self.named;
                            self.named += 1;

                            buf_expr.write(&format!("expr{id} = "));
                            buf_expr.write("&");
                            buf_expr.write(&expression);
                            buf_expr.writeln(",")?;

                            if let Entry::Vacant(e) = e {
                                e.insert(id);
                            }

                            id
                        }
                    };

                    buf_format.write(&format!("{{expr{id}}}"));
                    size_hint += 3;
                }
            }
        }

        buf.writeln("::std::write!(")?;
        buf.indent();
        buf.writeln("writer,")?;
        buf.writeln(&format!("{:#?},", &buf_format.buf))?;
        buf.writeln(buf_expr.buf.trim())?;
        buf.dedent()?;
        buf.writeln(")?;")?;
        Ok(size_hint)
        
        */
        println!("cool");
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
                WhitespaceHandling::Preserve => self.buf_writable.push(Writable::Lit(lws)),
                WhitespaceHandling::Minimize => {
                    self.buf_writable
                        .push(Writable::Lit(match lws.contains('\n') {
                            true => "\n",
                            false => " ",
                        }));
                }
            }
        }

        if !val.is_empty() {
            self.skip_ws = WhitespaceHandling::Preserve;
            self.buf_writable.push(Writable::Lit(val));
        }

        if !rws.is_empty() {
            self.next_ws = Some(rws);
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
