mod grammar;
use grammar::*;

fn main() {
    let mut gm = Grammar::new();
    gm.init_grammar("Grammar.rule");
    gm.print_grammar();
    let mut isf = ItemSetFamily::new(gm);
    isf.build_dfa();
    isf.print_dfa();
    isf.build_table();
    isf.print_table();
    isf.check_string(String::from("abbcde"));
}
