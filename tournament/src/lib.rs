use std::collections::HashMap;

#[derive(Debug)]
pub struct Table{
    matches_played: u32,
    matches_won: u32,
    matches_draw: u32,
    matches_lost: u32,
    points: u32    
}

pub fn tally(match_results: &str) -> String {
    let mut table_store: HashMap<&str, Table> = HashMap::new(); 
    for line in match_results.split('\n').into_iter() {
        tally_input_line(line, &mut table_store);
    }
    
    let mut table_store_vec: Vec<_> = table_store.into_iter().collect();
    table_store_vec.sort_by(|(_a, a_val), (_b, b_val)| b_val.points.cmp(&a_val.points));
    render_table_graph(&table_store_vec)
}

fn render_table_graph(table_store: &Vec<(&str, Table)>) -> String {
    let title = "Team                              | MP | W | D | L | P\n";
    let mut table_graph = format!("{}", title);
    for (team, data) in table_store{
        let needed_spaces = " ".repeat(34 - team.len());
      table_graph = format!("{}{}{}|  {} | {} | {} | {} | {}\n", table_graph, team, needed_spaces, data.matches_played,
        data.matches_won, data.matches_draw, data.matches_lost, data.points);
    }
    println!("{}", table_graph);
    table_graph
}


fn tally_input_line<'a>(input_line: &'a str, table_store: &mut HashMap<&'a str, Table>){
    let parsed_parts: Vec<&str> = input_line.trim_matches('\n').split(';').collect();
    if parsed_parts.len() != 3 {
        return;
    }
    for index in 0..=1 {
        if get_win_status(index, parsed_parts[2]) == "win" {
            table_store.entry(parsed_parts[index])
            .and_modify(|data| {
                data.matches_played = data.matches_played + 1;
                data.matches_won = data.matches_won + 1;
                data.points = data.points + 3;    
            })
            .or_insert(get_default_table("win"));
        } else if get_win_status(index, parsed_parts[2]) == "loss" {
            table_store.entry(parsed_parts[index])
            .and_modify(|data| {
                data.matches_played = data.matches_played + 1;
                data.matches_lost = data.matches_lost + 1;
            })
            .or_insert(get_default_table("loss"));
        } else {
            table_store.entry(parsed_parts[index])
            .and_modify(|data| {
                data.matches_played = data.matches_played + 1;
                data.matches_draw = data.matches_draw + 1;
                data.points = data.points + 1;
            })
            .or_insert(get_default_table("draw"));
        }
    }
}

fn get_win_status(index: usize, win_status: &str) -> String {
    if index == 0 && win_status == "win" {
        "win".to_owned()
    } else if index == 1 && win_status == "loss" {
        "win".to_owned()
    } else if win_status == "draw"{
        "draw".to_owned()
    } else {
        "loss".to_owned()
    }
}

fn get_default_table(status: &str) -> Table {
    match status {
        "win" => Table{
            matches_played: 1,
            matches_won: 1,
            matches_draw: 0,
            matches_lost: 0,
            points: 3
        },
        "loss" => Table{
            matches_played: 1,
            matches_won: 0,
            matches_draw: 0,
            matches_lost: 1,
            points: 0
        },
        _ => Table{
            matches_played: 1,
            matches_won: 0,
            matches_draw: 1,
            matches_lost: 0,
            points: 1
        }
    }
}
