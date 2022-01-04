pub struct WordProblem;


pub fn answer(command: &str) -> Option<i32> {
    let mut operand_stack: Vec<i32> = Vec::new();
    let mut operator_stack: Vec<&str> = Vec::new();
    let mut last_value: &str = "";
    let question: Vec<&str> = command.trim_end_matches('?')
                                .split(' ')
                                .filter(|word| !matches!(*word, "by" | "to" | "the" | "power"))
                                .collect::<Vec<&str>>();
    println!("{:?}", question);

    if question.len() < 3 {
        println!("Not long enough for a valid question");
        return None
    }
    if question[0] != "What" || question[1] != "is" {
        println!("The starting of question is wrong");
        return None
    }
    for value in question.iter().skip(2) {
        if let Ok(operand) = value.parse::<i32>() {
            if operand_stack.is_empty() {
                operand_stack.push(operand);
                last_value = "operand";
            } else if last_value != "operand"{
                let current_operator = operator_stack.pop().unwrap();
                let prev_operand = operand_stack.pop().unwrap();
                let result = evaluate(current_operator, prev_operand, operand);
                operand_stack.push(result);
                last_value = "operand";
            } else {
                return None
            }
        } else {
            if !operator_stack.is_empty() && operator_stack.last().unwrap() == &"raised" {
                // A case we have to handle for the 5th, 2end etc..
                let num = value.chars().next().unwrap();
                let current_operator = operator_stack.pop().unwrap();
                let prev_operand = operand_stack.pop().unwrap();
                let result = evaluate(current_operator, prev_operand, num.to_digit(10).unwrap() as i32);
                operand_stack.push(result);
                last_value = "operand";
                continue;
            }
            if !operator_stack.is_empty() || !is_valid_operator(value) {
                return None
            }
            operator_stack.push(value);
            last_value = "operator";
        }
    }

    if !operator_stack.is_empty() {
        println!("Missing operator");
        return None
    }
    println!("ANSWER => {:?}", operand_stack);
    operand_stack.pop()
}

fn is_valid_operator(operator: &str) -> bool {
    matches!(operator, "plus" | "minus" | "multiplied" | "divided" | "raised")
}

fn evaluate(operator: &str, operand1: i32, operand2: i32) -> i32 {
    match operator {
        "plus" => operand1 + operand2,
        "minus" => operand1 - operand2,
        "multiplied" => operand1 * operand2,
        "divided" => operand1 / operand2,
        "raised" => operand1.pow(operand2 as u32),
        _ => panic!("Evaluator fucked up")
    }
}