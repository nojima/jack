mod ast;
mod lexer;
mod token;

use lalrpop_util::lalrpop_mod;
use std::io::{self, Write};

lalrpop_mod!(pub syntax);

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        print!("expr> ");
        io::stdout().flush()?;

        buffer.clear();
        let n = stdin.read_line(&mut buffer)?;
        if n == 0 {
            // EOF
            return Ok(());
        }

        let lexer = lexer::Lexer::new(&buffer);
        let parser = syntax::ExprParser::new();
        println!("{:?}", parser.parse(lexer));
        println!();
    }
}

#[test]
fn parse_test() {
    let verify = |source: &str, expected: &str| {
        let parser = syntax::ExprParser::new();
        let lexer = lexer::Lexer::new(source);
        let maybe_ast = parser.parse(lexer);
        let actual = maybe_ast.map(|ast| format!("{:?}", ast));
        assert_eq!(actual, Ok(expected.to_owned()));
    };

    verify("null", "null");
    verify("true", "true");
    verify("false", "false");

    verify("0", "0.0");
    verify("1", "1.0");
    verify("10", "10.0");
    verify("-1", "-1.0");
    verify("3.14", "3.14");
    verify("-3.14", "-3.14");
    verify("1e10", "10000000000.0");
    verify("1E10", "10000000000.0");
    verify("-1E10", "-10000000000.0");

    verify("[]", "[]");
    verify("[null]", "[null]");
    verify("[1, 2]", "[1.0, 2.0]");
    verify("[true,]", "[true]");

    verify("{}", "{}");
    verify("{foo: true}", "{\"foo\": true}");
    verify("{aaa: 1.0, bbb: 2.0}", "{\"aaa\": 1.0, \"bbb\": 2.0}");

    //verify("\"hello\"", "\"hello\"")
}
