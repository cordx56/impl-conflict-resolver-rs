pub mod checker;
pub mod parser;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct TExp {
    name: String,
    params: Vec<TExp>,
}
#[derive(Debug, Clone)]
pub struct Bound {
    pos: Vec<TExp>,
    neg: Vec<TExp>,
}
#[derive(Debug, Clone)]
pub struct Param {
    name: String,
    bound: Option<Bound>,
}
#[derive(Debug, Clone)]
pub struct Trait {
    name: String,
    params: Vec<Param>,
    supertraits: Option<Bound>,
}
#[derive(Debug, Clone)]
pub struct Impl {
    params: Vec<Param>,
    trait_exp: TExp,
    impl_for: TExp,
}
#[derive(Debug, Clone)]
pub struct Struct {
    name: String,
    params: Option<Vec<Param>>,
}

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
impl Display for Bound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.pos.iter();
        if let Some(first) = iter.next() {
            write!(f, "{}", first)?;
            for t in iter {
                write!(f, " + {}", t)?;
            }
        }
        for t in self.neg.iter() {
            write!(f, " + !{}", t)?;
        }
        Ok(())
    }
}
impl Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(b) = &self.bound {
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
        write!(f, " {} for {}", self.trait_exp, self.impl_for)
    }
}
impl Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if let Some(ps) = &self.params {
            let mut iter = ps.iter();
            if let Some(i) = iter.next() {
                write!(f, "<{}", i)?;
                for i in iter {
                    write!(f, ", {}", i)?;
                }
                write!(f, ">")?;
            }
        }
        Ok(())
    }
}
