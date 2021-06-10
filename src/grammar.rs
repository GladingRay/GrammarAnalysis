use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

pub enum Symbol {
    VoidSymbol,
    EndSymbol,
    Terminate(char),
    NonTerminate(char)
}

pub enum TableItem {
    Goto(usize),
    ActionShift(usize),
    ActionReduction(usize),
    Accept,
    Error
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
    fn get_symbol_index(&self, production: usize, dot_pos: usize) -> usize {
        self.productions[production].tail[dot_pos]
    }
    pub fn get_char_from_index(&self, index: usize) -> char {
        self.symbol_table[index].to_char()
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
        self.symbol_table.push(Symbol::from('#'));
        
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
    pub fn print_item(&self, gm: &Grammar) {
        print!("{}->", gm.get_char_from_index(gm.productions[self.production].head));
        let mut index = 0;
        for num in &gm.productions[self.production].tail {
            if let Symbol::VoidSymbol = gm.symbol_table[*num] {
                print!("·");
                break;
            }
            else {
                if self.dot_pos == index {
                    print!("·");
                }
                print!("{}", gm.symbol_table[*num].to_char());
            }
            index = index + 1;
        }
        if self.dot_pos == gm.productions[self.production].tail.len() {
            print!("·");
        }
        println!();
    }
    pub fn is_eq(&self, other: &Item) -> bool {
        self.production == other.production && self.dot_pos == other.dot_pos
    }
    pub fn from(other: &Item) -> Item {
        Item {
            production: other.production,
            dot_pos:    other.dot_pos+1
        }
    }
    pub fn is_end(&self, gm: &Grammar) -> bool {
        if self.dot_pos == gm.productions[self.production].tail.len() {
            return true;
        }
        if let Symbol::VoidSymbol = gm.symbol_table[gm.get_symbol_index(self.production, self.dot_pos)] {
            true
        }
        else {
            false
        }
        
    }
    pub fn get_current_symbol(&self, gm: &Grammar) -> usize {
        gm.productions[self.production].tail[self.dot_pos]
    }
}

pub struct ItemSet {
    item_vec: Vec<Item>,
    transforms: HashMap<usize, usize>,
    analysis_table: HashMap<usize, TableItem>
}

impl ItemSet {
    pub fn new() -> ItemSet {
        ItemSet {
            item_vec: Vec::new(),
            transforms: HashMap::new(),
            analysis_table: HashMap::new()
        }
    }
    pub fn print_item_set(&self, gm: &Grammar) {
        for item in &self.item_vec {
            item.print_item(gm);
        }
    }
    pub fn print_transforms(&self, gm: &Grammar) {
        for (trans, dest) in &self.transforms {
            print!("|{}->{}|", gm.get_char_from_index(*trans), *dest);
        }
        println!();
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
    pub fn add_item(&mut self, item: Item) {
        self.item_vec.push(item);
    }
    pub fn make_closure(&mut self, gm: &Grammar) {
        let mut already_non_symb: HashSet<usize> = HashSet::new();
        let mut index = 0;
        while index < self.item_vec.len() {
            if self.item_vec[index].is_end(gm) {
                index = index + 1;
                continue;
            }
            let temp_symbol_index = gm.get_symbol_index(self.item_vec[index].production, self.item_vec[index].dot_pos);
            if let Symbol::NonTerminate(_s) = gm.symbol_table[temp_symbol_index] {
                if !already_non_symb.contains(&temp_symbol_index) {
                    already_non_symb.insert(temp_symbol_index);
                    let mut i = 0;
                    for prod in &gm.productions {
                        if prod.head == temp_symbol_index {
                            self.item_vec.push(Item::new(i));
                        }
                        i = i + 1;
                    }
                }
            }
            index = index + 1;
        }
    }
    pub fn get_move_map(&self, gm: &Grammar) ->HashMap<usize, ItemSet> {
        let mut move_map: HashMap<usize, ItemSet> = HashMap::new();
        for item in &self.item_vec {
            if item.is_end(gm) {
                continue;
            }
            let gsc = item.get_current_symbol(&gm);
            if !move_map.contains_key(&gsc) {
                let mut tis = ItemSet::new();
                tis.add_item(Item::from(item));
                move_map.insert(gsc, tis);
            }
            else {
                if let Some(item_set) = move_map.get_mut(&gsc) {
                    item_set.add_item(Item::from(item));
                }
            }
        }
        for (_trans, item_set) in &mut move_map {
            item_set.make_closure(gm);
        }
        move_map
    }
    fn is_exist_co(&self, gm: &Grammar) -> bool {
        let mut count_r = 0;
        for item in &self.item_vec {
            if item.is_end(gm) && gm.productions[item.production].head != 0 {
                count_r = count_r + 1;
            }
        }
        count_r > 1 || count_r == 1 && count_r < self.item_vec.len()
    }
    pub fn make_decision(&mut self, gm: &Grammar) -> bool {
        if self.is_exist_co(gm) {
            return false;
        }
        
        let mut s_index = 0;
        for s in &gm.symbol_table {
            if s.to_char() == '@' {
                s_index = s_index + 1;
                continue;
            }
            if self.transforms.len() == 0 {
                if let Symbol::NonTerminate(_n_t) = s {
                    self.analysis_table.insert(s_index, TableItem::Error);
                }
                else {
                    if self.item_vec[0].production != 0{
                        self.analysis_table.insert(s_index, TableItem::ActionReduction(self.item_vec[0].production));
                    }
                    else {
                        if s.to_char() == '#' {
                            self.analysis_table.insert(s_index, TableItem::Accept);
                        }
                        else {
                            self.analysis_table.insert(s_index, TableItem::Error);
                        }
                    }
                }
            }
            else {
                if let Symbol::NonTerminate(_nt) = s {
                    if self.transforms.contains_key(&s_index) {
                        let dest = if let Some(x) = self.transforms.get(&s_index) {
                            *x
                        }
                        else {
                            0
                        };
                        self.analysis_table.insert(s_index, TableItem::Goto(dest));
                    }
                    else {
                        self.analysis_table.insert(s_index, TableItem::Error);
                    }
                }
                else {
                    if self.transforms.contains_key(&s_index) {
                        let dest = if let Some(x) = self.transforms.get(&s_index) {
                            *x
                        }
                        else {
                            0
                        };
                        self.analysis_table.insert(s_index, TableItem::ActionShift(dest));
                    }
                    else {
                        self.analysis_table.insert(s_index, TableItem::Error);
                    }
                }
            }
            s_index = s_index + 1;
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
    pub fn is_exist(&self, item_set: &ItemSet) -> isize {
        let mut index: isize = 0;
        for i_s in &self.family {
            if i_s.is_eq(item_set) {
                return index;
            }
            index = index + 1;
        }  
        -1
    }
    pub fn build_dfa(&mut self) {
        let origin = Item::new(0);
        let mut item_set = ItemSet::new();
        item_set.add_item(origin);
        self.family.push(item_set);
        let mut index = 0;
        self.family[index].make_closure(&self.gm);
        while index < self.family.len() {
            
            let move_map = self.family[index].get_move_map(&self.gm);
            for (trans, t_set) in move_map {
                let set_index = self.is_exist(&t_set);
                if set_index == -1   {
                    self.family.push(t_set);
                    let new_index = self.family.len()-1;
                    self.family[index].transforms.insert(trans, new_index);
                }
                else {
                    self.family[index].transforms.insert(trans, set_index as usize);
                }
            }
            index = index + 1;
        }
    }
    pub fn build_table(&mut self) {
        for item_set in &mut self.family {
            if !item_set.make_decision(&self.gm) {
                println!("该文法非LR(0)文法。");
                break;
            }
        }
    }
    pub fn print_dfa(&self) {
        println!("ItemSetFamily:");
        let mut index = 0;
        for item_set in &self.family {
            println!("I{}:",index);
            item_set.print_item_set(&self.gm);
            index = index + 1;
        }
        println!("dfa:");
        index = 0;
        for item_set in &self.family {
            print!("{}:",index);
            item_set.print_transforms(&self.gm);
            index = index + 1;
        }
    }
    pub fn print_table(&self) {
        println!("analysis_table:");
        for s in &self.gm.symbol_table {
            if s.to_char() != '@' {
                print!("\t{}",s.to_char());
            }
            
        }
        println!();
        let mut index = 0;
        for item_set in &self.family {
            print!("{}:",index);
            let mut s_i = 0;
            for s in &self.gm.symbol_table {
                if s.to_char() != '@' {
                    print!("\t");
                    let x = if let Some(x1) = item_set.analysis_table.get(&s_i) {
                        x1
                    }
                    else {
                        &TableItem::Error
                    };
                    match x {
                        TableItem::Accept => print!("acc"),
                        TableItem::ActionReduction(n) => print!("r{}", n),
                        TableItem::ActionShift(n) => print!("s{}", n),
                        TableItem::Error => print!("err"),
                        TableItem::Goto(n) => print!("{}", n)
                    }
                }
                s_i = s_i + 1;
            }
            index = index + 1;
            println!();
        }
    }
    pub fn check_string(&self, s: String) {
        let mut symbol_stack: VecDeque<usize> = VecDeque::new();
        symbol_stack.push_back(self.gm.get_char_index('#'));
        let mut state_stack: VecDeque<usize> = VecDeque::new();
        state_stack.push_back(0);
        let s = s+"#";
        let s = s.as_bytes();
        let mut start_index = 0;
        println!("符号栈\t输入串\t状态栈\t动作");
        loop {
            for elem in &symbol_stack {
                print!("{}",self.gm.symbol_table[*elem].to_char());
            }
            print!("\t");
            let mut i = start_index;
            while i < s.len() {
                print!("{}", s[i] as char);
                i = i + 1;
            }
            print!("\t");
            for elem in &state_stack {
                print!("{}",elem);
            }
            print!("\t");
            let state_top = if let Some(x) = state_stack.back() {
                *x
            }
            else {0};
            let condition = if let Some(x) = self.family[state_top].analysis_table.get(&self.gm.get_char_index(s[start_index] as char)){
                x
            }
            else {
                &TableItem::Error
            };
            match condition {
                TableItem::Accept => {
                    print!("acc");
                    break;
                },
                TableItem::ActionReduction(n) => {
                    print!("r{}", n);
                    for _elem in &self.gm.productions[*n].tail {
                        symbol_stack.pop_back();
                        state_stack.pop_back();
                    }
                    let state_top = if let Some(x) = state_stack.back() {
                        *x
                    }
                    else {0};
                    let temp = if let Some(v) = self.family[state_top].analysis_table.get(&self.gm.productions[*n].head) {
                        v
                    }
                    else {
                        &TableItem::Error
                    };
                    if let TableItem::Goto(x) = temp {
                        state_stack.push_back(*x);
                        print!(", GOTO{}", x);
                    }
                    else {
                        print!("error");
                        break;
                    }
                    symbol_stack.push_back(self.gm.productions[*n].head);
                },
                TableItem::ActionShift(n) => {
                    print!("s{}", n);
                    symbol_stack.push_back(self.gm.get_char_index(s[start_index] as char));
                    start_index = start_index + 1;
                    state_stack.push_back(*n);

                },
                TableItem::Goto(_n) => {
                    
                },
                TableItem::Error => {
                    print!("error");
                    break;
                }
                
            }
            println!();
        }
        println!();
    }
}