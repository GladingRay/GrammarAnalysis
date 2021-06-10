mod grammar;
use grammar::*;
use std::io::stdin;

fn main() {
    let mut gm = Grammar::new();
    gm.init_grammar("Grammar.rule");
    gm.print_grammar();
    let mut isf = ItemSetFamily::new(gm);
    isf.build_dfa();
    isf.print_dfa();
    isf.build_table();
    isf.print_table();
    let mut input_str = String::new();
    println!("输入待检测字符串");
    stdin().read_line(&mut input_str).expect("input error");
    let input_str = input_str.trim();
    isf.check_string(input_str.to_string());
}
