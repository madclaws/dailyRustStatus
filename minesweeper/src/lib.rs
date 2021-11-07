pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut annotated_mines: Vec<String> = Vec::new();
    let row_count: usize = minefield.len();
    let mut col_count: usize = 0;

    if !minefield.is_empty() {
        col_count = minefield[0].len();
    }

    for i in 0..row_count as i32 {
        let mut annotated_row = String::from("");
        for j in 0..col_count as i32{
            annotated_row.push(annotate_cell(minefield, row_count, col_count, i, j));
        }
        annotated_mines.push(annotated_row);
    }
    annotated_mines
}

fn annotate_cell(minefield: &[&str], row_count: usize, col_count: usize, i: i32, j: i32) -> char {
    let mut surrounding_mine_count: i32 = 0;
    let row = minefield[i as usize];
    let element = row.chars().nth(j as usize).unwrap();
    if element == ' ' {
        // counting surrounding mines 
        // UP
        if (i - 1) >= 0 && minefield[(i - 1) as usize].chars().nth(j as usize).unwrap() == '*'{
            surrounding_mine_count += 1;
        }
        // DOWN
        if (i + 1) <= (row_count - 1) as i32 && minefield[(i + 1) as usize].chars().nth(j as usize).unwrap() == '*'{
            surrounding_mine_count += 1;
        }
        // LEFT
        if (j - 1) >= 0 && row.chars().nth((j-1) as usize).unwrap() == '*'{
            surrounding_mine_count += 1;
        }
        // RIGHT
        if (j + 1) <= (col_count - 1) as i32 && row.chars().nth((j + 1) as usize).unwrap() == '*'{
            surrounding_mine_count += 1;
        }
        // LEFT DIAGONAL UP
        if (i - 1) >= 0 && (j - 1) >= 0 && minefield[(i - 1) as usize].chars().nth((j - 1) as usize).unwrap() == '*'{
            surrounding_mine_count += 1;
        }
        // LEFT DIAGONAL DOWN
        if (i + 1) <= (row_count - 1) as i32 && (j + 1) <= (col_count - 1) as i32 {
            let cur_row = minefield[(i + 1) as usize];
            if cur_row.chars().nth((j + 1) as usize).unwrap() == '*' {
                surrounding_mine_count += 1;
            }
        }
        // RIGHT DIAGONAL UP
        if (i - 1) >= 0 && (j + 1) <= (col_count - 1) as i32 {
            let cur_row = minefield[(i - 1) as usize];
            if cur_row.chars().nth((j + 1) as usize).unwrap() == '*' {
                surrounding_mine_count += 1;
            }
        }
        // RIGHT DIAGONAL DOWN
        if (i + 1) <= (row_count - 1) as i32 && (j - 1) >= 0 {
            let cur_row = minefield[(i + 1) as usize];
            if cur_row.chars().nth((j - 1) as usize).unwrap() == '*' {
                surrounding_mine_count += 1;
            }
        }
    }
    if surrounding_mine_count > 0 {
        char::from_digit(surrounding_mine_count as u32, 10).unwrap()
    } else {
        element
    }
}