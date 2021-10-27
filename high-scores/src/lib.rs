#[derive(Debug)]
pub struct HighScores<'a>{
    score_list: &'a[u32]
}

impl <'a>HighScores<'a> {
    pub fn new(scores: &'a[u32]) -> Self {
        HighScores{score_list: scores}
    }

    pub fn scores(&self) -> &[u32] {
        self.score_list
    }

    pub fn latest(&self) -> Option<u32> {
        let mut last_one = self.scores().last();
        match last_one.as_mut() {
            Some(v) => Some(**v),
            None => None
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut slice = self.scores().to_vec();
        slice.sort_by(|a, b| b.cmp(a));
        if !slice.is_empty() {
            Some(slice[0])
        } else {
            None
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut slice = self.scores().to_vec();
        slice.sort_by(|a, b| b.cmp(a));
        let limit = u32::min(slice.len() as u32, 3) as usize;
        slice[..limit].to_vec()        
    }
}
