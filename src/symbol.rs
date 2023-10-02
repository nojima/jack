use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Mutex;

use compact_str::CompactString;
use once_cell::sync::Lazy;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Symbol(u32);

pub struct SymbolTable {
    next_symbol: u32,
    symbol2string: HashMap<Symbol, CompactString>,
    string2symbol: HashMap<CompactString, Symbol>,
}

impl SymbolTable {
    fn new() -> Self {
        Self {
            next_symbol: 1,
            symbol2string: HashMap::new(),
            string2symbol: HashMap::new(),
        }
    }

    fn intern(&mut self, s: CompactString) -> Symbol {
        match self.string2symbol.entry(s) {
            Occupied(entry) => *entry.get(),
            Vacant(entry) => {
                let symbol = Symbol(self.next_symbol);
                self.next_symbol += 1;
                let s = entry.key().clone();
                entry.insert(symbol);
                self.symbol2string.insert(symbol, s);
                symbol
            }
        }
    }

    fn to_str(&self, symbol: Symbol) -> &str {
        self.symbol2string.get(&symbol).unwrap()
    }
}

static SYMBOL_TABLE: Lazy<Mutex<SymbolTable>> = Lazy::new(|| Mutex::new(SymbolTable::new()));

impl From<CompactString> for Symbol {
    fn from(s: CompactString) -> Self {
        SYMBOL_TABLE.lock().unwrap().intern(s)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        SYMBOL_TABLE.lock().unwrap().to_str(*self).fmt(f)
    }
}
