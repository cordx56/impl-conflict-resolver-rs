use super::*;
use anyhow::{anyhow, Context as _, Result};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictCheckResult {
    Conflict,
    NonConflict,
}

#[derive(Debug, Clone, Eq, Hash)]
pub enum ConcreteType {
    Type {
        name: String,
        params: Vec<ConcreteType>,
    },
    Param {
        id: usize,
        name: String,
    },
}
impl PartialEq for ConcreteType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Type {
                name: n1,
                params: ps1,
            } => match other {
                Self::Type {
                    name: n2,
                    params: ps2,
                } => {
                    if n1 != n2 {
                        return false;
                    }
                    for (p1, p2) in ps1.iter().zip(ps2.iter()) {
                        if p1 != p2 {
                            return false;
                        }
                    }
                    true
                }
                Self::Param { .. } => false,
            },
            Self::Param { id: id1, .. } => match other {
                Self::Type { .. } => false,
                Self::Param { id: id2, .. } => id1 == id2,
            },
        }
    }
}
impl Display for ConcreteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConcreteType::Type { name, params } => {
                write!(f, "{}", name)?;
                let mut iter = params.iter();
                if let Some(i) = iter.next() {
                    write!(f, "<{}", i)?;
                    for i in iter {
                        write!(f, ", {}", i)?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
            ConcreteType::Param { name, .. } => {
                write!(f, "{}", name)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConcreteTrait {
    name: String,
    params: Vec<ConcreteType>,
}
impl Display for ConcreteTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        let mut iter = self.params.iter();
        if let Some(i) = iter.next() {
            write!(f, "<{}", i)?;
            for i in iter {
                write!(f, ", {}", i)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct ConcreteBound {
    pos: HashSet<ConcreteTrait>,
    neg: HashSet<ConcreteTrait>,
}
impl ConcreteBound {
    pub fn join(b: &ConcreteBound, c: &ConcreteBound) -> ConcreteBound {
        let b_pos = &b.pos;
        let c_pos = &c.pos;
        let pos: HashSet<_> = b_pos.union(&c_pos).cloned().collect();
        let b_neg = &b.neg;
        let c_neg = &c.neg;
        let neg: HashSet<_> = b_neg.union(&c_neg).cloned().collect();
        let bound = ConcreteBound { pos, neg };
        bound
    }
}
impl Display for ConcreteBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pos = self.pos.iter();
        if let Some(p) = pos.next() {
            write!(f, "{}", p)?;
            for p in pos {
                write!(f, " + {}", p)?;
            }
        }
        for n in self.neg.iter() {
            write!(f, " - {}", n)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnificationResult {
    Ok,
    Failure,
}
#[derive(Debug, Clone)]
struct Unifier(HashMap<usize, ConcreteType>);
impl Unifier {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn resolve(&mut self, cty: &ConcreteType) -> ConcreteType {
        match cty {
            ConcreteType::Type { name, params } => {
                let params = params.iter().map(|p| self.resolve(p)).collect();
                ConcreteType::Type {
                    name: name.clone(),
                    params,
                }
            }
            ConcreteType::Param { id, .. } => {
                let sct = self.0.get(id).cloned();
                if let Some(ct) = sct {
                    let new_ct = self.resolve(&ct);
                    // あとでトレイト境界を集める時に経路を辿れるようにする
                    //self.0.insert(*id, new_ct.clone());
                    new_ct
                } else {
                    cty.clone()
                }
            }
        }
    }

    pub fn type_unify(&mut self, cty1: &ConcreteType, cty2: &ConcreteType) -> UnificationResult {
        let cty1 = self.resolve(cty1);
        let cty2 = self.resolve(cty2);
        match &cty1 {
            ConcreteType::Type {
                name: n1,
                params: ps1,
            } => match &cty2 {
                ConcreteType::Type {
                    name: n2,
                    params: ps2,
                } => {
                    if n1 != n2 {
                        return UnificationResult::Failure;
                    }
                    for (p1, p2) in ps1.iter().zip(ps2.iter()) {
                        if self.type_unify(p1, p2) == UnificationResult::Failure {
                            return UnificationResult::Failure;
                        }
                    }
                    UnificationResult::Ok
                }
                ConcreteType::Param { id, .. } => {
                    self.0.insert(*id, cty1.clone());
                    UnificationResult::Ok
                }
            },
            ConcreteType::Param { id: id1, .. } => match cty2 {
                ConcreteType::Type { .. } => self.type_unify(&cty2, &cty1),
                ConcreteType::Param { .. } => {
                    self.0.insert(*id1, cty2.clone());
                    UnificationResult::Ok
                }
            },
        }
    }
    pub fn trait_unify(&mut self, ctr1: &ConcreteTrait, ctr2: &ConcreteTrait) -> UnificationResult {
        if ctr1.name != ctr2.name {
            UnificationResult::Failure
        } else {
            for (p1, p2) in ctr1.params.iter().zip(ctr2.params.iter()) {
                if self.type_unify(p1, p2) == UnificationResult::Failure {
                    return UnificationResult::Failure;
                }
            }
            UnificationResult::Ok
        }
    }

    fn get_routes(&self, routes: &mut Vec<(Option<ConcreteType>, HashSet<usize>)>, i: usize) {
        let mut route = None;
        for r in routes.iter_mut() {
            if r.1.contains(&i) {
                route = Some(r)
            }
        }
        if route.is_none() {
            let last = routes.len();
            routes.push((None, HashSet::from([i])));
            route = routes.get_mut(last);
        }
        let route = route.unwrap();
        if let Some(ct) = self.0.get(&i) {
            match ct {
                ConcreteType::Param { id, .. } => {
                    route.1.insert(*id);
                    self.get_routes(routes, *id);
                }
                ConcreteType::Type { .. } => {
                    route.0 = Some(ct.clone());
                }
            }
        } else {
            route.1.insert(i);
        }
    }
    pub fn get_all_unified_params(&self) -> Vec<(Option<ConcreteType>, HashSet<usize>)> {
        let mut routes = Vec::new();
        for i in self.0.keys() {
            self.get_routes(&mut routes, *i);
        }
        routes
    }
}

struct ConcreteImpl {
    trait_exp: ConcreteTrait,
    impl_for: ConcreteType,
}
struct ConflictCheckEnv<'a> {
    checker: &'a Checker,
    params: Vec<Option<ConcreteBound>>,
}
impl<'a> ConflictCheckEnv<'a> {
    pub fn new(checker: &'a Checker) -> Self {
        Self {
            checker,
            params: Vec::new(),
        }
    }
    fn texp_to_concrete_type(
        &self,
        env: &HashMap<String, usize>,
        te: &TExp,
    ) -> Result<ConcreteType> {
        if let Some(id) = env.get(&te.name) {
            Ok(ConcreteType::Param {
                id: *id,
                name: te.name.clone(),
            })
        } else {
            if let Some(st) = self.checker.structs.get(&te.name) {
                let te_param_len = te.params.len();
                if st.params.as_ref().map(|ps| ps.len()).unwrap_or(0) != te_param_len {
                    Err(anyhow!("Param length error between {} and {}", st, te))
                } else {
                    let params = te
                        .params
                        .iter()
                        .map(|p| self.texp_to_concrete_type(env, p))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(ConcreteType::Type {
                        name: te.name.clone(),
                        params,
                    })
                }
            } else {
                Err(anyhow!("Undefined struct {}", te.name))
            }
        }
    }
    fn texp_to_concrete_trait(
        &self,
        env: &HashMap<String, usize>,
        te: &TExp,
    ) -> Result<ConcreteTrait> {
        let params = te
            .params
            .iter()
            .map(|t| self.texp_to_concrete_type(env, t))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(ConcreteTrait {
            name: te.name.clone(),
            params,
        })
    }

    /// 論文中 Sup に相当
    pub fn sup(&self, ct: &ConcreteTrait) -> Result<Vec<ConcreteTrait>> {
        if let Some(Trait {
            params,
            supertraits,
            ..
        }) = self.checker.traits.get(&ct.name)
        {
            let mut env = HashMap::new();
            for (n, t) in params.iter().map(|p| &p.name).zip(ct.params.iter()) {
                env.insert(n.clone(), t.clone());
            }
            if let Some(supertraits) = supertraits {
                let supertraits = supertraits
                    .pos
                    .iter()
                    .map(|t| {
                        let params = t
                            .params
                            .iter()
                            .map(|p| {
                                if let Some(ct) = env.get(&p.name) {
                                    Ok(ct.clone())
                                } else {
                                    self.texp_to_concrete_type(&HashMap::new(), p)
                                }
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        anyhow::Ok(ConcreteTrait {
                            name: t.name.clone(),
                            params,
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(supertraits)
            } else {
                Ok(Vec::new())
            }
        } else {
            Err(anyhow!("Trait {} not declared", ct.name))
        }
    }
    /// 論文中 A' に相当
    pub fn a_d(&self, ct: &ConcreteTrait) -> Result<HashSet<ConcreteTrait>> {
        let mut res = HashSet::from([ct.clone()]);
        let sups = self
            .sup(ct)?
            .iter()
            .map(|t| self.sup(t))
            .collect::<Result<HashSet<_>, _>>()?
            .into_iter()
            .flatten()
            .collect::<HashSet<_>>();
        res.extend(sups);
        Ok(res)
    }
    /// 論文中 A に相当
    pub fn a(&self, b: &ConcreteBound) -> Result<HashSet<ConcreteTrait>> {
        let res = b
            .pos
            .iter()
            .map(|t| self.a_d(t))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect();
        Ok(res)
    }

    /// Check trait bounds
    fn check_bound(&self, b: &ConcreteBound) -> Result<ConflictCheckResult> {
        let a = self.a(b).with_context(|| format!("A({}) error", b))?;
        if 0 < a.intersection(&b.neg).count() {
            Ok(ConflictCheckResult::NonConflict)
        } else {
            Ok(ConflictCheckResult::Conflict)
        }
    }

    pub fn get_concrete_impl(&mut self, im: &Impl) -> Result<ConcreteImpl> {
        let mut env = HashMap::new();
        for p in &im.params {
            let param_id = self.params.len();
            if let Some(bound) = &p.bound {
                let pos = bound
                    .pos
                    .iter()
                    .map(|t| self.texp_to_concrete_trait(&env, t))
                    .collect::<Result<HashSet<_>, _>>()?;
                let neg = bound
                    .neg
                    .iter()
                    .map(|t| self.texp_to_concrete_trait(&env, t))
                    .collect::<Result<HashSet<_>, _>>()?;
                let b = ConcreteBound { pos, neg };
                self.params.push(Some(b));
            } else {
                self.params.push(None);
            }
            env.insert(p.name.clone(), param_id);
        }
        let trait_params = im
            .trait_exp
            .params
            .iter()
            .map(|p| self.texp_to_concrete_type(&env, p))
            .collect::<Result<Vec<_>, _>>()?;
        let trait_exp = ConcreteTrait {
            name: im.trait_exp.name.clone(),
            params: trait_params,
        };
        let impl_for = self.texp_to_concrete_type(&env, &im.impl_for)?;
        let cimpl = ConcreteImpl {
            trait_exp,
            impl_for,
        };
        Ok(cimpl)
    }

    fn check_impls<'b>(checker: &'a Checker, i1: &Impl, i2: &Impl) -> Result<ConflictCheckResult> {
        let mut env = Self::new(checker);
        let c1 = env.get_concrete_impl(i1)?;
        let c2 = env.get_concrete_impl(i2)?;
        let mut unif = Unifier::new();
        if unif.trait_unify(&c1.trait_exp, &c2.trait_exp) == UnificationResult::Ok {
            if unif.type_unify(&c1.impl_for, &c2.impl_for) == UnificationResult::Ok {
                let params = unif.get_all_unified_params();
                for (ct, ps) in params {
                    let mut bound = ConcreteBound {
                        pos: HashSet::new(),
                        neg: HashSet::new(),
                    };
                    for p in ps.iter() {
                        let b = &env.params[*p];
                        if let Some(b) = b {
                            bound = ConcreteBound::join(&bound, b);
                        } else {
                            return Ok(ConflictCheckResult::Conflict);
                        }
                    }
                    if let Some(_) = ct {
                        // 将来課題：現状では型変数と型式が単一化された段階で衝突
                        // orphan rule などを考慮しながら、どうするか考える
                        return Ok(ConflictCheckResult::Conflict);
                    } else {
                        if env
                            .check_bound(&bound)
                            .with_context(|| format!("Bound {} check error", bound))?
                            == ConflictCheckResult::NonConflict
                        {
                            return Ok(ConflictCheckResult::NonConflict);
                        }
                    }
                }
                return Ok(ConflictCheckResult::Conflict);
            }
        }
        Ok(ConflictCheckResult::NonConflict)
    }
}

/// 検査を行うやつ
pub struct Checker {
    structs: HashMap<String, Struct>,
    traits: HashMap<String, Trait>,
    impls: Vec<Impl>,
}

impl Checker {
    pub fn new() -> Self {
        Self {
            structs: HashMap::new(),
            traits: HashMap::new(),
            impls: Vec::new(),
        }
    }

    pub fn insert(&mut self, Program(p): Program) -> Result<()> {
        for d in p {
            match d {
                Decl::Struct(s) => {
                    self.structs.insert(s.name.clone(), s);
                }
                Decl::Trait(t) => {
                    self.traits.insert(t.name.clone(), t);
                }
                Decl::Impl(i) => {
                    self.impls.push(i);
                }
            }
        }
        Ok(())
    }

    fn check_impls(&self, i1: &Impl, i2: &Impl) -> Result<ConflictCheckResult> {
        ConflictCheckEnv::check_impls(self, i1, i2)
    }

    pub fn check(&mut self, p: Program) -> Result<Vec<(ConflictCheckResult, Impl, Impl)>> {
        let mut res = Vec::new();
        self.insert(p)?;
        for i in 0..self.impls.len() {
            for j in (i + 1)..self.impls.len() {
                let i1 = self.impls[i].clone();
                let i2 = self.impls[j].clone();
                res.push((
                    self.check_impls(&i1, &i2)
                        .with_context(|| format!("implementation {}, {} check error", i1, i2))?,
                    i1,
                    i2,
                ));
            }
        }
        Ok(res)
    }
}
