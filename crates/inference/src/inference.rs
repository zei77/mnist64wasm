use crate::conv1::conv1;
use crate::early_head::{early_scores, early_decision};

pub struct InferenceResult {
    pub class: usize,
    pub scores: [u16; 10],
    pub early: bool,
}

pub fn infer(plane0: &[u32; 24], plane1: &[u32; 24]) -> InferenceResult {
    let activations1 = conv1(plane0, plane1);
    let scores = early_scores(&activations1);
    let (class, passed) = early_decision(&scores);

    return InferenceResult {
        class,
        scores: scores,
        early: passed,
    };
}
