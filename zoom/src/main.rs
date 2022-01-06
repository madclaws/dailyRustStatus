fn main() {
    println!("Zoom it, Mr Data");
    // println!("{}", zoom(1));
    zoom(25);
}

fn zoom(n: i32) -> String {
    let mut patterns: Vec<String> = Vec::new();
    let mut current_char = "■ ";
    for val in 1..=n {
        if val % 2 == 0 {
            continue;
        }
        expand_horizontally(&mut patterns, current_char);
        create_new_line(&mut patterns, current_char, val as usize);
        if current_char == "■ " {
            current_char = "□ "
        } else {
            current_char = "■ "
        }
    }
    render_zoom(&patterns)
}

fn expand_horizontally(patterns: &mut Vec<String>, current_char: &str) {
    if !patterns.is_empty() {
        // Go through all keys, prepend and append the current char
        for index in 0..patterns.len() {
            patterns[index] = format!("{}{}{}", current_char, patterns[index], current_char);
        }
    }
}

fn create_new_line(patterns: &mut Vec<String>, char_pattern: &str, len: usize) {
    let new_pattern = char_pattern.repeat(len);
    patterns.push(new_pattern);
}

fn render_zoom(patterns: &Vec<String>) -> String{
    // Expect 0th key, rest should be prepended and appended to the zoom string
  let mut zoom_buffer = String::from("");
  for index in 0..patterns.len() {
    if index == 0 {
      zoom_buffer = format!("{}\n", patterns[index]);
    } else {
      zoom_buffer = format!("{}\n{}{}\n", patterns[index], zoom_buffer, patterns[index]);
    }
  }
  zoom_buffer.pop();
  println!("{}", zoom_buffer);
  zoom_buffer
}

#[test]
fn basic_test_1() {
  assert_eq!(zoom(1), "■");
}

#[test]
fn basic_test_2() {
  assert_eq!(zoom(3), "\
□□□
□■□
□□□"
  );
}

#[test]
fn basic_test_3() {
  assert_eq!(zoom(5), "\
■■■■■
■□□□■
■□■□■
■□□□■
■■■■■"
  );
}

#[test]
fn basic_test_4() {
  assert_eq!(zoom(7), "\
□□□□□□□
□■■■■■□
□■□□□■□
□■□■□■□
□■□□□■□
□■■■■■□
□□□□□□□"
  );
}

#[test]
fn basic_test_5() {
  assert_eq!(zoom(9), "\
■■■■■■■■■
■□□□□□□□■
■□■■■■■□■
■□■□□□■□■
■□■□■□■□■
■□■□□□■□■
■□■■■■■□■
■□□□□□□□■
■■■■■■■■■"
  );
}
