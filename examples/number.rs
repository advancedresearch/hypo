extern crate hypo;

use hypo::*;

/// Represents a hypothesis about numbers.
#[derive(Debug)]
pub enum Hyp {
    Le(u8),
    Not(Box<Hyp>),
    And(Box<Hyp>, Box<Hyp>),
    Or(Box<Hyp>, Box<Hyp>),
}

impl Hyp {
    /// Predicts answer given a value.
    pub fn predict(&self, val: u8) -> bool {
        match *self {
            Hyp::Le(v) => val <= v,
            Hyp::Not(ref hyp) => !hyp.predict(val),
            Hyp::And(ref a, ref b) => a.predict(val) && b.predict(val),
            Hyp::Or(ref a, ref b) => a.predict(val) || b.predict(val),
        }
    }
}

fn main() {
    let ref mut hypos = vec![
        Hyp::Le(0),
        Hyp::Le(1),
        Hyp::Le(2),
        Hyp::Le(3),
        Hyp::Le(4),
        Hyp::Le(5),
        Hyp::Le(6),
        Hyp::Le(7),
        Hyp::Le(8),
        Hyp::Le(9),
        Hyp::Not(Box::new(Hyp::Le(3))),
        Hyp::Not(Box::new(Hyp::Not(Box::new(Hyp::Le(3))))),
        Hyp::And(Box::new(Hyp::Le(5)), Box::new(Hyp::Not(Box::new(Hyp::Le(3))))),
        Hyp::And(Box::new(Hyp::Le(6)), Box::new(Hyp::Not(Box::new(Hyp::Le(3))))),
        Hyp::And(Box::new(Hyp::Le(7)), Box::new(Hyp::Not(Box::new(Hyp::Le(3))))),
        Hyp::Or(Box::new(Hyp::Le(2)), Box::new(Hyp::Not(Box::new(Hyp::Le(5)))))
    ];

    let mut i = 0;
    experiment(255, hypos, &mut |n| {
        i += 1;
        n <= 2 || n > 5
    }, |hyp, val| hyp.predict(val as u8));
    println!("{:?}", hypos);
    println!("{}", i);
}
