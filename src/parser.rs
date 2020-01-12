use rand::distributions::{Uniform,Distribution};
use pom::parser::*;
use std::str::{self, FromStr};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpressionTree{
    Sum(Vec<ExpressionTree>),
    Mult(i64,Box<ExpressionTree>),
    Negative(Box<ExpressionTree>),
    Constant(i64),
    Uniform(i64,i64)
}

impl ExpressionTree {
    pub fn eval(&self) -> i64{
        match self{
            ExpressionTree::Constant(i) => {*i},
            ExpressionTree::Uniform(i,j) => {
                let dice = Uniform::new_inclusive(i,j);
                let mut rng = rand::thread_rng();
                dice.sample(&mut rng) as i64
            },
            ExpressionTree::Sum(v) => {
                v.iter().fold(0i64,|acc,x| acc+x.eval())
            },
            ExpressionTree::Mult(n,e) => {
                (0 .. *n).fold(0, |acc,_| acc + e.eval())
            }
            ExpressionTree::Negative(e) => {
                0 - e.eval()
            }
        }
    }
    pub fn trivial(&self) -> bool{
        match self{
            ExpressionTree::Constant(_) =>
                true,
            _ => false
        }
    }

    pub fn bounds(&self) -> (i64,i64){
        match self {
            ExpressionTree::Constant(i) => (*i,*i),
            ExpressionTree::Uniform(i,j) => (*i,*j),
            ExpressionTree::Sum(v) => {
                v.iter().fold((0i64,0i64),|(lower,upper),x| {
                    let (l,u) = x.bounds();
                    (lower + l, upper + u)
                })
            },
            ExpressionTree::Mult(n,e) =>{
                let (l,u) = e.bounds();
                (n*l,n*u)
            }
            ExpressionTree::Negative(e) => {
                let (l,u) = e.bounds();
                (-u,-l)
            }
        }
    }
    pub fn size(&self) -> i64{
        match self {
            ExpressionTree::Sum(v) => {
                v.iter().fold(0i64,|acc,x| acc+x.size())
            },
            ExpressionTree::Mult(n,e) =>{
                n*e.size()
            }
            _ => 1
        }
    }
}

//matches whitespace
fn space<'a>() -> Parser<'a,u8,()> {
    one_of(b" \t\r\n").repeat(0..).discard()
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
                ExpressionTree::Mult(i,Box::new(ExpressionTree::Uniform(1,d)))
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

pub fn simple_dice_parser<'a>() -> Parser<'a,u8,ExpressionTree> {
    expression() - end()
}

pub fn parse(s:String) -> Result<ExpressionTree,&'static str> {
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
        assert_eq!(Ok(super::ExpressionTree::Constant(2)),parser.parse(input));
    }
    #[test]
    fn parse_die(){
        let parser = super::die();
        let input = b"2d8";
                assert_eq!(Ok(super::ExpressionTree::Mult(2, Box::new(super::ExpressionTree::Uniform(1, 8)))),parser.parse(input));
    }

    #[test]
    fn parse_negative(){
        let parser = super::negative();
        let input = b"-d6";
        assert_eq!(Ok(super::ExpressionTree::Negative(Box::new(super::ExpressionTree::Uniform(1,6)))),parser.parse(input));
    }

}
