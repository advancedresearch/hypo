#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

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
        let mut count = 0;
        for hy in hypos {
            if f(hy, i) {count += 1}
        }
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
