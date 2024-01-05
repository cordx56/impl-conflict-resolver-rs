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
pub fn param(s: &str) -> IResult<&str, Param> {
    map(
        tuple((
            id,
            opt(map(
                tuple((multispace0, tag(":"), multispace0, new_trait_bounds)),
                |(_, _, _, b)| b,
            )),
        )),
        |(name, bounds)| Param { name, bounds },
    )(s)
}
pub fn params(s: &str) -> IResult<&str, Vec<Param>> {
    alt((
        map(tuple((tag("<"), multispace0, tag(">"))), |_| Vec::new()),
        map(
            tuple((
                tag("<"),
                multispace0,
                many0(map(
                    tuple((param, multispace0, tag(","), multispace0)),
                    |(p, _, _, _)| p,
                )),
                multispace0,
                param,
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
pub fn trait_def(s: &str) -> IResult<&str, Trait> {
    map(
        tuple((
            tag("trait"),
            multispace1,
            id,
            multispace0,
            opt(params),
            opt(map(
                tuple((multispace0, tag(":"), multispace0, new_trait_bounds)),
                |(_, _, _, b)| b,
            )),
            multispace0,
            tag("{"),
            multispace0,
            tag("}"),
        )),
        |(_, _, name, _, opt_params, subtraits, _, _, _, _)| Trait {
            name,
            params: opt_params.unwrap_or(Vec::new()),
            subtraits,
        },
    )(s)
}

pub fn args(s: &str) -> IResult<&str, Vec<TExp>> {
    alt((
        map(tuple((tag("<"), multispace0, tag(">"))), |_| Vec::new()),
        map(
            tuple((
                tag("<"),
                multispace0,
                many0(map(
                    tuple((t_exp, multispace0, tag(","), multispace0)),
                    |(e, _, _, _)| e,
                )),
                multispace0,
                t_exp,
                multispace0,
                tag(">"),
            )),
            |(_, _, init, _, last, _, _)| {
                let mut exps = init;
                exps.push(last);
                exps
            },
        ),
    ))(s)
}
pub fn impl_def(s: &str) -> IResult<&str, Impl> {
    map(
        tuple((
            tag("impl"),
            multispace0,
            opt(params),
            multispace0,
            id,
            multispace0,
            opt(args),
            multispace0,
            tag("for"),
            multispace1,
            id,
            multispace0,
            tag("{"),
            multispace0,
            tag("}"),
        )),
        |(_, _, opt_params, _, trait_name, _, opt_args, _, _, _, impl_for, _, _, _, _)| Impl {
            params: opt_params.unwrap_or(Vec::new()),
            trait_exp: TExp {
                name: trait_name,
                params: opt_args.unwrap_or(Vec::new()),
            },
            impl_for: Struct(impl_for),
        },
    )(s)
}

pub fn struct_def(s: &str) -> IResult<&str, Struct> {
    map(
        tuple((tag("struct"), multispace1, id, multispace0, tag(";"))),
        |(_, _, name, _, _)| Struct(name),
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

// Extend
pub fn new_trait_bounds(s: &str) -> IResult<&str, Bounds> {
    map(
        tuple((
            t_exp,
            many0(map(
                tuple((multispace0, tag("+"), multispace0, t_exp)),
                |(_, _, _, t_exp)| t_exp,
            )),
            many0(map(
                tuple((multispace0, tag("-"), multispace0, t_exp)),
                |(_, _, _, t_exp)| t_exp,
            )),
        )),
        |(head, comp, neg)| {
            let mut pos = vec![head];
            pos.extend(comp);
            Bounds { pos, neg }
        },
    )(s)
}
