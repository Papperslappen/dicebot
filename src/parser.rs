use pom::parser::*;
use std::str::{self, FromStr};

use rand::distributions::{Uniform,Distribution};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DiceExpressionTree{
    Many(Vec<DiceExpressionTree>),
    Sum(Box<DiceExpressionTree>),
    Mult(i64,Box<DiceExpressionTree>),
    Product(Box<DiceExpressionTree>),
    Negative(Box<DiceExpressionTree>),
    Constant(i64),
    Die(i64)
}

pub type EvaluationResult = Vec<i64>;

impl DiceExpressionTree {
    pub fn roll(&self) -> EvaluationResult{
        match self{
            DiceExpressionTree::Constant(i) => {vec![*i]},
            DiceExpressionTree::Die(sides) => {
                let dice = Uniform::new_inclusive(1,sides);
                let mut rng = rand::thread_rng();
                vec![dice.sample(&mut rng) as i64]
            },
            DiceExpressionTree::Sum(e) => {
                vec![e.roll().iter().fold(0i64,|acc,x| acc+x)]
            },
            DiceExpressionTree::Product(e) => {
                vec![e.roll().iter().fold(1i64,|acc,x| acc*x)]
            },
            DiceExpressionTree::Mult(n,e) => {
                //e.roll().iter().map(|x| n*x).collect()
                (1 .. *n).fold(e.roll(), |acc,_| acc.iter().zip(e.roll().iter()).map(|(x,y)| x+y).collect())

            }
            DiceExpressionTree::Negative(e) => {
                e.roll().iter().map(|x| 0-x).collect()
            }
            DiceExpressionTree::Many(v) => {
                v.iter().map(|x| x.roll()).flatten().collect()
            }
        }
    }
    pub fn trivial(&self) -> bool{
        match self{
            DiceExpressionTree::Constant(_) =>
                true,
            _ => false
        }
    }

    // pub fn bounds(&self) -> (i64,i64){
    //     match self {
    //         DiceExpressionTree::Constant(i) => (*i,*i),
    //         DiceExpressionTree::Die(i) => (1,*j),
    //         DiceExpressionTree::Sum(v) => {
    //             v.iter().fold((0i64,0i64),|(lower,upper),x| {
    //                 let (l,u) = x.bounds();
    //                 (lower + l, upper + u)
    //             })
    //         },
    //         DiceExpressionTree::Mult(n,e) =>{
    //             let (l,u) = e.bounds();
    //             (n*l,n*u)
    //         }
    //         DiceExpressionTree::Negative(e) => {
    //             let (l,u) = e.bounds();
    //             (-u,-l)
    //         }
    //     }
    // }
    pub fn size(&self) -> i64{
        match self {
            DiceExpressionTree::Many(v) => {
                v.iter().fold(0i64,|acc,x| acc+x.size())
            }
            DiceExpressionTree::Sum(e) => {
                e.size()
            },
            DiceExpressionTree::Mult(n,e) =>{
                n*e.size()
            }
            _ => 1
        }
    }
}

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

//parses a number into an DiceExpressionTree::Constant
fn constant<'a>() -> Parser<'a,u8, DiceExpressionTree>{
        integer().map(|i| {DiceExpressionTree::Constant(i)})
}

//matches dice notation (both English and Swedish variants) and returns an DiceExpressionTree::Uniform
fn die<'a>() -> Parser<'a,u8,DiceExpressionTree>{
    (positive_integer().opt() - one_of(b"dt") + positive_integer())
    .map(|(n,d)| {
        match n {
            // For example 2d8
            Some(i) if i > 1 => {
                DiceExpressionTree::Mult(i,Box::new(DiceExpressionTree::Die(d)))
            }
            // 1d6 or d10
            Some(_) | None => {
                DiceExpressionTree::Die(d)
            }
        }
    })
}

//unary minus
fn negative<'a>() -> Parser<'a,u8,DiceExpressionTree> {
    (sym(b'-') * space().opt() * call(expression)).map(|e|DiceExpressionTree::Negative(Box::new(e)))
}

//sum
fn sum<'a>() -> Parser<'a,u8,DiceExpressionTree> {
    (leaf() - sym(b'+') + call(expression))
        .map(|(e,f)| DiceExpressionTree::Sum(Box::new(DiceExpressionTree::Many(vec!(e,f)))))
}
//difference
fn difference<'a>() -> Parser<'a,u8,DiceExpressionTree> {
    (leaf() - sym(b'-') + call(expression))
        .map(|(e,f)| DiceExpressionTree::Sum(Box::new(DiceExpressionTree::Many(vec!(e,DiceExpressionTree::Negative(Box::new(f)))))))
}

//non compound expression trees (no sums)
fn leaf<'a>() -> Parser<'a,u8,DiceExpressionTree> {
    space().opt() *
    ( repeat_many()
    | die()
    | parenthesis()
    | constant())
    -space().opt()
}

fn parenthesis<'a>() -> Parser<'a,u8,DiceExpressionTree> {
    sym(b'(') * space().opt() * call(expression) - space().opt() - sym(b')')
}

// multiple expressions separated by whitespace
fn many<'a>() -> Parser<'a,u8,DiceExpressionTree> {
    (leaf() - sym(b',') - space().opt() + call(expression))
        .map(|(u,v)| DiceExpressionTree::Many(vec!(u,v)))
}

// repeat expression n times
fn repeat_many<'a>() -> Parser<'a,u8,DiceExpressionTree> {
    (positive_integer() - sym(b'.') + call(leaf))
        .map(|(i,e)| {
                DiceExpressionTree::Many((0..i).map(|_| e.clone()).collect())
        })
}

fn expression<'a>() -> Parser<'a,u8,DiceExpressionTree> {
    space().opt() *
    ( many()
    | sum()
    | difference()
    | negative()
    | leaf()
    ) - space().opt()
}

pub fn simple_dice_parser<'a>() -> Parser<'a,u8,DiceExpressionTree> {
    expression() - end()
}

pub fn parse(s:String) -> Result<DiceExpressionTree,&'static str> {
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
        assert_eq!(Ok(super::DiceExpressionTree::Constant(2)),parser.parse(input));
    }

}
