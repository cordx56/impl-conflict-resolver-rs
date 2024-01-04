pub mod checker;
pub mod parser;

use std::fmt::Display;

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
    pub(crate) subtraits: Option<Bounds>,
}
#[derive(Debug, Clone)]
pub struct Impl {
    pub(crate) params: Vec<Param>,
    pub(crate) trait_exp: TExp,
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

impl Display for TExp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        let mut iter = self.params.iter();
        if let Some(first) = iter.next() {
            write!(f, "<{}", first)?;
            for t in iter {
                write!(f, ", {}", t)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}
impl Display for Bounds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.pos.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
            for t in iter {
                write!(f, " + {}", t)?;
            }
        }
        for t in self.neg.iter() {
            write!(f, " - {}", t)?;
        }
        Ok(())
    }
}
impl Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(b) = &self.bounds {
            write!(f, ": {}", b)?;
        }
        Ok(())
    }
}
impl Display for Impl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "impl")?;
        let mut iter = self.params.iter();
        if let Some(first) = iter.next() {
            write!(f, "<{}", first)?;
            for p in iter {
                write!(f, ", {}", p)?;
            }
            write!(f, ">")?;
        }
        write!(f, " {} for {}", self.trait_exp, self.impl_for.0)
    }
}
