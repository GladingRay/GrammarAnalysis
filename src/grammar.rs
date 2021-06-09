using std::collections::HashMap;


enum Symbol {
    VoidSymbol,
    Terminate(char),
    NonTerminate(char)
}

pub struct Production {
    head : usize,
    tail : Vec<usize> 
}

impl Production {
    pub fn new(head: usize, tail: Vec<usize>) -> Production {
        Production {
            head,
            tail
        }
    }
}

pub struct Grammar {
    symbol_table: Vec<Symbol>,
    productions : Vec<Production>
}

impl Grammar {
    pub fn new() -> Grammar {
        Grammar {
            symbol_table: Vec::new(),
            productions: Vec::new()
        }
    }
}

pub struct Item {
    production : usize,
    dot_pos    : usize
}

impl Item {
    pub fn new(production: usize) -> Item {
        Item {
            production,
            dot_pos: 0
        }
    }
}

pub struct ItemSet {
    item_vec: Vec<Item>,
    transforms: HashMap<usize, usize>
}

pub struct ItemSetFamily {
    family: Vec<ItemSet>
}