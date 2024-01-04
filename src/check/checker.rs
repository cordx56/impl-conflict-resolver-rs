use super::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictCheckResult {
    Conflict,
    NotConflict,
}

#[derive(Debug, Clone)]
pub enum TypeExpr {
    Type(Struct),
    Param(Option<ConcreteBounds>),
}
#[derive(Debug, Clone)]
pub struct ConcreteTrait {
    pub(crate) name: String,
    pub(crate) params: Vec<TypeExpr>,
}
#[derive(Debug, Clone)]
pub struct ConcreteBounds {
    pub(crate) pos: Vec<ConcreteTrait>,
    pub(crate) neg: Vec<ConcreteTrait>,
}
#[derive(Debug, Clone)]
struct Env {
    bounds: HashMap<String, Option<ConcreteBounds>>,
}
impl Env {
    pub fn new() -> Self {
        Self {
            bounds: HashMap::new(),
        }
    }
    pub fn insert<'a>(
        &mut self,
        checker: &Checker,
        params: impl IntoIterator<Item = &'a Param>,
    ) -> Result<(), String> {
        for p in params.into_iter() {
            let opt_cb = if let Some(b) = &p.bounds {
                Some(self.resolve_bounds(checker, b)?)
            } else {
                None
            };
            self.bounds.insert(p.name.clone(), opt_cb);
        }
        Ok(())
    }
    pub fn resolve_type(&self, checker: &Checker, te: &TExp) -> Result<TypeExpr, String> {
        if let Some(b) = self.bounds.get(&te.name) {
            Ok(TypeExpr::Param(b.clone()))
        } else if checker.structs.contains(&te.name) {
            Ok(TypeExpr::Type(Struct(te.name.clone())))
        } else {
            Err(format!("Type {} not found", te))
        }
    }
    pub fn resolve_bounds(&self, checker: &Checker, b: &Bounds) -> Result<ConcreteBounds, String> {
        let pos = b
            .pos
            .iter()
            .map(|p| self.resolve_trait(checker, p))
            .collect::<Result<Vec<_>, _>>()?;
        let neg = b
            .neg
            .iter()
            .map(|p| self.resolve_trait(checker, p))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ConcreteBounds { pos, neg })
    }
    pub fn resolve_trait(&self, checker: &Checker, te: &TExp) -> Result<ConcreteTrait, String> {
        let params = te
            .params
            .iter()
            .map(|p| self.resolve_type(checker, p))
            .collect::<Result<Vec<_>, _>>()?;
        let name = te.name.clone();
        Ok(ConcreteTrait { name, params })
    }
}

/// 検査を行うやつ
pub struct Checker {
    structs: HashSet<String>,
    traits: HashMap<String, Trait>,
    impls: HashMap<String, Vec<Impl>>,
}

