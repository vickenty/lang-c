use std::collections::{HashMap, HashSet};

use ast::*;
use span::Node;
use strings;

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum Symbol {
    Typename,
    Identifier,
}

pub struct Env<T: Name> {
    pub symbols: Vec<HashMap<T, Symbol>>,
    pub extensions_gnu: bool,
    pub extensions_clang: bool,
    pub reserved: HashSet<&'static str>,
}

impl<T: Name> Env<T> {
    pub fn with_core() -> Env<T> {
        let mut reserved = HashSet::default();
        reserved.extend(strings::RESERVED_C11.iter());
        Env {
            extensions_gnu: false,
            extensions_clang: false,
            symbols: vec![HashMap::default()],
            reserved: reserved,
        }
    }

    pub fn with_gnu() -> Env<T> {
        let mut symbols = HashMap::default();
        let mut reserved = HashSet::default();
        symbols.insert(T::get_from_str("__builtin_va_list"), Symbol::Typename);
        reserved.extend(strings::RESERVED_C11.iter());
        reserved.extend(strings::RESERVED_GNU.iter());
        Env {
            extensions_gnu: true,
            extensions_clang: false,
            symbols: vec![symbols],
            reserved: reserved,
        }
    }

    pub fn with_clang() -> Env<T> {
        let mut symbols = HashMap::default();
        let mut reserved = HashSet::default();
        symbols.insert(T::get_from_str("__builtin_va_list"), Symbol::Typename);
        reserved.extend(strings::RESERVED_C11.iter());
        reserved.extend(strings::RESERVED_GNU.iter());
        reserved.extend(strings::RESERVED_CLANG.iter());
        Env {
            extensions_gnu: true,
            extensions_clang: true,
            symbols: vec![symbols],
            reserved: reserved,
        }
    }

    pub fn enter_scope(&mut self) {
        self.symbols.push(HashMap::new());
    }

    pub fn leave_scope(&mut self) {
        self.symbols.pop().expect("more scope pops than pushes");
    }

    pub fn is_typename(&self, ident: &T) -> bool {
        for scope in self.symbols.iter().rev() {
            if let Some(symbol) = scope.get(ident) {
                return *symbol == Symbol::Typename;
            }
        }
        false
    }

    pub fn handle_declarator(&mut self, d: &Node<Declarator<T>>, sym: Symbol) {
        if let Some(name) = find_declarator_name(&d.node.kind.node) {
            self.add_interned_symbol(&name, sym)
        }
    }

    #[cfg(test)]
    pub fn add_symbol(&mut self, s: &str, symbol: Symbol) {
        let scope = self
            .symbols
            .last_mut()
            .expect("at least one scope should be always present");
        scope.insert(T::get_from_str(s), symbol);
    }

    pub fn add_interned_symbol(&mut self, s: &T, symbol: Symbol) {
        let scope = self
            .symbols
            .last_mut()
            .expect("at least one scope should be always present");
        scope.insert(s.clone(), symbol);
    }

    #[cfg(test)]
    pub fn add_typename(&mut self, s: &str) {
        self.add_symbol(s, Symbol::Typename)
    }
}

fn find_declarator_name<T: Name>(d: &DeclaratorKind<T>) -> Option<&T> {
    match d {
        &DeclaratorKind::Abstract => None,
        &DeclaratorKind::Identifier(ref i) => Some(&i.node.name),
        &DeclaratorKind::Declarator(ref d) => find_declarator_name(&d.node.kind.node),
    }
}
