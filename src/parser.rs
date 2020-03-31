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
    (sym(b'-') * space().opt() * call(leaf)).map(|e|e.negate())
}

//Max
fn max<'a>() -> Parser<'a,u8,DiceExpression> {
    (seq(b"max") * space().opt() * call(leaf)).map(|e|e.max())
}

//Min
fn min<'a>() -> Parser<'a,u8,DiceExpression> {
    (seq(b"min") * space().opt() * call(leaf)).map(|e|e.min())
}

//Min
fn sum<'a>() -> Parser<'a,u8,DiceExpression> {
    (seq(b"sum") * space().opt() * call(leaf)).map(|e|e.sum())
}

//Sum
fn plus<'a>() -> Parser<'a,u8,DiceExpression> {
    (leaf() - sym(b'+') + call(expression))
        .map(|(e,f)| e.add(f))
}

//Difference
fn minus<'a>() -> Parser<'a,u8,DiceExpression> {
    (leaf() - sym(b'-') + call(expression))
        .map(|(e,f)| e.subtract(f))
}

//Multiply
fn multiply<'a>() -> Parser<'a,u8,DiceExpression> {
    ((die() | constant() | parenthesis()) - sym(b'*') + call(leaf))
        .map(|(e,f)| e.multiply(f))
}

//Less than
fn lt<'a>() -> Parser<'a,u8,DiceExpression> {
    (leaf() - sym(b'<') + call(expression))
        .map(|(e,f)| e.lt(f))
}

//Equal
fn eq<'a>() -> Parser<'a,u8,DiceExpression> {
    (leaf() - sym(b'=') + call(expression))
        .map(|(e,f)| e.eq(f))
}
//Greater than
fn gt<'a>() -> Parser<'a,u8,DiceExpression> {
    (leaf() - sym(b'>') + call(expression))
        .map(|(e,f)| e.gt(f))
}
// repeat expression n times
fn repeat_many<'a>() -> Parser<'a,u8,DiceExpression> {
    (positive_integer() - sym(b'.') + call(leaf))
        .map(|(i,e)| e.repeat(i))
}

// multiple expressions separated by whitespace
fn many<'a>() -> Parser<'a,u8,DiceExpression> {
    (leaf() - sym(b',') - space().opt() + call(expression))
        .map(|(u,v)| u.also(v))
}

// matches any expression in parenthesis
fn parenthesis<'a>() -> Parser<'a,u8,DiceExpression> {
    sym(b'(') * space().opt() * call(expression) - space().opt() - sym(b')')
}


//non compound expression trees (no sums)
fn leaf<'a>() -> Parser<'a,u8,DiceExpression> {
    space().opt() *
    ( multiply()
    | repeat_many()
    | die()
    | parenthesis()
    | constant()
    | negative()
    | max()
    | min()
    | sum())
    -space().opt()
}

fn expression<'a>() -> Parser<'a,u8,DiceExpression> {
    space().opt() *
    ( many()
    | plus()
    | minus()
    | lt()
    | gt()
    | eq()
    | negative()
    | leaf()
    ) - space().opt()
}

pub fn simple_dice_parser<'a>() -> Parser<'a,u8,DiceExpression> {
    expression() - end()
}

pub fn parse(s:&str) -> Result<DiceExpression,&'static str> {
    let p = simple_dice_parser();
    if let Ok(p) = p.parse(s.as_bytes()){
        Ok(p)
    } else {
        Err("not parsable")
    }
}

#[cfg(test)]
mod tests {
    use super::DiceExpression::*;



    #[test]
    fn constant(){
        let constant_parser = super::constant();
        let input = b"2";
        assert_eq!(Ok(Constant(2)),constant_parser.parse(input));
    }

    #[test]
    fn die(){
        let simple_parser = super::simple_dice_parser();
        assert_eq!(Ok(Die(6)),simple_parser.parse(b"d6"));
    }

}
