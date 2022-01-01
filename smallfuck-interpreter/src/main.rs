fn main() {
    println!("Smallfuck interpreter!");
    interpreter(">*[>*]", "01101100");
}

fn interpreter(code: &str, tape: &str) -> String {
    let code_vec: Vec<char> = code
        .chars()
        .filter(|c| matches!(c, '<' | '>' | '*' | '[' | ']'))
        .collect();
    let mut tape_vec: Vec<char> = tape.chars().collect();
    let mut code_ptr: i32 = 0;
    let mut tape_ptr: i32 = 0;

    println!("CODE TO EXECUTE => {:?}", code_vec);
    println!("MEMORY TAPE => {:?}", tape_vec);

    // interpreter loop
    loop {
        // program exit conditions
        if is_ptrs_inbounds(&tape_ptr, tape_vec.len(), &code_ptr, code_vec.len()) {
            match code_vec[code_ptr as usize] {
                '<' => {
                    tape_ptr -= 1;
                    code_ptr += 1
                }
                '>' => {
                    tape_ptr += 1;
                    code_ptr += 1
                }
                '*' => {
                    if tape_vec[tape_ptr as usize] == '1' {
                        tape_vec[tape_ptr as usize] = '0'
                    } else {
                        tape_vec[tape_ptr as usize] = '1'
                    }
                    code_ptr += 1;
                }
                '[' => {
                    if tape_vec[tape_ptr as usize] == '0' {
                        if let Some(index) =
                            get_matching_bracket_index_right(']', &code_vec[code_ptr as usize..])
                        {
                            code_ptr += index as i32 + 1;
                        } else {
                            panic!("No match for '[', code invalid, fuck you");
                        }
                    } else {
                        code_ptr += 1;
                    }
                }
                ']' => {
                    if tape_vec[tape_ptr as usize] == '1' {
                        if let Some(index) =
                            get_matching_bracket_index_right('[', &code_vec[code_ptr as usize..])
                        {
                            code_ptr += index as i32;
                        } else {
                            panic!("No match for ']', code invalid, fuck you");
                        }
                    } else {
                        code_ptr += 1;
                    }
                }
                _ => panic!("Given code is fucked up!!"),
            }
        } else {
            break;
        }
    }
    let output = tape_vec.into_iter().collect::<String>();
    println!("SMALL FUCKING OUTPUT => {}", output);
    output
}

fn is_ptrs_inbounds(
    tape_ptr: &i32,
    tape_length: usize,
    code_ptr: &i32,
    code_length: usize,
) -> bool {
    let is_tape_bound = *tape_ptr >= 0 && *tape_ptr < tape_length as i32;

    let is_code_bound = *code_ptr >= 0 && *code_ptr < code_length as i32;

    is_tape_bound && is_code_bound
}

fn get_matching_bracket_index_right(code_char: char, code_slice: &[char]) -> Option<usize> {
    code_slice.iter().position(|code| *code == code_char)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_smallfuck() {
        // Flips the leftmost cell of the tape
        assert_eq!(interpreter("*", "00101100"), "10101100");

        // Flips the second and third cell of the tape
        assert_eq!(interpreter(">*>*", "00101100"), "01001100");

        // Flips all the bits in the tape
        assert_eq!(interpreter("*>*>*>*>*>*>*>*", "00101100"), "11010011");

        // Flips all the bits that are initialized to 0
        assert_eq!(interpreter("*>*>>*>>>*>*", "00101100"), "11111111");

        // Goes somewhere to the right of the tape and then flips all bits that are initialized to 1, progressing leftwards through the tape
        assert_eq!(interpreter(">>>>>*<*<<*", "00101100"), "00000000");

        // Try to jump rightwards, but the bit is 1, so no jump
        // The output will be same as second test case
        assert_eq!(interpreter(">*[>*]", "00101100"), "01001100");

        // Jump rightwards and reach out of bound, thus exiting the program
        assert_eq!(interpreter(">**[>*]", "00101100"), "00101100");
    }
}
