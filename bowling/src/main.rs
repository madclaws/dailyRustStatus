use bowling::*;

fn main() {
    let mut bowling = BowlingGame::new();
    bowling.roll(3);
    bowling.roll(2);

    bowling.roll(3);
    bowling.roll(2);

    bowling.roll(3);
    bowling.roll(2);

    bowling.roll(3);
    bowling.roll(2);

    bowling.roll(3);
    bowling.roll(2);

    bowling.roll(3);
    bowling.roll(2);

    bowling.roll(3);
    bowling.roll(2);

    bowling.roll(3);
    bowling.roll(2);

    bowling.roll(3);
    bowling.roll(2);

    bowling.roll(3);
    bowling.roll(2);
    println!("Bowling score {:?}", bowling.score());
}