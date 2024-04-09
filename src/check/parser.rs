use super::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1, newline},
    combinator::{all_consuming, map, opt},
    multi::many0,
    sequence::tuple,
    IResult,
};

pub fn id(s: &str) -> IResult<&str, String> {
    map(alphanumeric1, |name: &str| name.to_string())(s)
}

pub fn t_exp(s: &str) -> IResult<&str, TExp> {
    map(
        tuple((
            id,
            opt(map(
                tuple((
                    multispace0,
                    tag("<"),
                    multispace0,
                    many0(map(
                        tuple((t_exp, multispace0, tag(","), multispace0)),
                        |(p_exp, _, _, _)| p_exp,
                    )),
                    multispace0,
                    t_exp,
                    multispace0,
                    tag(">"),
                )),
                |(_, _, _, init, _, last, _, _)| {
                    let mut params = init;
                    params.push(last);
                    params
                },
            )),
        )),
        |(name, opt_params)| TExp {
            name,
            params: opt_params.unwrap_or(Vec::new()),
        },
    )(s)
}
pub fn param(
    bounds: impl Fn(&str) -> IResult<&str, Bound>,
) -> impl FnMut(&str) -> IResult<&str, Param> {
    move |s| {
        map(
            tuple((
                id,
                opt(map(
                    tuple((multispace0, tag(":"), multispace0, &bounds)),
                    |(_, _, _, b)| b,
                )),
            )),
            |(name, bound)| Param { name, bound },
        )(s)
    }
}
pub fn params(
    bounds: impl Fn(&str) -> IResult<&str, Bound>,
) -> impl FnMut(&str) -> IResult<&str, Vec<Param>> {
    move |s| {
        alt((
            map(tuple((tag("<"), multispace0, tag(">"))), |_| Vec::new()),
            map(
                tuple((
                    tag("<"),
                    multispace0,
                    many0(map(
                        tuple((param(&bounds), multispace0, tag(","), multispace0)),
                        |(p, _, _, _)| p,
                    )),
                    multispace0,
                    param(&bounds),
                    multispace0,
                    tag(">"),
                )),
                |(_, _, init, _, last, _, _)| {
                    let mut params = init;
                    params.push(last);
                    params
                },
            ),
        ))(s)
    }
}
pub fn trait_def(s: &str) -> IResult<&str, Trait> {
    map(
        tuple((
            tag("trait"),
            multispace1,
            id,
            multispace0,
            opt(params(trait_bound)),
            opt(map(
                tuple((multispace0, tag(":"), multispace0, trait_bound)),
                |(_, _, _, b)| b,
            )),
            multispace0,
            tag("{"),
            multispace0,
            tag("}"),
        )),
        |(_, _, name, _, opt_params, supertraits, _, _, _, _)| Trait {
            name,
            params: opt_params.unwrap_or(Vec::new()),
            supertraits,
        },
    )(s)
}

pub fn impl_def(s: &str) -> IResult<&str, Impl> {
    map(
        tuple((
            tag("impl"),
            multispace0,
            opt(params(extend_trait_bound)),
            multispace0,
            t_exp,
            multispace0,
            tag("for"),
            multispace1,
            t_exp,
            multispace0,
            tag("{"),
            multispace0,
            tag("}"),
        )),
        |(_, _, opt_params, _, trait_exp, _, _, _, impl_for, _, _, _, _)| Impl {
            params: opt_params.unwrap_or(Vec::new()),
            trait_exp,
            impl_for,
        },
    )(s)
}

pub fn struct_def(s: &str) -> IResult<&str, Struct> {
    map(
        tuple((
            tag("struct"),
            multispace1,
            id,
            multispace0,
            opt(params(trait_bound)),
            multispace0,
            tag(";"),
        )),
        |(_, _, name, _, params, _, _)| Struct { name, params },
    )(s)
}

pub fn program(s: &str) -> IResult<&str, Program> {
    all_consuming(map(
        many0(map(
            tuple((
                multispace0,
                alt((
                    map(struct_def, |d| Decl::Struct(d)),
                    map(trait_def, |d| Decl::Trait(d)),
                    map(impl_def, |d| Decl::Impl(d)),
                )),
                newline,
                multispace0,
            )),
            |(_, d, _, _)| d,
        )),
        |ds| Program(ds),
    ))(s)
}

pub fn trait_bound(s: &str) -> IResult<&str, Bound> {
    map(
        tuple((
            t_exp,
            many0(map(
                tuple((multispace0, tag("+"), multispace0, t_exp)),
                |(_, _, _, t_exp)| t_exp,
            )),
        )),
        |(head, comp)| {
            let mut pos = vec![head];
            pos.extend(comp);
            Bound {
                pos,
                neg: Vec::new(),
            }
        },
    )(s)
}
// Extend
pub fn extend_trait_bound(s: &str) -> IResult<&str, Bound> {
    map(
        tuple((
            opt(tag("!")),
            t_exp,
            many0(map(
                tuple((multispace0, tag("+"), multispace0, opt(tag("!")), t_exp)),
                |(_, _, _, opr, t_exp)| (opr, t_exp),
            )),
        )),
        |(first_neg, head, tail)| {
            let mut pos = Vec::new();
            let mut neg = Vec::new();
            if first_neg.is_none() {
                pos.push(head);
            } else {
                neg.push(head);
            }
            for (opr, t_exp) in tail {
                if opr.is_none() {
                    pos.push(t_exp);
                } else {
                    neg.push(t_exp);
                }
            }
            Bound { pos, neg }
        },
    )(s)
}
