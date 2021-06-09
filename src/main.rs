mod grammar;
use grammar::*;

fn main() {
    let mut gm = Grammar::new();
    gm.init_grammar("Grammar.rule");
    println!("Hello, world!");
    gm.print_grammar();
}
