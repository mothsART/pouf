use std::fs;

use clap::{Arg, Command};
use askama_parser::{Ast, Syntax};

use pouf::template::{generator::AstLevel, generator::Generator};

#[test]
fn test_rated_example() {
    let template_m = Command::new("MyApp").arg(Arg::new("out")).get_matches();
    let contents =
    fs::read_to_string("examples/templates/isbn.txt").expect("Should have been able to read the file");
    let ast = Ast::from_str(&contents, &Syntax::default()).unwrap();
    let mut g = Generator::new(&template_m);
    if let Err(_err) = g.handle(ast.nodes(), AstLevel::Top) {
        assert!(false);
    }
    else {
        assert!(true);
    }
}
