#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn set_x(&mut self, x: usize)  {
        self.x = x
    }
    pub fn set_y(&mut self, y: usize)  {
        self.y = y
    }
}

fn main() {
    println!("Super Street Fighter - Character selection");
    super_street_fighter_selection(
        &[
        &[       "",    "Ryu",  "E.Honda",  "Blanka",   "Guile", ""       ],
        &[ "Balrog",    "Ken",  "Chun Li", "Zangief", "Dhalsim", "Sagat"  ],
        &[   "Vega", "T.Hawk", "Fei Long",  "Deejay",   "Cammy", "M.Bison"],
    ], Position::new(0, 1),  &[Direction::Down, Direction::Down, Direction::Down, Direction::Down]
    );
}

fn super_street_fighter_selection(fighters: &[&[&str]],
    position: Position,
    moves: &[Direction]) -> Vec<String> {
        let mut hovered_characters: Vec<String> = Vec::new();
        let mut position_ref: Position = position;
        for current_move in moves {
            let current_x = position_ref.x;
            apply_move(current_move, &mut position_ref, &fighters[current_x]);
            hovered_characters.push(String::from(
                fighters[position_ref.x][position_ref.y]));
        }
        println!("Hovered characters => {:?}", hovered_characters);
        hovered_characters
}

fn apply_move(control_move: &Direction, position: &mut Position, current_row: &[&str]) {
    // handling empty elements.
    match control_move {
        Direction::Up => {
            if position.y > 0 && current_row[position.x] != "" {
                position.set_y(position.y - 1)
            }
        }
        Direction::Down => {
            if position.y < 1 && current_row[position.x] != "" {
                position.set_y(position.y + 1)
            }
        }
        Direction::Left => {
            position.set_x(get_valid_position_x(&position, current_row))
        }
        Direction::Right => {
            position.set_x(get_valid_position_x_right(&position, current_row))
        }
    }
}

fn get_valid_position_x(current_position: &Position, current_row: &[&str]) -> usize {
    let mut current_x: usize = current_position.x;
    for index in 0..current_row.len() - 1 {
        if current_row[index] == "" {
            current_x -= 1 
        }
        if current_x <= 0 {
            current_x = current_row.len() - 1
        }
    }
    println!("VALID X => {}", current_x);
    current_x
}

fn get_valid_position_x_right(current_position: &Position, current_row: &[&str]) -> usize {
    let mut current_x: usize = current_position.x;
    for index in 0..current_row.len() - 1 {
        if current_row[index] == "" {
            current_x += 1 
        }
        if current_x >= current_row.len() - 1 {
            current_x = 0
        }
    }
    current_x
}


