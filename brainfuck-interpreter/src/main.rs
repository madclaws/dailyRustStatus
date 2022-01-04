fn main() {
    println!("BrainFuck Interpreter!");
    
    brain_fuck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]); // Multiply 2 numbers - returns 72
    // brain_fuck(",[.[-],]", ez_vec("Codewars", 0));
}

fn ez_vec(s: &str, i: u8) -> Vec<u8> {
    let mut v = s.to_string().into_bytes();
    v.push(i);
    v
}   

fn brain_fuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut input_ptr: i32 = 0; 
    let mut tape: Vec<u8> = vec![0;50];
    let mut tape_ptr: i32 = 0;
    let mut code_ptr: i32 = 0;
    let mut output: Vec<u8> = vec![0;0];
    let code_vec: Vec<char> = code.chars()
                              .filter(|c| matches!(c, '>'|'<'|'+'|'-'|'.'|','|'['|']'))
                              .collect();

    println!("CODE {:?}", code_vec);
    if code.is_empty() {
        println!("Code is empty, fuck you");
        return output
    }

    loop {
        if !is_pointers_in_bound(code_ptr, code_vec.len() as u32, tape_ptr, tape.len() as u32) {
            break;
        }
        println!("---------------------------------");
        println!("COMMAND SET => {}", code);

        match code_vec[code_ptr as usize] {
            '>' => {
                println!("COMMAND => >");
                tape_ptr += 1;
                code_ptr += 1;
            },
            '<' => {
                println!("COMMAND => <");

                tape_ptr -= 1;
                code_ptr += 1;
            },
            '+' => {
                println!("COMMAND => +");

                tape[tape_ptr as usize] = tape[tape_ptr as usize].checked_add(1).unwrap_or(0);
                // println!("Added")
                code_ptr += 1;
            },
            '-' => {
                println!("COMMAND => -");

                tape[tape_ptr as usize] = tape[tape_ptr as usize].checked_sub(1).unwrap_or(255);
                code_ptr += 1;
            },
            '.' => {
                println!("COMMAND => .");

                output.push(tape[tape_ptr as usize]);
                // println!("OUTPUT => {:?}", output);
                code_ptr += 1;
            },
            ',' => {
                println!("COMMAND => ,");

                println!("TAKING INPUT => {}", input[input_ptr as usize]);
                tape[tape_ptr as usize] = input[input_ptr as usize];
                input_ptr += 1;
                code_ptr += 1;
            },
            '[' => {
                println!("COMMAND => [");

                if tape[tape_ptr as usize] == 0 {
                    if let Some(index) =
                        get_matching_bracket_index_right(&code_vec[code_ptr as usize..])
                    {
                        code_ptr += index as i32 + 1;
                    } else {
                        panic!("No match for '[', code invalid, brain fuck you");
                    }
                } else {
                    code_ptr += 1
                }
            },
            ']' => {
                println!("COMMAND => ]");

                if tape[tape_ptr as usize] != 0 {
                    if let Some(index) =
                    get_matching_bracket_index_left(&code_vec, code_ptr as usize)
                    {
                        code_ptr = index as i32 + 1;
                    } else {
                        panic!("No match for ']', code invalid, brain fuck you");
                    }
                } else {
                    code_ptr += 1
                }
            },
            _ => {
                panic!("Given code is brain fucked up!")
            }
        }
        println!("TAPE => {:?}", tape);
        println!("TAPE PTR => {:?}", tape_ptr);
        println!("---------------------------------")
    }

    // println!("{:?}", String::from_utf8(output.clone()));
    println!("{:?}", output);
    output
}

fn is_pointers_in_bound(code_ptr: i32, code_len: u32, tape_ptr: i32, tape_len: u32) -> bool {
    let is_code_ptr_bound = code_ptr >= 0 && code_ptr < code_len as i32;
    let is_tape_ptr_bound = tape_ptr >= 0 && tape_ptr < tape_len as i32;

    is_code_ptr_bound && is_tape_ptr_bound
}

fn get_matching_bracket_index_right(code_slice: &[char]) -> Option<usize> {
    let mut stack: Vec<char> = Vec::new();
    
    for index in 0..code_slice.len() {
        if index == 0 {
            stack.push(code_slice[index]);    
            continue;    
        }
        if code_slice[index] != ']' && code_slice[index] != '[' {
            continue;
        }
        if code_slice[index] == ']' {
            stack.pop();
        } else {
            stack.push(code_slice[index]);
        }
        if stack.is_empty() {
            return Some(index);
        }
    }
    
    None
}

fn get_matching_bracket_index_left(code_slice: &[char], bracket_index: usize) -> Option<usize> {
    let mut ptr: i32 = (bracket_index - 1) as i32;
    let mut stack: Vec<char> = Vec::new();
    stack.push(']'); 
    loop {
        if ptr < 0 {
            break;
        }
        if code_slice[ptr as usize] != ']' && code_slice[ptr as usize] != '[' {
            ptr -= 1;
            continue;
        } 

        if code_slice[ptr as usize] == '[' {
            stack.pop();
        } else {
            stack.push(code_slice[ptr as usize]);
        }
        if stack.is_empty() {
            return Some(ptr as usize);
        }
        ptr -= 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_tests() {
        // Echo until byte 255 encountered
        assert_eq!(String::from_utf8(brain_fuck(",+[-.,+]", ez_vec("Codewars", 255))).unwrap(), "Codewars");
        // Echo until byte 0 encountered
        assert_eq!(String::from_utf8(brain_fuck(",[.[-],]", ez_vec("Codewars", 0))).unwrap(), "Codewars");
        // Multiply two numbers
        assert_eq!(brain_fuck(",>,<[>[->+>+<<]>>[-<<+>>]<<<-]>>.", vec![8, 9]), vec![72]);
    }
    
    // Takes a static string and a terminating byte and returns an owned Vec<u8> for convenience
    // Without it, character-based tests are a pain   
    fn ez_vec(s: &str, i: u8) -> Vec<u8> {
      let mut v = s.to_string().into_bytes();
      v.push(i);
      v
    }   
}
