use std::collections::HashMap;
use std::fs;

pub fn run() {
    println!("\nDay 4 => The Giant Squid");
    let raw_input = fs::read_to_string("/home/madclaws/labs/aoc_2021/inputs/day_4.txt")
        .expect("Unable to load file");
    let raw_input_by_line: Vec<&str> = raw_input.split('\n').collect();
    let mut boards: HashMap<i32, Vec<(i32, bool)>> = HashMap::new();

    let draw_list: Vec<i32> = raw_input_by_line[0]
        .split(",")
        .map(|str_num| str_num.parse::<i32>().unwrap())
        .collect();

    println!("DRAW LIST => {:?}\n", draw_list);
    create_bingo_boards(&mut boards, &raw_input_by_line);
    let mut winners: Vec<i32> =  Vec::new();
    let mut win_list = Vec::new();
    let mut last_ball = 0;
    for ball in draw_list {
        println!("Ball drewed {}", ball);
        mark_on_board(ball, &mut boards);
        win_list  = check_board_win(&mut boards, &winners);
        if win_list.len() > 0 {
            println!("WIN LIST {:?}", win_list);
            for winny in win_list {
                winners.push(winny);
            }
            // println!("Winners {:?}" , winners);
            last_ball = ball;
            if winners.len() == boards.len() {
                break;
            }
        }
    }
    println!("Winner board =>{:?}, last ball {}" , winners.last(), last_ball);

    if let Some(board) = boards.get(&winners.last().unwrap()) {
        let mut umarked_sum = 0;
        for (value, is_marked) in board {
            println!("{} {}", value, is_marked);
            if !is_marked {
                umarked_sum += value;
            }
        }
        println!("Final score => {}", umarked_sum * last_ball);
    }
}

fn create_bingo_boards(boards: &mut HashMap<i32, Vec<(i32, bool)>>, raw_input_by_line: &[&str]) {
    let mut board: Vec<(i32, bool)> = Vec::new();
    let mut board_count: i32 = 0;
    for val in 2..raw_input_by_line.len() {
        let row_line = raw_input_by_line[val];
        // println!("ROW LINE {}", row_line);
        if row_line == "" {
            boards.insert(board_count, board);
            board = Vec::new();
            board_count += 1;
        } else {
            let row_vec: Vec<i32> = row_line
                .split_ascii_whitespace()
                .map(|elem| elem.parse::<i32>().unwrap())
                .collect();
            for element in row_vec {
                board.push((element, false));
            }
        }
    }
    boards.insert(board_count, board);
}

fn mark_on_board(number: i32, boards: &mut HashMap<i32, Vec<(i32, bool)>>) {
    for (_board_no, board) in boards {
        for index in 0..25 {
            if board[index].0 == number {
                board[index] = (number, true);
            }
        }
    }
}

fn check_board_win(boards: &mut HashMap<i32, Vec<(i32, bool)>>, winners: &[i32]) -> Vec<i32> {
    let mut win_boards: Vec<i32> = Vec::new();
    let mut to_continue: bool = false;
    for (board_no, board) in boards {
        to_continue = false;
        for win_index in winners {
            if win_index == board_no {
                to_continue = true;
                break;
            }
        } 
        if to_continue {
            continue;
        }
        let mut marked_count: i32 = 0;

        // Row check
        for i in 0..5 {
            for j in 0..5 {
                let index = (i * 5) + j;
                if board[index].1 {
                    marked_count += 1;
                } else {
                    marked_count = 0;
                }
            }
            if marked_count >= 5 {
                win_boards.push(*board_no);
            }
        }
      
        // column check
        for j in 0..5 {
            for i in 0..5 {
                let index = (i * 5) + j;
                if board[index].1 {
                    marked_count += 1;
                } else {
                    marked_count = 0;
                }
            }
            if marked_count >= 5 {
                win_boards.push(*board_no);
            }
        }
    }
    win_boards
}
