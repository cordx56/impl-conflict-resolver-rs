use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1},
    combinator::{map, opt, value},
    multi::many0,
    sequence::tuple,
    IResult,
};

pub struct TExp {
    name: String,
    params: Vec<TExp>,
}
pub struct Bounds {
    pos: Vec<TExp>,
    neg: Vec<TExp>,
}
pub struct Param {
    name: String,
    bounds: Bounds,
}
pub struct Trait {
    name: String,
    params: Vec<Param>,
    subtrait: Bounds,
}
pub struct Impl {
    params: Vec<Param>,
    trait_name: String,
    args: Vec<TExp>,
    impl_for: TExp,
}

pub fn id(s: &str) -> IResult<&str, String> {
    map(alphanumeric1, |name: &str| name.to_string())(s)
}

pub fn t_exp(s: &str) -> IResult<&str, TExp> {
    map(
        tuple((
            id,
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
        |(name, _, _, _, init, _, last, _, _)| {
            let mut params = init;
            params.push(last);
            TExp { name, params }
        },
    )(s)
}
pub fn trait_bounds(s: &str) -> IResult<&str, Bounds> {
    map(
        tuple((
            t_exp,
            many0(map(
                tuple((multispace0, tag("+"), multispace0, t_exp)),
                |(_, _, _, t_exp)| t_exp,
            )),
        )),
        |(head, tail)| {
            let mut pos = vec![head];
            pos.extend(tail);
            Bounds {
                pos,
                neg: Vec::new(),
            }
        },
    )(s)
}
pub fn param(s: &str) -> IResult<&str, Param> {
    map(
        tuple((id, multispace0, tag(":"), multispace0, trait_bounds)),
        |(name, _, _, _, bounds)| Param { name, bounds },
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
            params,
            multispace0,
            tag(":"),
            multispace0,
            trait_bounds,
            multispace0,
            tag("{"),
            multispace0,
            tag("}"),
        )),
        |(_, _, name, _, params, _, _, _, subtrait, _, _, _, _)| Trait {
            name,
            params,
            subtrait,
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
            params,
            multispace0,
            id,
            multispace0,
            opt(args),
            multispace1,
            tag("for"),
            multispace1,
            t_exp,
            multispace0,
            tag("{"),
            multispace0,
            tag("}"),
        )),
        |(_, _, params, _, trait_name, _, opt_args, _, _, _, impl_for, _, _, _, _)| {
            let args = if let Some(args) = opt_args {
                args
            } else {
                Vec::new()
            };
            Impl {
                params,
                trait_name,
                args,
                impl_for,
            }
        },
    )(s)
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
