use std::collections::HashMap;
use std::fs;

enum Symbol {
    VoidSymbol,
    EndSymbol,
    Terminate(char),
    NonTerminate(char)
}

impl Symbol {
    fn from(c: char) -> Symbol {
        if c <= 'Z' && c >= 'A' {
            Symbol::NonTerminate(c)
        }
        else if c == '$' {
            Symbol::VoidSymbol
        }
        else if c == '#' {
            Symbol::EndSymbol
        }
        else {
            Symbol::Terminate(c)
        }
    }
    fn to_char(&self) -> char {
        match self {
            Symbol::VoidSymbol => '$',
            Symbol::EndSymbol => '#',
            Symbol::Terminate(c) => *c,
            Symbol::NonTerminate(c) => *c
        }
    }
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
    fn symbol_is_exist(&self, c: char) -> bool {
        for elem in &self.symbol_table {
            if c == elem.to_char() {
                return true;
            }
        }
        false
    }
    fn get_char_index(&self, c: char) -> usize {
        let mut index: usize = 0;
        for elem in &self.symbol_table {
            if c == elem.to_char() {
                return index;
            }
            index = index + 1;
        };
        0
    }
    pub fn init_grammar(&mut self, filename: &str) {
        let file_content = match fs::read_to_string(filename) {
            Ok(f_c) => f_c,
            _ => String::from("there is a error occur when init.")
        };
        self.symbol_table.push(Symbol::from('@'));
        self.productions.push(Production::new(0, vec![1]));
        for s in file_content.lines() {
            let head;
            let mut tail: Vec<usize> = Vec::new();
            let head_tail: Vec<&str> = s.split(' ').collect();
            let head_char = head_tail[0].as_bytes()[0] as char ;
            if !self.symbol_is_exist(head_char) {
                self.symbol_table.push(Symbol::from(head_char));
            }
            head = self.get_char_index(head_char);

            for c in head_tail[1].chars() {
                
                if !self.symbol_is_exist(c) {
                    self.symbol_table.push(Symbol::from(c));
                }
                tail.push(self.get_char_index(c));
            }

            self.productions.push(Production::new(head, tail));
            println!("{}", s);
        }
    }
    pub fn print_grammar(&self) {
        print!("symbol table:");
        for elem in &self.symbol_table {
            print!("{} ", elem.to_char());
        }
        println!();
        println!("productions:");
        for elem in &self.productions {
            print!("{}->", self.symbol_table[elem.head].to_char());
            for i in &elem.tail {
                print!("{}", self.symbol_table[*i].to_char());
            }
            println!();
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
    pub fn is_eq(&self, other: &Item) -> bool {
        self.production == other.production && self.dot_pos == other.dot_pos
    }
}

pub struct ItemSet {
    item_vec: Vec<Item>,
    transforms: HashMap<usize, usize>
}

impl ItemSet {
    pub fn new() -> ItemSet {
        ItemSet {
            item_vec: Vec::new(),
            transforms: HashMap::new()
        }
    }
    pub fn is_item_in(&self, item: &Item) -> bool {
        for i in &self.item_vec {
            if i.is_eq(item) {
                return true;
            }
        }
        false
    }
    pub fn is_eq(&self, other: &ItemSet) -> bool {
        if self.item_vec.len() != other.item_vec.len() {
            return false;
        }
        for i in &self.item_vec {
            if !other.is_item_in(i) {
                return false;
            }
        }
        true
    }
}

pub struct ItemSetFamily {
    family: Vec<ItemSet>,
    gm  : Grammar
}

impl ItemSetFamily {
    pub fn new(gm: Grammar) -> ItemSetFamily {
        ItemSetFamily {
            family: Vec::new(),
            gm
        }
    }
}