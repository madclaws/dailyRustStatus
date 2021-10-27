#[derive(Debug)]
#[derive(Clone)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let input_vector: Vec<CalculatorInput> = inputs.to_owned().to_vec();
    let mut output_stack: Vec<i32> =Vec::new();
    for (_i, val) in input_vector.iter().enumerate() {
        match val {
            CalculatorInput::Add => {
                if let Some((operand_a, operand_b)) = process_calculation(&mut output_stack) {
                    output_stack.push(operand_a + operand_b); 
                } else {
                    return None
                }
            }
            CalculatorInput::Subtract => {   
                if let Some((operand_a, operand_b)) = process_calculation(&mut output_stack) {
                    output_stack.push(operand_b - operand_a); 
                } else {
                    return None
                }
            }
            CalculatorInput::Multiply => {
                if let Some((operand_a, operand_b)) = process_calculation(&mut output_stack) {
                    output_stack.push(operand_a * operand_b); 
                } else {
                    return None
                }
            }
            CalculatorInput::Divide => {
                if let Some((operand_a, operand_b)) = process_calculation(&mut output_stack) {
                    output_stack.push(operand_b / operand_a); 
                } else {
                    return None
                }
            }
            CalculatorInput::Value(val) =>{
                output_stack.push(*val)  
            } 
        }
    }
    if output_stack.len() == 1 {
        Some(output_stack[0])
    } else {
        None
    }
}

fn process_calculation(output_stack: &mut Vec<i32>) -> Option<(i32, i32)>{
    if output_stack.len() < 2 {
        return None
    } 
    let operand_a = output_stack.pop().unwrap();
    let operand_b = output_stack.pop().unwrap();
    Some((operand_a, operand_b))
}
