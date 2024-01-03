use super::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
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
    constraints: HashMap<String, String>,
}
impl Env {
    pub fn new() -> Self {
        Self {
            bounds: HashMap::new(),
            constraints: HashMap::new(),
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
            Err("Type not found".to_string())
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

/*
struct ParamEnv {
    params: HashMap<String, Option<Bounds>>,
}
impl ParamEnv {
    /// トレイト宣言から ParamEnv を生成
    fn trait_env(t: &Trait) -> ParamEnv {
        let params = t
            .params
            .clone()
            .into_iter()
            .map(|p| (p.name, p.bounds))
            .collect();
        ParamEnv { params }
    }
    /// implementationから ParamEnv を生成
    fn impl_env(i: &Impl) -> ParamEnv {
        let params = i
            .params
            .clone()
            .into_iter()
            .map(|p| (p.name, p.bounds))
            .collect();
        ParamEnv { params }
    }
    /// 名前から TypeExpr を取得する
    pub fn get_type(&self, name: impl AsRef<str>) -> TypeExpr {
        let n = name.as_ref();
        if let Some(c) = self.constrait.get(n) {
            self.get_type(c)
        } else if let Some(b) = self.params.get(n) {
            TypeExpr::Param {
                name: n.to_string(),
                bounds: b.clone(),
            }
        } else {
            TypeExpr::Type(Struct(n.to_string()))
        }
    }
    /// パーサの作る TExp を TypeExprに変換する
    pub fn resolve_type(&self, te: &TExp) -> TypeExpr {
        if let Some(b) = self.params.get(&te.name) {
            TypeExpr::Param {
                name: te.name.clone(),
                bounds: b.clone(),
            }
        } else {
            TypeExpr::Type(Struct(te.name.clone()))
        }
    }
}

pub fn sub_b(env: &ParamEnv, b: &Bounds) -> Vec<TraitExpr> {
    b.pos
        .iter()
        .map(|t| TraitExpr {
            name: t.name.clone(),
            params: t.params.iter().map(|b| env.resolve(b)).collect(),
        })
        .collect()
}
*/

pub struct Checker {
    structs: HashSet<String>,
    traits: HashMap<String, Trait>,
    impls: HashMap<Struct, Vec<Impl>>,
}

impl Checker {
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
                    if let Some(v) = self.impls.get_mut(&i.impl_for) {
                        v.push(i);
                    } else {
                        self.impls.insert(i.impl_for.clone(), vec![i]);
                    }
                }
            }
        }
    }
    /// 論文中 SubB に相当
    pub fn sub_b(&self, env: &Env, b: &Bounds) -> Result<Vec<ConcreteTrait>, String> {
        let res = b
            .pos
            .iter()
            .map(|t| env.resolve_trait(self, t))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(res)
    }
    /// 論文中 Sub に相当
    pub fn sub(&self, te: &ConcreteTrait) -> Result<Vec<ConcreteTrait>, String> {
        if let Some(Trait {
            params, subtraits, ..
        }) = self.traits.get(&te.name)
        {
            let mut env = Env::new();
            env.insert(self, params);
            if let Some(bs) = subtraits {
                self.sub_b(&env, bs)
            } else {
                Ok(Vec::new())
            }
        } else {
            Err("Trait not declared".to_string())
        }
    }
    /// 論文中 D に相当
    pub fn d(&self, ct: &ConcreteTrait) -> Result<Vec<ConcreteTrait>, String> {
        let res = self
            .sub(ct)?
            .iter()
            .map(|t| self.d(t))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        Ok(res)
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
                    Ok(ConflictCheckResult::NotConflict)
                }
            },
            TypeExpr::Param(p1) => match te2 {
                TypeExpr::Type(_) => return self.check_type_expr(te2, te1),
                TypeExpr::Param(p2) => {
                    if let Some(b1) = p1 {
                        if let Some(b2) = p2 {
                        }
                    }
                    Ok(ConflictCheckResult::Conflict)
                }
            },
        }
    }

    pub fn analyze_impl(&self, i: &Impl) -> Result<(ConcreteTrait, Struct), String> {
        if let Some(t) = self.traits.get(&i.trait_exp.name) {
            let mut env = Env::new();
            env.insert(self, &i.params);
            Ok((env.resolve_trait(self, &i.trait_exp)?, i.impl_for.clone()))
        } else {
            Err("Trait not declared".to_string())
        }
    }
    fn check_impls(&self, i1: &Impl, i2: &Impl) -> Result<ConflictCheckResult, String> {
        let (i1_t, i1_s) = self.analyze_impl(i1)?;
        let (i2_t, i2_s) = self.analyze_impl(i1)?;
        if i1_s != i2_s {
            Ok(ConflictCheckResult::NotConflict)
        } else if i1_t.params.len() == 0 && i1_t.name == i2_t.name {
            Ok(ConflictCheckResult::Conflict)
        } else {
            let uv = i1_t.params.iter().zip(i2_t.params.iter());
            Ok(ConflictCheckResult::NotConflict)
        }
    }

    pub fn check(&mut self, p: Program) {
        self.insert(p);
        for (_, i) in self.impls.iter() {
            // self.check_impls
        }
    }
}
