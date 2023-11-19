use clap::Parser;
use typst_syntax::parse;
use typst_geshihua::convert_markup;

use crate::cli::CliArguments;

mod cli;

fn main() {
    let CliArguments { input } = CliArguments::parse();
    let content = std::fs::read_to_string(input).unwrap();
    let root = parse(&content);
    // print!("{:#?}", root);
    let markup = root.cast().unwrap();
    let doc = convert_markup(markup);
    print!("{:#?}", doc);
    let res = doc.pretty(120).to_string();
    print!("{}", res);
}

