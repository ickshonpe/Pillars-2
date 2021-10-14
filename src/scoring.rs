use ron::de::from_reader;
use ron::ser::to_writer;
use std::{error::Error, path::Path};

use serde::{Deserialize, Serialize};

pub fn calculate_points(matched_blocks: u64, level: u64) -> u64 {
    let multiplier = (matched_blocks as f64).log(1.4) + (level as f64).log(1.3);
    let points = 5.0 * matched_blocks as f64 * multiplier;
    points.ceil() as u64
}

pub const HIGH_SCORE_PATH: &str = "high_scores";

pub fn load_high_scores_table<P: AsRef<Path>>(
    high_scores_path: P,
) -> Result<HighScoresTable<HighScore>, Box<dyn Error>> {
    let high_scores_file = std::fs::File::open(high_scores_path)?;
    let high_scores_table = from_reader(high_scores_file)?;
    Ok(high_scores_table)
}

pub fn save_high_score_table<P: AsRef<Path>>(
    high_scores_path: P,
    high_score_table: &HighScoresTable<HighScore>,
) -> Result<(), Box<dyn Error>> {
    let high_score_file = std::fs::File::create(high_scores_path)?;
    to_writer(high_score_file, high_score_table)?;
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HighScore {
    pub score: u64,
    pub level: u64,
    pub handle: String,
}

impl Ord for HighScore {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for HighScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HighScoresTable<H>
where
    H: Ord + Clone,
{
    scores: Vec<H>,
    prev_score: Option<H>,
}

impl<H> HighScoresTable<H>
where
    H: Ord + Clone,
{
    pub fn new() -> Self {
        Self {
            scores: Vec::new(),
            prev_score: None,
        }
    }

    pub fn list(&self) -> &[H] {
        &self.scores
    }

    pub fn best(&self) -> Option<&H> {
        if self.scores.is_empty() {
            None
        } else {
            Some(&self.scores[0])
        }
    }

    pub fn find_position(&self, h: &H) -> usize {
        for (idx, g) in self.scores.iter().enumerate() {
            if g < h {
                return idx;
            }
        }
        self.scores.len()
    }

    pub fn insert(&mut self, h: H) -> usize {
        let index = self.find_position(&h);
        self.prev_score = Some(h.clone());
        self.scores.insert(index, h);
        index
    }

    pub fn prev(&self) -> &Option<H> {
        &self.prev_score
    }
}

#[cfg(test)]
mod tests {
    use super::{load_high_scores_table, save_high_score_table, HighScore, HighScoresTable};

    #[test]
    fn loading_and_saves() {
        let h = HighScore {
            score: 11,
            level: 10,
            handle: "agr".to_owned(),
        };
        let mut t = HighScoresTable::new();
        t.insert(h);
        assert_eq!(t.best().unwrap().score, 11);
        let _ = save_high_score_table("h_file", &t);
        let u = load_high_scores_table("h_file").unwrap();
        assert_eq!(u.best().unwrap().score, 11);
    }
}
