extern crate rand;
extern crate pom;
use rand::distributions::{Uniform,Distribution};
use pom::parser::*;
use std::str::{self, FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionTree{
    Sum(Vec<ExpressionTree>),
    Negative(Box<ExpressionTree>),
    Constant(i32),
    Uniform(i32,i32)
}

impl ExpressionTree {
    pub fn eval(&self) -> i32{
        match self{
            ExpressionTree::Constant(i) => {*i},
            ExpressionTree::Uniform(i,j) => {
                let dice = Uniform::new_inclusive(i,j);
                let mut rng = rand::thread_rng();
                dice.sample(&mut rng) as i32
            },
            ExpressionTree::Sum(v) => {
                v.iter().fold(0i32,|acc,x| acc+x.eval())
            }
            ExpressionTree::Negative(e) => {
                0 - e.eval()
            }
        }
    }
}

//matches whitespace
fn space<'a>() -> Parser<'a,u8,()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

//matches integers
fn integer<'a>() -> Parser<'a,u8,i32> {
    let number = sym(b'-').opt() + one_of(b"0123456789").repeat(1..);
    number.collect()
        .convert(str::from_utf8)
        .convert(|s|{i32::from_str(s)})
}

//matches positive nonzero integers
fn positive_integer<'a>() -> Parser<'a,u8, i32>{
    (one_of(b"123456789") + one_of(b"0123456789")
        .repeat(0..))
        .collect()
        .convert(str::from_utf8)
        .convert(|s|{i32::from_str(s)})
}

//parses a number into an ExpressionTree::Constant
fn constant<'a>() -> Parser<'a,u8, ExpressionTree>{
        integer().map(|i| {ExpressionTree::Constant(i)})
}

//matches dice notation (both English and Swedish variants) and returns an ExpressionTree::Uniform
fn die<'a>() -> Parser<'a,u8,ExpressionTree>{
    (positive_integer().opt() - one_of(b"dt") + positive_integer())
    .map(|(n,d)| {
        match n {
            // For example 2d8
            Some(i) if i > 1 => {
                ExpressionTree::Sum(vec!(ExpressionTree::Uniform(1,d);i as usize))
            }
            // 1d6 or d10
            Some(_) | None => {
                ExpressionTree::Uniform(1,d)
            }
        }
    })
}

//unitary minus
fn negative<'a>() -> Parser<'a,u8,ExpressionTree> {
    (sym(b'-') * space().opt() * call(expression)).map(|e|ExpressionTree::Negative(Box::new(e)))
}

//sum
fn sum<'a>() -> Parser<'a,u8,ExpressionTree> {
    (leaf() - sym(b'+') + call(expression))
        .map(|(e,f)| ExpressionTree::Sum(vec!(e,f)))
}
//difference
fn difference<'a>() -> Parser<'a,u8,ExpressionTree> {
    (leaf() - sym(b'-') + call(expression))
        .map(|(e,f)| ExpressionTree::Sum(vec!(e,ExpressionTree::Negative(Box::new(f)))))
}

//non compound expression trees (no sums)
fn leaf<'a>() -> Parser<'a,u8,ExpressionTree> {
    space().opt() *
    ( die()
    | parenthesis()
    | constant())
    -space().opt()
}

fn parenthesis<'a>() -> Parser<'a,u8,ExpressionTree> {
    sym(b'(') * space().opt() * call(expression) - space().opt() - sym(b')')
}

fn expression<'a>() -> Parser<'a,u8,ExpressionTree> {
    space() *
    ( sum()
    | difference()
    | negative()
    | leaf()
    ) - space().opt()
}

pub fn dice_parser<'a>() -> Parser<'a,u8,ExpressionTree> {
    expression() - end()
}



pub fn parse(s:String) -> Result<i32,&'static str> {
    let p = dice_parser();
    if let Ok(p) = p.parse(s.as_bytes()){
        Ok(p.eval())
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
        assert_eq!(Ok(super::ExpressionTree::Constant(2)),parser.parse(input));
    }
    #[test]
    fn parse_die(){
        let parser = super::die();
        let input = b"2d8";
        assert_eq!(Ok(super::ExpressionTree::Sum(vec!(super::ExpressionTree::Uniform(1,8),super::ExpressionTree::Uniform(1,8)))),parser.parse(input));
    }
    #[test]
    fn parse_negative(){
        let parser = super::negative();
        let input = b"-d6";
        assert_eq!(Ok(super::ExpressionTree::Negative(Box::new(super::ExpressionTree::Uniform(1,6)))),parser.parse(input));
    }

}
