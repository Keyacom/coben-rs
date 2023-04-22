use super::chance::Chance;
use super::types::*;
use super::utils::{fsum, ndigits};
use super::super::cli::arg::CliOptions;
use clap::Parser;
use expandtabs_rs::StringExt;
use statrs::statistics::Statistics;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Calculator {
    scores: ScoreList,
    scores_next_event: ScoreList,
    chances: ChanceList,
    immunity_req: ImmunityReq,
}

impl Calculator {
    pub fn calculate_immunity_req(
        scores: &ScoreList,
        scores_next_event: &ScoreList,
    ) -> ImmunityReq {
        let float_sne: Vec<f64> = scores_next_event.iter().map(|&e| e as f64).collect();
        let float_scores: Vec<f64> = scores.iter().map(|&e| e as f64).collect();
        float_sne.mean() + float_scores.mean()
    }
    pub fn new(scores: ScoreList, scores_next_event: ScoreList) -> Result<Self, String> {
        let immunity_req = Self::calculate_immunity_req(&scores, &scores_next_event);
        let c = Self {
            scores,
            scores_next_event,
            chances: vec![],
            immunity_req,
        };
        c.validate(false).map(|_| c)
    }
    fn validate(&self, check_chances: bool) -> Result<(), String> {
        if self.scores.is_empty() {
            return Err("List of current scores must have a nonzero length".to_string());
        }
        if self.scores.len() != self.scores_next_event.len() {
            return Err(
                "List of current scores and list of scores for next event must have equal length"
                    .to_string(),
            );
        }
        if check_chances && self.scores.len() != self.chances.len() {
            return Err(
                "List of current scores and list of chances must have equal length".to_string(),
            );
        }
        Ok(())
    }
    pub fn calculate(&mut self) {
        let not_immune_scores: Vec<f64> = self
            .scores
            .iter()
            .map(|&e| {
                if e as f64 > self.immunity_req {
                    f64::NAN
                } else {
                    e.into()
                }
            })
            .collect();

        let inv_difs: Vec<f64> = not_immune_scores
            .iter()
            .map(|&e| 1.0 / e - 1.0 / self.immunity_req)
            .collect();

        let nums: Vec<f64> = inv_difs.iter().filter(|&e| !e.is_nan()).copied().collect();
        let sum = fsum(&nums);

        for (p, s) in self.scores.iter().enumerate() {
            let score = *s as f64;
            if score > self.immunity_req {
                self.chances
                    .push(Chance::Immunity((score / self.immunity_req).floor() as u8));
            } else {
                // No NaN check in multiplication: NaN only if s > self.immunity_req
                // fsum argument still has NaN check
                self.chances.push(Chance::Coben(100.0 * inv_difs[p] / sum));
            }
        }
        self.validate(true).unwrap();
    }
    // pub fn scores(&self) -> ScoreList {
    //     self.scores.clone()
    // }
    // pub fn scores_next_event(&self) -> ScoreList {
    //     self.scores_next_event.clone()
    // }
    // pub fn chances(&self) -> ChanceList {
    //     self.chances.clone()
    // }
    // pub fn immunity_req(&self) -> ImmunityReq {
    //     self.immunity_req
    // }
}

impl Display for Calculator {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut v: Vec<String> = vec![];
        for i in 0..self.chances.len() {
            let score = self.scores[i];
            v.push(
                format!("{}\t{}", score, self.chances[i].display_with_opts(CliOptions::parse()))
                    .expand_tabs(ndigits(score.into()) as u16 + 3)
                    .to_string(),
            );
        }
        write!(f, "{}", v.join("\n"))
    }
}
