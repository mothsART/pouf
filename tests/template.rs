use std::fs;

use askama_parser::{Ast, Syntax};
use clap::{Arg, Command};

use pouf::template::generator::{AstLevel, Generator, ParsedError};

fn sample_to_parse(path: &str) -> Result<usize, ParsedError> {
    let template_m = Command::new("MyApp").arg(Arg::new("out")).get_matches();
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let ast = Ast::from_str(&contents, &Syntax::default()).unwrap();
    let mut g = Generator::new(&template_m);
    return g.handle(ast.nodes(), AstLevel::Top);
}

#[test]
fn test_rated_example_isbn() {
    let r = sample_to_parse("examples/templates/isbn.txt");
    if let Err(_err) = r {
        assert!(false);
    } else {
        assert!(true);
    }
}

#[test]
fn test_rated_example_people() {
    let r = sample_to_parse("examples/templates/people.json");
    if let Err(_err) = r {
        assert!(false);
    } else {
        assert!(true);
    }
}

#[test]
fn test_rated_example_misc() {
    let r = sample_to_parse("examples/templates/misc.txt");
    if let Err(_err) = r {
        assert!(false);
    } else {
        assert!(true);
    }
}

#[test]
fn test_error_example_people() {
    let r = sample_to_parse("tests/errors/people_one.json");
    if let Err(err) = r {
        assert!(true);
        assert_eq!("\"ok\" in \"people.g.ok\" doesn't exist.", err.to_string());
    } else {
        assert!(false);
    }
}
