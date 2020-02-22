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
