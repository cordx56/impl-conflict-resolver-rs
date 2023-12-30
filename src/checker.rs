use crate::common::*;
use std::collections::HashMap;

pub fn sub_b(b: &Bounds) -> &Vec<TExp> {
    &b.pos
}

pub struct Checker {
    ids: HashMap<String, Decl>,
}

impl Checker {
    pub fn sub(&self) {}
    pub fn d_t(&self, t: &TExp) {}
}
