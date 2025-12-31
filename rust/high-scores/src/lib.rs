#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if let Some(a) = self.scores.last() {
            Some(*a)
        } else {
            None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if let Some(best) = self.scores.iter().max() {
            Some(*best)
        } else {
            None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut array: Vec<u32> = self.scores.iter().map(|number| *number).collect();
        array.sort();
        array.reverse();
        if let Some(array) = array.chunks(3).next() {
            array.to_vec()
        } else {
            vec![]
        }
    }
}
