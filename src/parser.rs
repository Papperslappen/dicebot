use pom::parser::*;
use std::str::{self, FromStr};

use crate::expressiontree::DiceExpression;

//matches whitespace
fn space<'a>() -> Parser<'a,u8,()> {
    one_of(b" \t\r\n").repeat(1..).discard()
}

//matches integers
fn integer<'a>() -> Parser<'a,u8,i64> {
    let number = sym(b'-').opt() + one_of(b"0123456789").repeat(1..);
    number.collect()
        .convert(str::from_utf8)
        .convert(|s|{i64::from_str(s)})
}

//matches positive nonzero integers
fn positive_integer<'a>() -> Parser<'a,u8, i64>{
    (one_of(b"123456789") + one_of(b"0123456789")
        .repeat(0..))
        .collect()
        .convert(str::from_utf8)
        .convert(|s|{i64::from_str(s)})
}

//parses a number into an DiceExpression::Constant
fn constant<'a>() -> Parser<'a,u8, DiceExpression>{
        integer().map(|i| {DiceExpression::Constant(i)})
}

//matches dice notation (both English and Swedish variants) and returns an DiceExpression::Uniform
fn die<'a>() -> Parser<'a,u8,DiceExpression>{
    (positive_integer().opt() - one_of(b"dt") + positive_integer())
    .map(|(n,d)| {
        match n {
            // For example 2d8
            Some(i) if i > 1 => {
                DiceExpression::Die(d).repeat(i).sum()
            }
            // 1d6 or d10
            Some(_) | None => {
                DiceExpression::Die(d)
            }
        }
    })
}

//unary minus
fn negative<'a>() -> Parser<'a,u8,DiceExpression> {
    (sym(b'-') * space().opt() * call(expression)).map(|e|e.negate())
}

//sum
fn sum<'a>() -> Parser<'a,u8,DiceExpression> {
    (leaf() - sym(b'+') + call(expression))
        .map(|(e,f)| e.add(f))
}
//difference
fn difference<'a>() -> Parser<'a,u8,DiceExpression> {
    (leaf() - sym(b'-') + call(expression))
        .map(|(e,f)| e.subtract(f))
}

//non compound expression trees (no sums)
fn leaf<'a>() -> Parser<'a,u8,DiceExpression> {
    space().opt() *
    ( repeat_many()
    | die()
    | parenthesis()
    | constant())
    -space().opt()
}

fn parenthesis<'a>() -> Parser<'a,u8,DiceExpression> {
    sym(b'(') * space().opt() * call(expression) - space().opt() - sym(b')')
}

// multiple expressions separated by whitespace
fn many<'a>() -> Parser<'a,u8,DiceExpression> {
    (leaf() - sym(b',') - space().opt() + call(expression))
        .map(|(u,v)| DiceExpression::Many(vec!(u,v)))
}

// repeat expression n times
fn repeat_many<'a>() -> Parser<'a,u8,DiceExpression> {
    (positive_integer() - sym(b'.') + call(leaf))
        .map(|(i,e)| e.repeat(i))
}

fn expression<'a>() -> Parser<'a,u8,DiceExpression> {
    space().opt() *
    ( many()
    | sum()
    | difference()
    | negative()
    | leaf()
    ) - space().opt()
}

pub fn simple_dice_parser<'a>() -> Parser<'a,u8,DiceExpression> {
    expression() - end()
}

pub fn parse(s:String) -> Result<DiceExpression,&'static str> {
    let p = simple_dice_parser();
    if let Ok(p) = p.parse(s.as_bytes()){
        Ok(p)
    } else {
        Err("not parsable")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_constant(){
        let parser = super::constant();
        let input = b"2";
        assert_eq!(Ok(super::DiceExpression::Constant(2)),parser.parse(input));
    }

}
