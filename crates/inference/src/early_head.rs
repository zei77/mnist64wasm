use crate::params::{
    EARLY_WEIGHTS as WEIGHTS,
    EARLY_THRESHOLDS as THRESHOLDS
};

pub fn early_scores(activations: &[u64; 50]) -> [u16; 10] {
    let mut scores = [0u16; 10];

    for class in 0..scores.len() {
        for i in 0..activations.len() {
            scores[class] += (!(activations[i] ^ WEIGHTS[class][i])).count_ones() as u16;
        }
    }

    scores
}

pub fn early_decision(scores: &[u16; 10]) -> (usize, bool) {
    let mut best_class = 0;
    let mut best = scores[0];
    let mut second = 0u16;

    for class in 1..scores.len() {
        let score = scores[class];

        if score >= best {
            second = best;
            best = score;
            best_class = class;
        } else if score >= second {
            second = score;
        }
    }

    // asm: scales scores by 2 before calculating the margin.
    let margin = (best - second) * 2u16;

    if margin >= THRESHOLDS[best_class] as u16 {
        (best_class, true)
    } else {
        (usize::MAX, false)
    }
}
