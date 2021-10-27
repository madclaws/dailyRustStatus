use high_scores::*;

fn main() {
    let high_score = HighScores::new(&[30, 50, 20, 70]);
    println!("{:?}", high_score.personal_top_three())
}
