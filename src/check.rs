pub mod checker;
pub mod parser;

#[derive(Debug, Clone)]
pub struct TExp {
    pub(crate) name: String,
    pub(crate) params: Vec<TExp>,
}
#[derive(Debug, Clone)]
pub struct Bounds {
    pub(crate) pos: Vec<TExp>,
    pub(crate) neg: Vec<TExp>,
}
#[derive(Debug, Clone)]
pub struct Param {
    pub(crate) name: String,
    pub(crate) bounds: Option<Bounds>,
}
#[derive(Debug, Clone)]
pub struct Trait {
    pub(crate) name: String,
    pub(crate) params: Vec<Param>,
    pub(crate) subtrait: Option<Bounds>,
}
#[derive(Debug, Clone)]
pub struct Impl {
    pub(crate) params: Vec<Param>,
    pub(crate) trait_name: String,
    pub(crate) args: Vec<TExp>,
    pub(crate) impl_for: Struct,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Struct(pub(crate) String);

#[derive(Debug, Clone)]
pub enum Decl {
    Struct(Struct),
    Trait(Trait),
    Impl(Impl),
}

#[derive(Debug, Clone)]
pub struct Program(pub(crate) Vec<Decl>);
