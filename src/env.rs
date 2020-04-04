use std::collections::{HashMap, HashSet};

use ast::*;
use span::Node;
use strings;

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
enum Symbol {
    Typename,
    Identifier,
}

pub struct Env {
    symbols: Vec<HashMap<String, Symbol>>,
    pub extensions_gnu: bool,
    pub extensions_clang: bool,
    pub reserved: HashSet<&'static str>,
}

impl Env {
    #[cfg(test)]
    pub fn new() -> Env {
        Env::with_gnu()
    }

    pub fn with_core() -> Env {
        let mut reserved = HashSet::default();
        reserved.extend(strings::RESERVED_C11.iter());
        Env {
            extensions_gnu: false,
            extensions_clang: false,
            symbols: vec![HashMap::default()],
            reserved: reserved,
        }
    }

    pub fn with_gnu() -> Env {
        let mut symbols = HashMap::default();
        let mut reserved = HashSet::default();
        symbols.insert("__builtin_va_list".to_owned(), Symbol::Typename);
        reserved.extend(strings::RESERVED_C11.iter());
        reserved.extend(strings::RESERVED_GNU.iter());
        Env {
            extensions_gnu: true,
            extensions_clang: false,
            symbols: vec![symbols],
            reserved: reserved,
        }
    }

    pub fn with_clang() -> Env {
        let mut symbols = HashMap::default();
        let mut reserved = HashSet::default();
        symbols.insert("__builtin_va_list".to_owned(), Symbol::Typename);
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

    pub fn is_typename(&self, s: &str) -> bool {
        self.symbols.iter().rev().find_map(|sc| sc.get(s)) == Some(&Symbol::Typename)
    }

    pub fn handle_declaration<'a, T>(
        &mut self,
        specifiers: &[Node<DeclarationSpecifier>],
        declarators: T,
    ) where
        T: 'a + Iterator<Item = &'a Node<Declarator>>,
    {
        let symbol_type = if specifiers.iter().any(is_typedef) {
            Symbol::Typename
        } else {
            Symbol::Identifier
        };
        for declarator in declarators {
            if let Some(name) = find_declarator_name(&declarator.node.kind.node) {
                self.add_symbol(name, symbol_type);
            }
        }
    }

    fn add_symbol(&mut self, s: &str, symbol: Symbol) {
        let scope = self
            .symbols
            .last_mut()
            .expect("at least one scope should be always present");
        scope.insert(s.to_string(), symbol);
    }

    #[cfg(test)]
    pub fn add_typename(&mut self, s: &str) {
        self.add_symbol(s, Symbol::Typename)
    }
}

fn is_typedef(ds: &Node<DeclarationSpecifier>) -> bool {
    match &ds.node {
        &DeclarationSpecifier::StorageClass(Node {
            node: StorageClassSpecifier::Typedef,
            ..
        }) => true,
        _ => false,
    }
}

fn find_declarator_name(d: &DeclaratorKind) -> Option<&str> {
    match d {
        &DeclaratorKind::Abstract => None,
        &DeclaratorKind::Identifier(ref i) => Some(&i.node.name),
        &DeclaratorKind::Declarator(ref d) => find_declarator_name(&d.node.kind.node),
    }
}
