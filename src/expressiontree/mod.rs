use std::fmt;
pub mod roll;
pub mod probability;
use serde::Serialize;

#[derive(Serialize,Debug, PartialEq, Eq, Clone)]
pub enum DiceExpression{
    Many(Vec<DiceExpression>),
    Sum(Box<DiceExpression>),
    Max(Box<DiceExpression>),
    Min(Box<DiceExpression>),
    Add(Box<DiceExpression>,Box<DiceExpression>),
    Multiply(Box<DiceExpression>,Box<DiceExpression>),
    Equal(Box<DiceExpression>,Box<DiceExpression>),
    LessThan(Box<DiceExpression>,Box<DiceExpression>),
    Negative(Box<DiceExpression>),
    Constant(i64),
    Die(i64),
    DieOutcome(i64,i64)
}

use DiceExpression::*;

impl From<i64> for DiceExpression {
    fn from(val: i64) -> Self {
        Constant(val)
    }
}

impl fmt::Display for DiceExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Constant(i) => {write!(f,"{}",i)},
            Die(sides) => {
                write!(f,"d{}",sides)
            },
            Sum(e) => {
                match &**e { //?????
                    Many(v) => write!(f,"{}",v.iter().map(|x| format!("{}",x)).collect::<Vec<String>>().join(" + ")),
                    _ => write!(f,"sum({})",e)
                }

            },
            Negative(e) => {
                if e.size() > 1 {
                    write!(f,"-({})",e)
                } else {
                    write!(f,"-{}",e)
                }
            },
            Max(e) => {
                write!(f,"max({})",e)
            }
            Min(e) => {
                write!(f,"min({})",e)
            }
            Add(left,right) => {
                write!(f,"{} + {}",left,right)
            },
            Multiply(left,right) => {
                if right.size() > 1 {
                    write!(f,"{} * ({})",left,right)
                }else{
                    write!(f,"{} * {}",left,right)
                }
            },
            Equal(left,right) => {
                let mut form = String::from("");
                if left.size() > 1 {
                    form = form + &format!("({})",left);
                }else {
                    form = form + &format!("{}",left);
                }
                form = form + " = ";
                if right.size() > 1 {
                    form = form + &format!("({})",right);
                }else {
                    form = form + &format!("{}",right);
                }
                write!(f,"{}",form)
            },
            LessThan(left,right) => {
                let mut form = String::from("");
                if left.size() > 1 {
                    form = form + &format!("({})",left);
                }else {
                    form = form + &format!("{}",left);
                }
                form = form + " < ";
                if right.size() > 1 {
                    form = form + &format!("({})",right);
                }else {
                    form = form + &format!("{}",right);
                }
                write!(f,"{}",form)
            },
            Many(v) => {
                write!(f,"{}",v.iter().fold(String::new(),|acc,val| acc+format!("{}",val).as_str()+",").trim_end_matches(','))
            },
            DieOutcome(sides,result) => {
                write!(f,"(d{}):{}",sides,result)
            }
        }
    }
}

impl DiceExpression {
    // Return true if only a single constant
    pub fn trivial(&self) -> bool{
        match self{
            Constant(_) =>
                true,
            _ => false
        }
    }

    // Return the number of elements
    pub fn size(&self) -> usize{
        match self {
            Sum(e) | Negative(e) | Min(e) | Max(e) => {
                1+e.size()
            },
            Many(v) => {
                v.iter().fold(0,|acc,x| acc+x.size())
            }
            Add(l,r)
            | Multiply(l,r)
            | Equal(l,r)
            | LessThan(l,r) => {
                1+l.size() + r.size()
            },
            Constant(_) => 1,
            Die(_) | DieOutcome(_,_) => 1
        }
    }

    // Return the number of elements
    pub fn number_of_rolls(&self) -> usize{
        match self {
            Sum(e) | Negative(e) | Min(e) | Max(e) => {
                e.number_of_rolls()
            },
            Many(v) => {
                v.iter().fold(0,|acc,x| acc+x.number_of_rolls())
            }
            Add(l,r)
            | Multiply(l,r)
            | Equal(l,r)
            | LessThan(l,r) => {
                l.number_of_rolls() + r.number_of_rolls()
            },
            Constant(_) => 0,
            Die(_) | DieOutcome(_,_) => 1
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
