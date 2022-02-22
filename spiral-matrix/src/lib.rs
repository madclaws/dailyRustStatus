pub enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP
}
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    // creating a 2d matrix
    let mut s_matrix: Vec<Vec<u32>> = Vec::new();
    let mut value: u32 = 1;
    let mut can_spiral = true;
    let mut current_direciton = Direction::RIGHT;
    let mut current_row: usize = 0;
    let mut current_col: usize = 0;
    for row in 0..size {
        s_matrix[row as usize] = Vec::new();
        for col in 0..size {
            s_matrix[row as usize][col as usize] = 0;
        }
    }
    
    while can_spiral {
        match current_direciton {
            Direction::RIGHT => {
                s_matrix[current_row][current_col] = value;
                if current_col == (size - 1) as usize {
                    current_direciton = get_next_direction(current_direciton);
                } else {
                    current_col += 1;
                }
            }
            Direction::DOWN => {
                s_matrix[current_row][current_col] = value;
                if current_row == (size - 1) as usize {
                    current_direciton = get_next_direction(current_direciton);
                } else {
                    current_row += 1;
                }
            },
            Direction::LEFT => {
                s_matrix[current_row][current_col] = value;
                if current_col == 0 as usize {
                    current_direciton = get_next_direction(current_direciton);
                } else {
                    current_col -= 1;
                }
            },
            Direction::UP => {
                s_matrix[current_row][current_col] = value;
                if current_row == 0 as usize {
                    current_direciton = get_next_direction(current_direciton);
                } else {
                    current_row -= 1;
                }
            }
        }
        value += 1;
    }

    s_matrix
}

fn get_next_direction(current_direction: Direction) -> Direction{
    match current_direction {
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
        Direction::UP => Direction::RIGHT,
    }
}
