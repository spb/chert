use chert_derive::ChertStruct;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(index = 1)]
    expression: String,
}

#[derive(Clone, Debug, ChertStruct)]
struct Variables {
    _a: i64,
}

fn main() {
    let args = Arguments::parse();

    let tokens = chert::lex::lex(&args.expression).unwrap();
    let node = chert::parse::parse::<Variables>(tokens).unwrap();
    if let chert::parse::nodes::Node::Boolean(node) = node {
        println!("{node:?}");
        let engine = chert::compile::compile(Vec::from([(0, node)]));
        let results = engine.eval(&Variables { _a: 0 });
        println!("{results:?}");
    } else {
        panic!("expression must result in a boolean");
    }
}
