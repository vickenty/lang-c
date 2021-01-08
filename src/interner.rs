use ast::Name;

/// Trait that specifies objects which can be used as interners.
///
pub trait Interner: Default {
    type Interned: Name;

    fn intern_str<S: AsRef<str>>(&mut self, string: S) -> Self::Interned;
    fn recover_str(&mut self, handle: Self::Interned) -> String;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Default)]
pub struct DummyInterner;

impl Interner for DummyInterner {
    type Interned = String;

    fn intern_str<S: AsRef<str>>(&mut self, string: S) -> String {
        string.as_ref().to_owned()
    }

    fn recover_str(&mut self, handle: Self::Interned) -> String {
        handle
    }
}
