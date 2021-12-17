pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_pts: Vec<(usize, usize)> = Vec::new();
    let mut greater_row_indices: Vec<(usize, usize, usize)> = Vec::new();
    let mut smallest_col_indices: Vec<(usize, usize, usize)> = Vec::new();

    for i in 0..input.len() {
        let mut largest_row_index: Option<(usize, usize, usize)> = None;
        for j in 0..input[i].len() {
            if largest_row_index.is_none() {
                largest_row_index = Some((i, j, input[i][j] as usize));
            } else {
                if largest_row_index.unwrap().2 < input[i][j] as usize {
                    largest_row_index = Some((i, j, input[i][j] as usize));
                } 
            }
        }
        if let Some(index_info) = largest_row_index {

            for index in 0..input.len() {
                if input[index_info.0][index] == index_info.2 as u64 {
                    greater_row_indices.push((
                        index_info.0,
                        index,
                        input[index_info.0][index] as usize
                    ));
                }
            }
        }
    }

    // For every column
    for j in 0..input.len() {
        let mut smallest_col_index: Option<(usize, usize, usize)> = None;
        for i in 0..input[j].len() {
            if smallest_col_index.is_none() {
                smallest_col_index = Some((i, j, input[i][j] as usize));
            } else {
                if smallest_col_index.unwrap().2 > input[i][j] as usize {
                    smallest_col_index = Some((i, j, input[i][j] as usize));
                } 
            }
        }

        if smallest_col_index.is_some() {
            for index in 0..input[smallest_col_index.unwrap().0].len() {
                if input[index][smallest_col_index.unwrap().1] == smallest_col_index.unwrap().2 as u64{
                    if is_saddle(&greater_row_indices, (index, smallest_col_index.unwrap().1, input[index][smallest_col_index.unwrap().1] as usize)) {
                        smallest_col_indices.push(smallest_col_index.unwrap());
                        saddle_pts.push((index, smallest_col_index.unwrap().1))
                    }
                }
            }
        }
    }

    println!("{:?}", greater_row_indices);
    println!("{:?}", smallest_col_indices);
    println!("{:?}", saddle_pts);

    saddle_pts
}

fn is_saddle(greater_row_indices: &[(usize, usize, usize)], small_col_index: (usize, usize, usize)) -> bool {
    for index in 0..greater_row_indices.len() {
        if small_col_index.0 == greater_row_indices[index].0 && small_col_index.1 == greater_row_indices[index].1 {
            return true
        }
    }
    return false
}
