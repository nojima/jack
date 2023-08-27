mod ast;
mod eval;
mod lexer;
mod symbol;
mod token;
mod value;

use std::{
    fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub syntax);

#[derive(clap::Parser)]
#[command(name = "jack")]
#[command(author = "Yusuke Nojima")]
#[command(about = "A JSON Generation Language")]
struct Cli {
    filename: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.filename {
        Some(filename) => execute_file(&filename),
        None => repl(),
    }
}

fn execute_file(filename: &Path) -> anyhow::Result<()> {
    let source_code = fs::read_to_string(filename)?;
    let lexer = lexer::Lexer::new(&source_code);
    let parser = syntax::ExprParser::new();
    let node = parser.parse(lexer)?;
    let env = eval::Env::new();
    let value = eval::eval_expr(&env, &node)?;
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

fn repl() -> anyhow::Result<()> {
    let mut rl = rustyline::DefaultEditor::new()?;
    let env = eval::Env::new();

    loop {
        let line = rl.readline("expr> ")?;

        let lexer = lexer::Lexer::new(&line);
        let parser = syntax::ExprParser::new();
        let node = match parser.parse(lexer) {
            Ok(node) => node,
            Err(e) => {
                println!("ERROR: {e}");
                continue;
            }
        };
        //println!("AST = << {node:?} >>");

        let value = match eval::eval_expr(&env, &node) {
            Ok(v) => v,
            Err(e) => {
                println!("ERROR: {e}");
                continue;
            }
        };
        let j = match serde_json::to_string_pretty(&value) {
            Ok(j) => j,
            Err(e) => {
                println!("ERROR: {e}");
                continue;
            }
        };
        println!("=> {j}");
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
    verify("-1", "Neg(1.0)");
    verify("3.14", "3.14");
    verify("1e10", "10000000000.0");
    verify("1E10", "10000000000.0");

    verify("[]", "[]");
    verify("[null]", "[null]");
    verify("[1, 2]", "[1.0, 2.0]");
    verify("[true,]", "[true]");

    verify("{}", "{}");
    verify("{\"foo\": true}", "{\"foo\": true}");
    verify(
        "{\"aaa\": 1.0, \"bbb\": 2.0}",
        "{\"aaa\": 1.0, \"bbb\": 2.0}",
    );

    verify("\"hello\"", "\"hello\"")
}
