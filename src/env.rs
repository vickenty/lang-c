use std::collections::{HashMap, HashSet};

use ast::*;
use interner::Interner;
use span::Node;
use strings;

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum Symbol {
    Typename,
    Identifier,
}

pub struct Env<'a, T: 'a + Interner> {
    pub symbols: Vec<HashMap<T::Interned, Symbol>>,
    pub interner: &'a mut T,
    pub extensions_gnu: bool,
    pub extensions_clang: bool,
    pub reserved: HashSet<&'static str>,
}

impl<'a, T: Interner> Env<'a, T> {
    pub fn with_core(interner: &'a mut T) -> Env<'a, T> {
        let mut reserved = HashSet::default();
        reserved.extend(strings::RESERVED_C11.iter());
        Env {
            extensions_gnu: false,
            extensions_clang: false,
            symbols: vec![HashMap::default()],
            interner: interner,
            reserved: reserved,
        }
    }

    pub fn with_gnu(interner: &'a mut T) -> Env<'a, T> {
        let mut symbols = HashMap::default();
        let mut reserved = HashSet::default();
        symbols.insert(interner.intern_str("__builtin_va_list"), Symbol::Typename);
        reserved.extend(strings::RESERVED_C11.iter());
        reserved.extend(strings::RESERVED_GNU.iter());
        Env {
            extensions_gnu: true,
            extensions_clang: false,
            symbols: vec![symbols],
            interner: interner,
            reserved: reserved,
        }
    }

    pub fn with_clang(interner: &'a mut T) -> Env<'a, T> {
        let mut symbols = HashMap::default();
        let mut reserved = HashSet::default();
        symbols.insert(interner.intern_str("__builtin_va_list"), Symbol::Typename);
        reserved.extend(strings::RESERVED_C11.iter());
        reserved.extend(strings::RESERVED_GNU.iter());
        reserved.extend(strings::RESERVED_CLANG.iter());
        Env {
            extensions_gnu: true,
            extensions_clang: true,
            symbols: vec![symbols],
            interner: interner,
            reserved: reserved,
        }
    }

    pub fn enter_scope(&mut self) {
        self.symbols.push(HashMap::new());
    }

    pub fn leave_scope(&mut self) {
        self.symbols.pop().expect("more scope pops than pushes");
    }

    pub fn is_typename(&self, ident: &T::Interned) -> bool {
        for scope in self.symbols.iter().rev() {
            if let Some(symbol) = scope.get(ident) {
                return *symbol == Symbol::Typename;
            }
        }
        false
    }

    pub fn handle_declarator(&mut self, d: &Node<Declarator<T::Interned>>, sym: Symbol) {
        if let Some(name) = find_declarator_name::<T>(&d.node.kind.node) {
            self.add_interned_symbol(&name, sym)
        }
    }

    #[cfg(test)]
    pub fn add_symbol(&mut self, s: &str, symbol: Symbol) {
        let scope = self
            .symbols
            .last_mut()
            .expect("at least one scope should be always present");
        scope.insert(self.interner.intern_str(s), symbol);
    }

    pub fn add_interned_symbol(&mut self, s: &T::Interned, symbol: Symbol) {
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

fn find_declarator_name<T: Interner>(d: &DeclaratorKind<T::Interned>) -> Option<&T::Interned> {
    match d {
        &DeclaratorKind::Abstract => None,
        &DeclaratorKind::Identifier(ref i) => Some(&i.node.name),
        &DeclaratorKind::Declarator(ref d) => find_declarator_name::<T>(&d.node.kind.node),
    }
}