#[cfg(test)]
    #[rustfmt::skip]
    const FIGHTERS_A: [&[&'static str]; 3] = [
        &[       "",    "Ryu",  "E.Honda",  "Blanka",   "Guile", ""       ],
        &[ "Balrog",    "Ken",  "Chun Li", "Zangief", "Dhalsim", "Sagat"  ],
        &[   "Vega", "T.Hawk", "Fei Long",  "Deejay",   "Cammy", "M.Bison"],
    ];
    
    #[rustfmt::skip]
    const FIGHTERS_B: [&[&'static str]; 3] = [
        &[       "",    "Ryu",  "E.Honda",  "Cammy",  "Blanka",   "Guile",        "", "Chun Li" ],
        &[ "Balrog",    "Ken",  "Chun Li",       "", "M.Bison", "Zangief", "Dhalsim", "Sagat"   ],
        &[   "Vega",       "", "Fei Long", "Balrog",  "Deejay",   "Cammy",        "", "T.Hawk"  ],
    ];
    
    #[rustfmt::skip]
    const FIGHTERS_C: [&[&'static str]; 6] = [
        &[        "",     "Ryu",  "E.Honda",  "Cammy" ],
        &[  "Balrog",     "Ken",  "Chun Li",       "" ],
        &[    "Vega",        "", "Fei Long", "Balrog",],
        &[  "Blanka",   "Guile",         "", "Chun Li"],
        &[ "M.Bison", "Zangief",  "Dhalsim", "Sagat"  ],
        &[  "Deejay",   "Cammy",         "", "T.Hawk" ],
    ];    
    
    #[test]
    fn no_selection() {
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 0), &[] as &[Direction]),
            vec![] as Vec<String>,
            "it should work with no selection cursor moves",
        );
    }
    
    #[test]
    fn empty_vertical_spaces_single_move() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Up]),
            vec!["Balrog"],
            "it should stop on empty spaces vertically",
        );
    }
    
    #[test]
    fn empty_vertical_spaces_multiple_moves() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Up, Up, Up, Up]),
            vec!["Balrog", "Balrog", "Balrog", "Balrog"],
            "it should stop on empty spaces vertically (up)",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(0, 1), &[Down, Down, Down, Down]),
            vec!["Vega", "Vega", "Vega", "Vega"],
            "it should stop on empty spaces vertically (down)",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(5, 1), &[Up, Up, Up, Up]),
            vec!["Sagat", "Sagat", "Sagat", "Sagat"],
            "it should stop on empty spaces vertically (up)",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(5, 1), &[Down, Down, Down, Down]),
            vec!["M.Bison", "M.Bison", "M.Bison", "M.Bison"],
            "it should stop on empty spaces vertically (down)",
        );        
    }
    
    #[test]
    fn rotate_horizontally() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(2, 0), &[Left, Left, Left, Left, Left, Left, Left, Left]),
            vec!["Ryu", "Guile", "Blanka", "E.Honda", "Ryu", "Guile", "Blanka", "E.Honda"],
            "it should rotate horizontally (left)",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(3, 1), &[Left, Left, Left, Left, Left, Left, Left, Left]),
            vec!["Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Chun Li", "Ken"],
            "it should rotate horizontally (left)",
        );        
    }
    
    #[test]
    fn rotate_horizontally_with_empty_spaces() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(2, 0), &[Right, Right, Right, Right, Right, Right, Right, Right]),
            vec!["Blanka", "Guile", "Ryu", "E.Honda", "Blanka", "Guile", "Ryu", "E.Honda"],
            "it should rotate horizontally with empty spaces",
        );        
    }
    
    #[test]
    fn rotate_on_all_rows() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_A[..], Position::new(2, 0), &[Right, Right, Right, Right, Right, Right, Down, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Down, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right]),
            vec!["Blanka", "Guile", "Ryu", "E.Honda", "Blanka", "Guile", "Dhalsim", "Zangief", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Cammy", "M.Bison", "Vega", "T.Hawk", "Fei Long", "Deejay", "Cammy", "M.Bison", "Vega", "T.Hawk", "Fei Long", "Deejay", "Cammy"],
            "it should rotate on all rows",
        );
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_B[..], Position::new(2, 0), &[Right, Right, Right, Right, Right, Right, Down, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Left, Down, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right, Right]),
            vec!["Cammy", "Blanka", "Guile", "Chun Li", "Ryu", "E.Honda", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "M.Bison", "Chun Li", "Ken", "Balrog", "Sagat", "Dhalsim", "Zangief", "Cammy", "T.Hawk", "Vega", "Fei Long", "Balrog", "Deejay", "Cammy", "T.Hawk", "Vega", "Fei Long", "Balrog", "Deejay", "Cammy"],
            "it should rotate on all rows",
        );        
    }
    
    #[test]
    fn should_rotate_on_all_rows() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_B[..], Position::new(3, 0), &[Down, Right, Right, Right, Down, Left, Left, Down, Right, Right, Right, Up]),
            vec!["Cammy", "Blanka", "Guile", "Chun Li", "Sagat", "Dhalsim", "Zangief", "Cammy", "T.Hawk", "Vega", "Fei Long", "Chun Li"],
            "it should rotate on all rows",
        );
    }
    
    #[test]
    fn should_work_with_longer_grid() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_C[..], Position::new(3, 0), &[Left, Left, Down, Right, Right, Right, Right, Down, Left, Left, Left, Left, Down, Right, Right, Down, Right, Right, Right, Down, Left, Left, Left, Down, Left, Left, Left]),
            vec!["E.Honda", "Ryu", "Ken", "Chun Li", "Balrog", "Ken", "Chun Li", "Fei Long", "Vega", "Balrog", "Fei Long", "Vega", "Blanka", "Guile", "Chun Li", "Sagat", "M.Bison", "Zangief", "Dhalsim", "Dhalsim", "Zangief", "M.Bison", "Sagat", "T.Hawk", "Cammy", "Deejay", "T.Hawk"],
            "it should work with longer grid",
        );        
    }
    
    #[test]
    fn should_work_with_odd_initial_position() {
        use Direction::*;
        
        assert_eq!(
            super_street_fighter_selection(&FIGHTERS_C[..], Position::new(3, 3), &[Left, Left, Down, Right, Right, Right, Right, Down, Left, Left, Left, Left, Up, Right, Right,  Up, Right, Right, Right]),
            vec!["Guile", "Blanka", "M.Bison", "Zangief", "Dhalsim", "Sagat", "M.Bison", "Deejay", "T.Hawk", "Cammy", "Deejay", "T.Hawk", "Sagat", "M.Bison", "Zangief", "Guile", "Chun Li", "Blanka", "Guile"],
            "it should work with odd initial position",
        );
}
