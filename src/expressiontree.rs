use std::fmt;

use rand::distributions::{Uniform,Distribution};
use itertools::Itertools;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DiceExpression{
    Many(Vec<DiceExpression>),
    Sum(Box<DiceExpression>),
    Product(Box<DiceExpression>),
    Max(Box<DiceExpression>),
    Min(Box<DiceExpression>),
    Add(Box<DiceExpression>,Box<DiceExpression>),
    Multiply(Box<DiceExpression>,Box<DiceExpression>),
    Equal(Box<DiceExpression>,Box<DiceExpression>),
    LessThan(Box<DiceExpression>,Box<DiceExpression>),
    Negative(Box<DiceExpression>),
    Constant(i64),
    Die(i64),
    Outcome(i64,i64)
}

use DiceExpression::*;


impl fmt::Display for DiceExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Constant(i) => {write!(f,"{}",i)},
            Die(sides) => {
                write!(f,"d{}",sides)
            },
            Sum(e) => {
                write!(f,"sum{}",e)
            },
            Product(e) => {
                write!(f,"prod{}",e)
            },
            Negative(e) => {
                write!(f,"-{}",e)
            },
            Max(e) => {
                write!(f,"max {}",e)
            }
            Min(e) => {
                write!(f,"min {}",e)
            }
            Add(left,right) => {
                write!(f,"{} + {}",left,right)
            },
            Multiply(left,right) => {
                write!(f,"{} * {}",left,right)
            },
            Equal(left,right) => {
                write!(f,"{} = {}",left,right)
            },
            LessThan(left,right) => {
                write!(f,"{} < {}",left,right)
            },
            Many(v) => {
                write!(f,"({})",v.iter().fold(String::new(),|acc,val| acc+format!("{}",val).as_str()+",").trim_end_matches(','))
            },
            Outcome(sides,result) => {
                write!(f,"d{}:{}",sides,result)
            }
        }
    }
}

pub type Roll = Vec<i64>;

impl DiceExpression {
    pub fn roll(&self) -> Roll{
        match self{
            Constant(i) => {vec![*i]},
            Die(sides) => {
                let dice = Uniform::new_inclusive(1,sides);
                let mut rng = rand::thread_rng();
                vec![dice.sample(&mut rng) as i64]
            },
            Sum(e) => {
                vec![e.roll().iter().fold(0i64,|acc,x| acc+x)]
            },
            Product(e) => {
                vec![e.roll().iter().fold(1i64,|acc,x| acc*x)]
            },
            Negative(e) => {
                e.roll().iter().map(|x| 0-x).collect()
            },
            Max(e) => {
                e.roll().iter().max().map_or(vec!(),|value| vec!(*value))
            }
            Min(e) => {
                e.roll().iter().min().map_or(vec!(),|value| vec!(*value))
            }
            Add(left,right) => {
                left.roll().iter().cartesian_product(right.roll().iter()).map(|(l,r)|l+r).collect()
            },
            Multiply(left,right) => {
                left.roll().iter().cartesian_product(right.roll().iter()).map(|(l,r)|l*r).collect()
            },
            Equal(left,right) => {
                left.roll().iter().cartesian_product(right.roll().iter()).map(|(l,r)| if l==r {1} else {0}).collect()
            },
            LessThan(left,right) => {
                left.roll().iter().cartesian_product(right.roll().iter()).map(|(l,r)| if l<r {1} else {0}).collect()
            },
            Many(v) => {
                v.iter().map(|x| x.roll()).flatten().collect()
            },
            Outcome(_,result) => vec!(*result)
        }
    }


    pub fn outcome(&self) -> DiceExpression {
        match self{
            Constant(i) => Constant(*i),
            Die(sides) => {
                let dice = Uniform::new_inclusive(1,sides);
                let mut rng = rand::thread_rng();
                Outcome(*sides,dice.sample(&mut rng) as i64)
            },
            Sum(e) => {
                Sum(Box::new(e.outcome()))
            },
            Product(e) => {
                Product(Box::new(e.outcome()))
            },
            Negative(e) => {
                Negative(Box::new(e.outcome()))
            },
            Max(e) => {
                Max(Box::new(e.outcome()))
            }
            Min(e) => {
                Min(Box::new(e.outcome()))
            }
            Add(left,right) => {
                Add(Box::new(left.outcome()),Box::new(right.outcome()))
            },
            Multiply(left,right) => {
                Multiply(Box::new(left.outcome()),Box::new(right.outcome()))
            },
            Equal(left,right) => {
                Equal(Box::new(left.outcome()),Box::new(right.outcome()))
            },
            LessThan(left,right) => {
                LessThan(Box::new(left.outcome()),Box::new(right.outcome()))
            },
            Many(v) => {
                Many(v.iter().map(|x| x.outcome()).collect())
            },
            Outcome(sides,result) => Outcome(*sides,*result)
        }
    }


    // Return true if only a single constant
    pub fn trivial(&self) -> bool{
        match self{
            Constant(_) =>
                true,
            _ => false
        }
    }

    // pub fn bounds(&self) -> (i64,i64){
    //     match self {
    //         DiceExpression::Constant(i) => (*i,*i),
    //         DiceExpression::Die(i) => (1,*j),
    //         DiceExpression::Sum(v) => {
    //             v.iter().fold((0i64,0i64),|(lower,upper),x| {
    //                 let (l,u) = x.bounds();
    //                 (lower + l, upper + u)
    //             })
    //         },
    //         DiceExpression::Mult(n,e) =>{
    //             let (l,u) = e.bounds();
    //             (n*l,n*u)
    //         }
    //         DiceExpression::Negative(e) => {
    //             let (l,u) = e.bounds();
    //             (-u,-l)
    //         }
    //     }
    // }


    // Return the number of elements
    pub fn size(&self) -> i64{
        match self {
            Sum(e) | Product(e) | Negative(e) | Min(e) | Max(e) => {
                e.size()
            },
            Many(v) => {
                v.iter().fold(0i64,|acc,x| acc+x.size())
            }
            Add(l,r)
            | Multiply(l,r)
            | Equal(l,r)
            | LessThan(l,r) => {
                l.size() * r.size()
            },
            _ => 1
        }
    }

    pub fn sum(self) -> DiceExpression {
        Sum(Box::new(self))
    }

    pub fn add(self, other : DiceExpression) -> DiceExpression {
        Add(Box::new(self),Box::new(other))
    }

    pub fn negate(self) -> DiceExpression{
        Negative(Box::new(self))
    }
    pub fn subtract(self, other : DiceExpression) -> DiceExpression {
        self.add(other.negate())
    }

    pub fn multiply(self, other : DiceExpression) -> DiceExpression {
        Multiply(Box::new(self),Box::new(other))
    }

    pub fn also(self, other : DiceExpression) -> DiceExpression {
        Many(vec!(self,other))
    }

    pub fn eq(self,other : DiceExpression) -> DiceExpression{
        Equal(Box::new(self),Box::new(other))
    }

    pub fn lt(self,other : DiceExpression) -> DiceExpression{
        LessThan(Box::new(self),Box::new(other))
    }

    pub fn gt(self,other : DiceExpression) -> DiceExpression{
        LessThan(Box::new(other),Box::new(self))
    }

    pub fn repeat(self, times: i64) -> DiceExpression {
        if times>1 {
            Many((0..times).map(|_|self.clone()).collect())
        } else {
            self
        }
    }

    pub fn max(self) -> DiceExpression {
        Max(Box::new(self))
    }

    pub fn min(self) -> DiceExpression {
        Min(Box::new(self))
    }

}
