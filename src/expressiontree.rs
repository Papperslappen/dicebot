use rand::distributions::{Uniform,Distribution};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DiceExpression{
    Many(Vec<DiceExpression>),
    Sum(Box<DiceExpression>),
    Product(Box<DiceExpression>),
    Negative(Box<DiceExpression>),
    Constant(i64),
    Die(i64)
}

use DiceExpression::*;

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
            }
            Many(v) => {
                v.iter().map(|x| x.roll()).flatten().collect()
            }
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
            Many(v) => {
                v.iter().fold(0i64,|acc,x| acc+x.size())
            }
            Sum(e) | DiceExpression::Product(e) => {
                e.size()
            },
            _ => 1
        }
    }

    pub fn sum(self) -> DiceExpression {
        Sum(Box::new(self))
    }

    pub fn add(self, other : DiceExpression) -> DiceExpression {
        Sum(Box::new(Many(vec!(self,other))))
    }

    pub fn negate(self) -> DiceExpression{
        Negative(Box::new(self))
    }

    pub fn subtract(self, other : DiceExpression) -> DiceExpression {
        self.add(other.negate())
    }

    pub fn multiply(self, other : DiceExpression) -> DiceExpression {
        self.add(other.negate())
    }

    pub fn also(self, other : DiceExpression) -> DiceExpression {
        Many(vec!(self,other))
    }

    pub fn repeat(self, times: i64) -> DiceExpression {
        if times>1 {
            Many((0..times).map(|_|self.clone()).collect())
        } else {
            self
        }
    }

}
