pub fn encrypt(input: &str) -> String {
    let mut encrypted_input: String = String::from("");
    
    // Phase 1 - Normalize the input
    let normalized_input: String = input.to_lowercase()
    .chars()
    .filter(|x| !matches!(x, '.'|','|';'|':'|'?'|' '))
    .collect();
   
    // phase 2 - rectangle (r * c)
    // c >= r, c-r <= 1
   println!("{normalized_input}");
    
   let encrypt_dimensions = find_rec_dimensions(normalized_input.len() as u32);
   println!("Encrypt dimension {:?}", encrypt_dimensions);

   let mut encrypt_counter: usize = 0;
   let mut encrypt_rect_input: Vec<String> = Vec::new();
   let mut norm_chars_iter = normalized_input.chars();
   for _i in 0..encrypt_dimensions.1 {
    for _j in 0..encrypt_dimensions.0 {
        encrypt_rect_input[encrypt_counter as usize].push(norm_chars_iter.next().unwrap());  
       }
   } 
   encrypted_input
    
    
}

pub fn find_rec_dimensions(length: u32) -> (i32, i32) {
    for i in 1..length {
        for j in 1..length {
            if i >= j && (i - j) <= 1 {
                // println!("{}:{}", i, j);
                if (i * j) >= length {
                    return (i as i32, j as i32)
                }
            }
        }
    }
    return (0, 0)
}
