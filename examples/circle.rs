use hypo::*;

#[derive(Debug)]
pub struct Circle {
    pub pos: [f64; 2],
    pub rad: f64,
}

impl Circle {
    pub fn inside(&self, pos: [f64; 2]) -> bool {
        let dx = pos[0] - self.pos[0];
        let dy = pos[1] - self.pos[1];
        dx * dx + dy * dy < self.rad * self.rad
    }
}

fn main() {
    let mut hypotheses = vec![
        Circle {pos: [0.0, 0.0], rad: 1.0},
        Circle {pos: [1.0, 0.0], rad: 1.0},
        Circle {pos: [0.0, 0.0], rad: 2.0},
        Circle {pos: [0.2, 0.0], rad: 1.0},
        Circle {pos: [1.2, 0.0], rad: 1.0},
        Circle {pos: [0.2, 0.0], rad: 2.0},
    ];

    let answer = Circle {pos: [0.0, 0.0], rad: 1.0};

    let guesses = vec![
        [0.0, 0.0],
        [0.1, 0.0],
        [1.5, 0.0],
        [3.0, 0.0],
        [0.0, 2.0],
        [0.0, 3.0],
        [5.0, 0.0],
        [1.1, 0.0],
    ];

    println!("Correlation count g0, g1: {}", guess_count_correlation(0, 1, &hypotheses, |h, n| h.inside(guesses[n])));
    println!("");

    println!("Guess fitness:");
    for i in 0..guesses.len() {
        println!("g{} = {}", i, guess_fitness(i, &hypotheses, |h, n| h.inside(guesses[n])));
    }
    println!("");

    experiment(guesses.len(), &mut hypotheses, |n| answer.inside(guesses[n]), |h, n| h.inside(guesses[n]));

    println!("Remaining hypotheses:");
    for i in 0..hypotheses.len() {
        println!("{:?}", hypotheses[i]);
    }
}


