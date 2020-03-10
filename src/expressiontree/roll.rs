use rand::distributions::{Uniform,Distribution};
use itertools::Itertools;

pub type Roll = Vec<i64>;
use super::*;

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
}
