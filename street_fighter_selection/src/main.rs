#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    println!("Street fighter - character selection!");
    street_fighter_selection(
        &[
            ["Ryu", "E.Honda", "Blanka", "Guile", "Balrog", "Vega"],
            ["Ken", "Chun Li", "Zangief", "Dhalsim", "Sagat", "M.Bison"],
        ],
        &[0, 0],
        &[
            Direction::Up,
            Direction::Left,
            Direction::Right,
            Direction::Left,
            Direction::Left,
        ],
    );
}

fn street_fighter_selection(
    fighters: &[[&str; 6]; 2],
    position: &[i64; 2],
    moves: &[Direction],
) -> Vec<String> {
    let mut hovered_characters: Vec<String> = Vec::new();
    // println!("fighter grid {:?}", fighters);
    let mut position_ref = (position[0].clone(), position[1].clone());
    for current_move in moves {
        apply_move(current_move, &mut position_ref, fighters[0].len() as i64);
        // println!("Current position => {:?}", position_ref);
        hovered_characters.push(String::from(
            fighters[position_ref.0 as usize][position_ref.1 as usize],
        ));
    }
    // println!("Hovered characters => {:?}", hovered_characters);
    hovered_characters
}

fn apply_move(control_move: &Direction, position: &mut (i64, i64), row_length: i64) {
    match control_move {
        Direction::Up => {
            if position.0 == 1 {
                position.0 = 0
            }
        }
        Direction::Down => {
            if position.0 == 0 {
                position.0 = 1
            }
        }
        Direction::Left => {
            if position.1 == 0 {
                position.1 = row_length - 1;
            } else {
                position.1 -= 1;
            }
        }
        Direction::Right => {
            if position.1 == row_length - 1 {
                position.1 = 0;
            } else {
                position.1 += 1;
            }
        }
    }
}

#[cfg(test)]

const FIGHTERS: [[&str; 6]; 2] = [
    ["Ryu", "E.Honda", "Blanka", "Guile", "Balrog", "Vega"],
    ["Ken", "Chun Li", "Zangief", "Dhalsim", "Sagat", "M.Bison"],
];

#[test]
fn few_moves() {
    let moves = [
        Direction::Up,
        Direction::Left,
        Direction::Right,
        Direction::Left,
        Direction::Left,
    ];

    assert_eq!(
        street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
        ["Ryu", "Vega", "Ryu", "Vega", "Balrog"],
    );
}

#[test]
fn no_moves() {
    let moves: [Direction; 0] = [];

    assert_eq!(
        street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
        [] as [String; 0]
    );
}

#[test]
fn moving_left() {
    use Direction::*;
    let moves = [Left, Left, Left, Left, Left, Left, Left, Left];

    assert_eq!(
        street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
        ["Vega", "Balrog", "Guile", "Blanka", "E.Honda", "Ryu", "Vega", "Balrog"],
    );
}

#[test]
fn moving_right() {
    use Direction::*;
    let moves = [Right, Right, Right, Right, Right, Right, Right, Right];

    assert_eq!(
        street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
        ["E.Honda", "Blanka", "Guile", "Balrog", "Vega", "Ryu", "E.Honda", "Blanka"],
    );
}

#[test]
fn uses_all_4_directions_clockwise_twice() {
    use Direction::*;
    let moves = [Up, Left, Down, Right, Up, Left, Down, Right];

    assert_eq!(
        street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
        ["Ryu", "Vega", "M.Bison", "Ken", "Ryu", "Vega", "M.Bison", "Ken"],
    );
}

#[test]
fn always_moving_down() {
    use Direction::*;
    let moves = [Down, Down, Down, Down];

    assert_eq!(
        street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
        ["Ken", "Ken", "Ken", "Ken"],
    );
}

#[test]
fn always_moving_up() {
    use Direction::*;
    let moves = [Up, Up, Up, Up];

    assert_eq!(
        street_fighter_selection(&FIGHTERS, &[0, 0], &moves),
        ["Ryu", "Ryu", "Ryu", "Ryu"],
    );
}
