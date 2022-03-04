pub fn encrypt(input: &str) -> String {
    let mut encrypted_input: String = String::from("");
    
    // Phase 1 - Normalize the input
    let normalized_input: String = input.to_lowercase()
    .chars()
    .filter(|x| !matches!(x, '.'|','|';'|':'|'?'|' '|'\n'|'-'))
    .collect();
   
    println!("{normalized_input}");

    // phase 2 - rectangle (r * c)
    // c >= r, c-r <= 1
   let encrypt_dimensions = find_rec_dimensions(normalized_input.len() as u32);
   println!("Encrypt dimension {:?}", encrypt_dimensions);

    // Phase 3 => making rectangle of words
   let mut encrypt_rect_input: Vec<String> = vec![String::from(""); encrypt_dimensions.1 as usize];
   let mut norm_chars_iter = normalized_input.chars();
   for i in 0..encrypt_dimensions.1 {
        for _j in 0..encrypt_dimensions.0 {
            if let Some(norm_char) = norm_chars_iter.next() {
                encrypt_rect_input[i as usize].push(norm_char);  
            } else {
                encrypt_rect_input[i as usize].push(' ');  
            }
        }
   }
   
   for i in 0..encrypt_dimensions.0 {
       for j in 0..encrypt_dimensions.1 {
        encrypted_input.push(encrypt_rect_input[j as usize].chars().nth(i as usize).unwrap());
       }
   }

    let mut output: String = String::from("");
    for (i, chr) in encrypted_input.chars().enumerate() {
        if i > 0 && i % encrypt_dimensions.1 as usize == 0 {
            output.push(' ');
        }
        output.push(chr);
    }
   
   println!("{output:?}");
   output
}

pub fn find_rec_dimensions(length: u32) -> (i32, i32) {
    for i in 1..length {
        for j in 1..length {
            if i >= j && (i - j) <= 1 {
                if (i * j) >= length {
                    return (i as i32, j as i32)
                }
            }
        }
    }
    return (0, 0)
}
