mod ast;
mod eval;
mod lexer;
mod symbol;
mod token;
mod value;

use std::fs;
use std::io::{stdin, Read};
use std::path::{Path, PathBuf};

use clap::Parser;
use lalrpop_util::{lalrpop_mod, ParseError};
use rustyline::DefaultEditor;

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
    let source_code = if filename.to_string_lossy() == "-" {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        fs::read_to_string(filename)?
    };
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
        let node = repl_read_and_parse(&mut rl)?;
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

fn repl_read_and_parse(rl: &mut DefaultEditor) -> anyhow::Result<ast::Expr> {
    let mut prompt = "expr> ";
    let mut line = String::new();
    loop {
        line.push_str(&rl.readline(&prompt)?);
        let lexer = lexer::Lexer::new(&line);
        let parser = syntax::ExprParser::new();
        let expr = match parser.parse(lexer) {
            Ok(node) => node,
            Err(e) => match e {
                ParseError::UnrecognizedEof { .. } => {
                    line.push('\n');
                    prompt = "....| ";
                    continue;
                }
                _ => Err(e)?,
            },
        };
        return Ok(expr);
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
