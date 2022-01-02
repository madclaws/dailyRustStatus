struct GridPtr {
    i: i32,
    j: i32,
    height: i32,
    width: i32,
}

impl GridPtr {
    fn new(height: i32, width: i32) -> Self {
        GridPtr {
            i: 0,
            j: 0,
            height,
            width,
        }
    }

    fn add_i(&mut self, incr: i32) {
        if self.i + incr < 0 {
            self.i = self.height - 1
        } else if self.i + incr == self.height {
            self.i = 0;
        } else {
            self.i += incr
        }
    }

    fn add_j(&mut self, incr: i32) {
        if self.j + incr < 0 {
            self.j = self.width - 1;
        } else if self.j + incr == self.width {
            self.j = 0;
        } else {
            self.j += incr
        }
    }
}
fn main() {
    println!("Paintfuck Interpreter");
    interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 100, 6, 9);
}

fn interpreter(code: &str, iterations: usize, width: usize, height: usize) -> String {
    let code_list: Vec<char> = code
        .chars()
        .filter(|c| matches!(c, 'n' | 's' | 'e' | 'w' | '*' | '[' | ']'))
        .collect();

    let mut grid: Vec<u32> = vec![0; width * height];
    let mut grid_ptr: GridPtr = GridPtr::new(height as i32, width as i32);
    let mut code_ptr: i32 = 0;

    if code.is_empty() || iterations == 0 {
        println!("Code or iterations is empty");
        return generate_output_from_grid(&grid, grid_ptr.height, grid_ptr.width);
    }

    let mut current_iter: i32 = 0;
    loop {
        if code_ptr < 0 || code_ptr >= code_list.len() as i32 {
            println!("Commands EOL reached!");
            break;
        }

        if current_iter == iterations as i32 {
            println!("Iterations finished!");
            break;
        }

        match code_list[code_ptr as usize] {
            'n' => {
                grid_ptr.add_i(-1);
                code_ptr += 1;
            }
            'e' => {
                grid_ptr.add_j(1);
                code_ptr += 1
            }
            's' => {
                grid_ptr.add_i(1);
                code_ptr += 1
            }
            'w' => {
                grid_ptr.add_j(-1);
                code_ptr += 1
            }
            '*' => {
                flip_bit(&mut grid, &grid_ptr);
                code_ptr += 1
            }
            '[' => {
                let index = (width as i32 * grid_ptr.i) + grid_ptr.j;
                if grid[index as usize] == 0 {
                    if let Some(index) =
                        get_matching_bracket_index_right(']', &code_list[code_ptr as usize..])
                    {
                        code_ptr += index as i32 + 1
                    } else {
                        panic!("No match for '[', code invalid, paint fuck you");
                    }
                } else {
                    code_ptr += 1
                }
            }
            ']' => {
                let index = (width as i32 * grid_ptr.i) + grid_ptr.j;
                if grid[index as usize] != 0 {
                    if let Some(index) =
                        get_matching_bracket_index_right('[', &code_list[0..code_ptr as usize])
                    {
                        code_ptr = index as i32 + 1
                    } else {
                        panic!("No match for ']', code invalid, paint fuck you");
                    }
                } else {
                    code_ptr += 1
                }
            }
            _ => panic!("Given code is fucked up!!"),
        }
        current_iter += 1;
    }

    generate_output_from_grid(&grid, grid_ptr.height, grid_ptr.width)
}

fn generate_output_from_grid(grid: &[u32], height: i32, width: i32) -> String {
    let mut output = String::from("");
    for i in 0..height {
        for j in 0..width {
            let index = (width * i) + j;
            if grid[index as usize] == 1 {
                output.push('1')
            } else {
                output.push('0')
            }
        }
        if i < height - 1 {
            output.push_str("\r\n")
        }
    }
    println!("OUTPUT GRID => \n{:?}", output);
    output
}

fn flip_bit(grid: &mut [u32], grid_ptr: &GridPtr) {
    let index = (grid_ptr.width * grid_ptr.i) + grid_ptr.j;
    grid[index as usize] = (grid[index as usize] + 1) % 2;
}

fn get_matching_bracket_index_right(code_char: char, code_slice: &[char]) -> Option<usize> {
    code_slice.iter().position(|code| *code == code_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_cases() {
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 0, 6, 9)), display_expected("000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should initialize all cells in the datagrid to 0");
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 7, 6, 9)), display_expected("111100\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should adhere to the number of iterations specified");
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 19, 6, 9)), display_expected("111100\r\n000010\r\n000001\r\n000010\r\n000100\r\n000000\r\n000000\r\n000000\r\n000000"), "Your interpreter should traverse the 2D datagrid correctly");
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 42, 6, 9)), display_expected("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should traverse the 2D datagrid correctly for all of the \"n\", \"e\", \"s\" and \"w\" commands");
        assert_eq!(display_actual(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 100, 6, 9)), display_expected("111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000"), "Your interpreter should terminate normally and return a representation of the final state of the 2D datagrid when all commands have been considered from left to right even if the number of iterations specified have not been fully performed");
    }

    /// Prints representation of datagrid - 0's are black and 1's are white.
    /// Note: it only works properly if your interpreter returns a representation
    /// of the datagrid in the correct format.
    fn pretty_print(datagrid: &str) -> &str {
        let rows = datagrid.split("\r\n");
        let mut output = String::new();
        output += "<pre>";
        for row in rows {
            for cell in row.chars() {
                output += "<span style=\"color:";
                output += if cell == '0' { "black" } else { "white" };
                output += ";background-color:";
                output += if cell == '0' { "black" } else { "white" };
                output += "\">xx</span>";
            }
            output += "<br />";
        }
        output += "</pre>";
        println!("{}", output);
        datagrid
    }

    /// Displays the grid the interpreter returns
    fn display_actual(actual: &str) -> &str {
        println!("You returned:");
        pretty_print(actual)
    }

    /// Displays the expected final state of datagrid
    fn display_expected(expected: &str) -> &str {
        println!("Expected final state of data grid:");
        pretty_print(expected)
    }
}
