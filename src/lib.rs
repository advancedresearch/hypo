#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

/// Counts hypotheses returning `true` for a guess.
pub fn guess_count<H, F>(i: usize, hypos: &[H], f: F) -> usize
    where F: Fn(&H, usize) -> bool
{
    let mut count = 0;
    for hy in hypos {
        if f(hy, i) {count += 1}
    }
    count
}

/// Counts hypotheses that matches two guesses.
pub fn guess_count_correlation<H, F>(i: usize, j: usize, hypos: &[H], f: F) -> usize
    where F: Fn(&H, usize) -> bool
{
    let mut count = 0;
    for hy in hypos {
        if f(hy, i) == f(hy, j) {count += 1}
    }
    count
}

/// Returns a fitness parameter less or equal to zero.
/// Zero is the optimal fitness for a guess to eliminate hypotheses.
pub fn guess_fitness<H, F>(i: usize, hypos: &[H], f: F) -> f64
    where F: Fn(&H, usize) -> bool
{
    -((hypos.len() / 2) as f64 - guess_count(i, hypos, f) as f64).powi(2)
}

/// Returns an optimal guess given a list of hypotheses.
///
/// An optimal guess is an experiment that is as close as possible
/// to eliminating half the hypotheses when being tested.
///
/// Here, the experiment is referred to by an index.
///
/// - `n` is the number of available experiments
/// - `f` is a function that makes a prediction from experiment
pub fn optimal_guess<H, F>(n: usize, hypos: &[H], f: F) -> Option<usize>
    where F: Fn(&H, usize) -> bool
{
    let mut min: Option<(usize, usize)> = None;
    let half = hypos.len() / 2;
    for i in 0..n {
        let count = guess_count(i, hypos, &f);
        let dist = if count > half {count - half} else {half - count};
        if min.is_none() || min.unwrap().0 > dist {
            min = Some((dist, i));
        }
    }
    min.map(|(_, b)| b)
}

/// Remove all hypotheses that predicted the wrong answer.
pub fn update<H, F>(hypos: &mut Vec<H>, val: usize, answer: bool, f: F)
    where F: Fn(&H, usize) -> bool
{
    for i in (0..hypos.len()).rev() {
        if f(&hypos[i], val) != answer {
            hypos.swap_remove(i);
        }
    }
}

/// Reduce number of hypotheses by repeatedly making optimal guesses for experiments.
pub fn experiment<H, F, P>(n: usize, hypos: &mut Vec<H>, mut f: F, p: P)
    where F: FnMut(usize) -> bool,
          P: Copy + Fn(&H, usize) -> bool
{
    loop {
        if let Some(guess) = optimal_guess(n, hypos, p) {
            let n = hypos.len();
            update(hypos, guess, f(guess), p);
            if hypos.len() == n {break}
        } else {break}
    }
}
