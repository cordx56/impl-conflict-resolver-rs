use super::*;
use std::collections::{HashMap, HashSet};

pub fn sub_b(b: &Bounds) -> &Vec<TExp> {
    &b.pos
}
fn trait_env(t: &Trait) -> Env {
    Env(t
        .params
        .clone()
        .into_iter()
        .map(|p| (p.name, p.bounds))
        .collect())
}
fn impl_env(i: &Impl) -> Env {
    Env(i
        .params
        .clone()
        .into_iter()
        .map(|p| (p.name, p.bounds))
        .collect())
}

struct Env(HashMap<String, Option<Bounds>>);

pub struct Checker {
    structs: HashSet<Struct>,
    traits: HashMap<String, Trait>,
    impls: HashMap<Struct, Vec<Impl>>,
}

impl Checker {
    fn insert(&mut self, Program(p): Program) {
        for d in p {
            match d {
                Decl::Struct(s) => {
                    self.structs.insert(s);
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
    pub fn sub(&self, env: &Env, te: &TExp) -> Result<Vec<&TExp>, String> {
        if let Some(Trait { subtrait, .. }) = self.traits.get(&te.name) {
            if let Some(bs) = subtrait {
                Ok(sub_b(bs).iter().collect()) // 型パラメータ入った時のことあとで考える
            } else {
                Ok(Vec::new())
            }
        } else {
            Err(String::from("Trait not declared"))
        }
    }
    pub fn d_t(&self, env: &Env, te: &TExp) -> Result<Vec<&TExp>, String> {
        Ok(self
            .sub(env, te)?
            .iter()
            .map(|t| self.d_t(env, t))
            .flatten()
            .flatten()
            .collect())
    }
    pub fn d_b(&self, env: &Env, b: &Bounds) -> Result<Vec<&TExp>, String> {
        Ok(b.pos
            .iter()
            .map(|t| self.d_t(env, t))
            .flatten()
            .flatten()
            .collect())
    }

    fn check_impls(&self, i1: &Impl, i2: &Impl) {}

    pub fn check(&mut self, p: Program) {
        self.insert(p);
        for (_, i) in self.impls.iter() {
            // self.check_impls
        }
    }
}