impl Checker {
    pub fn new() -> Self {
        Self {
            structs: HashSet::new(),
            traits: HashMap::new(),
            impls: HashMap::new(),
        }
    }
    fn insert(&mut self, Program(p): Program) {
        for d in p {
            match d {
                Decl::Struct(s) => {
                    self.structs.insert(s.0);
                }
                Decl::Trait(t) => {
                    self.traits.insert(t.name.clone(), t);
                }
                Decl::Impl(i) => {
                    if let Some(v) = self.impls.get_mut(&i.impl_for.0) {
                        v.push(i);
                    } else {
                        self.impls.insert(i.impl_for.0.clone(), vec![i]);
                    }
                }
            }
        }
    }
    /// 論文中 SubB に相当
    fn sub_b(&self, env: &Env, b: &Bounds) -> Result<Vec<ConcreteTrait>, String> {
        let res = b
            .pos
            .iter()
            .map(|t| env.resolve_trait(self, t))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(res)
    }
    /// 論文中 Sub に相当
    pub fn sub(&self, ct: &ConcreteTrait) -> Result<Vec<ConcreteTrait>, String> {
        if let Some(Trait {
            params, subtraits, ..
        }) = self.traits.get(&ct.name)
        {
            let mut env = Env::new();
            env.insert(self, params)?;
            if let Some(bs) = subtraits {
                self.sub_b(&env, bs)
            } else {
                Ok(Vec::new())
            }
        } else {
            Err(format!("Trait {} not declared", ct.name))
        }
    }
    /// 論文中 D に相当
    pub fn d(&self, ct: &ConcreteTrait) -> Result<Vec<ConcreteTrait>, String> {
        let mut res = vec![ct.clone()];
        let d: Vec<_> = self
            .sub(ct)?
            .iter()
            .map(|t| self.d(t))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        res.extend(d);
        Ok(res)
    }
    pub fn d_s<'a>(
        &self,
        cts: impl IntoIterator<Item = &'a ConcreteTrait>,
    ) -> Result<Vec<ConcreteTrait>, String> {
        let res = cts
            .into_iter()
            .map(|t| self.d(t))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        Ok(res)
    }

    fn check_bounds(
        &self,
        b: &ConcreteBounds,
        c: &ConcreteBounds,
    ) -> Result<ConflictCheckResult, String> {
        if b.neg.len() == 0 && c.neg.len() == 0 {
            return Ok(ConflictCheckResult::Conflict);
        }
        for n in self.d_s(&b.neg)? {
            for p in self.d_s(&c.pos)? {
                if self.check_trait(&n, &p)? == ConflictCheckResult::Conflict {
                    return Ok(ConflictCheckResult::NotConflict);
                }
            }
        }
        for n in self.d_s(&c.neg)? {
            for p in self.d_s(&b.pos)? {
                if self.check_trait(&n, &p)? == ConflictCheckResult::Conflict {
                    return Ok(ConflictCheckResult::NotConflict);
                }
            }
        }
        Ok(ConflictCheckResult::Conflict)
    }
    pub fn check_type_expr(
        &self,
        te1: &TypeExpr,
        te2: &TypeExpr,
    ) -> Result<ConflictCheckResult, String> {
        match te1 {
            TypeExpr::Type(t1) => match te2 {
                TypeExpr::Type(t2) => {
                    if t1 == t2 {
                        Ok(ConflictCheckResult::Conflict)
                    } else {
                        Ok(ConflictCheckResult::NotConflict)
                    }
                }
                TypeExpr::Param(p2) => {
                    if let Some(cb) = p2 {
                        if let Some(is) = self.impls.get(&t1.0) {
                            for i in is {
                                let mut env = Env::new();
                                env.insert(self, &i.params)?;
                                let t = env.resolve_trait(self, &i.trait_exp)?;
                                let pos = self.d(&t)?;
                                let new_b = ConcreteBounds {
                                    pos,
                                    neg: Vec::new(),
                                };
                                if self.check_bounds(cb, &new_b)? == ConflictCheckResult::Conflict {
                                    return Ok(ConflictCheckResult::Conflict);
                                }
                            }
                        }
                        Ok(ConflictCheckResult::NotConflict)
                    } else {
                        Ok(ConflictCheckResult::Conflict)
                    }
                }
            },
            TypeExpr::Param(p1) => match te2 {
                TypeExpr::Type(_) => self.check_type_expr(te2, te1),
                TypeExpr::Param(p2) => {
                    if let Some(b1) = p1 {
                        if let Some(b2) = p2 {
                            return Ok(self.check_bounds(b1, b2)?);
                        }
                    }
                    Ok(ConflictCheckResult::Conflict)
                }
            },
        }
    }
    pub fn check_trait(
        &self,
        t1: &ConcreteTrait,
        t2: &ConcreteTrait,
    ) -> Result<ConflictCheckResult, String> {
        if t1.name != t2.name {
            Ok(ConflictCheckResult::NotConflict)
        } else {
            let uv = t1
                .params
                .iter()
                .zip(t2.params.iter())
                .map(|(u, v)| self.check_type_expr(u, v))
                .collect::<Result<Vec<_>, _>>()?;
            if uv.contains(&ConflictCheckResult::NotConflict) {
                Ok(ConflictCheckResult::NotConflict)
            } else {
                Ok(ConflictCheckResult::Conflict)
            }
        }
    }

    pub fn analyze_impl(&self, i: &Impl) -> Result<(ConcreteTrait, Struct), String> {
        if let Some(_) = self.traits.get(&i.trait_exp.name) {
            let mut env = Env::new();
            env.insert(self, &i.params)?;
            Ok((env.resolve_trait(self, &i.trait_exp)?, i.impl_for.clone()))
        } else {
            Err("Trait not declared".to_string())
        }
    }
    fn check_impls(&self, i1: &Impl, i2: &Impl) -> Result<ConflictCheckResult, String> {
        let (i1_t, i1_s) = self.analyze_impl(i1)?;
        let (i2_t, i2_s) = self.analyze_impl(i2)?;
        if i1_s != i2_s {
            Ok(ConflictCheckResult::NotConflict)
        } else {
            if i1_t.name == i2_t.name {
                if i1_t.params.len() == 0 {
                    return Ok(ConflictCheckResult::Conflict);
                }
                let results = i1_t
                    .params
                    .iter()
                    .zip(i2_t.params.iter())
                    .map(|(u, v)| self.check_type_expr(u, v))
                    .collect::<Result<Vec<_>, _>>()?;
                if results.contains(&ConflictCheckResult::NotConflict) {
                    Ok(ConflictCheckResult::NotConflict)
                } else {
                    Ok(ConflictCheckResult::Conflict)
                }
            } else {
                Ok(ConflictCheckResult::NotConflict)
            }
        }
    }

    pub fn check(&mut self, p: Program) -> Result<Vec<(ConflictCheckResult, Impl, Impl)>, String> {
        let mut res = Vec::new();
        self.insert(p);
        for (_, im) in self.impls.iter() {
            for i in 0..im.len() {
                for j in i + 1..im.len() {
                    let i = im[i].clone();
                    let j = im[j].clone();
                    let check_result = self.check_impls(&i, &j)?;
                    res.push((check_result, i, j));
                }
            }
        }
        Ok(res)
    }
}
