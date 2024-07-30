use crate::{Ins, Op};

pub fn lex(input: &str) -> Vec<Ins> {
    let filtered: Vec<char> = input.chars().filter(|c| "<>+-.,[]".contains(*c)).collect();

    let mut output: Vec<Ins> = Vec::new();
    let mut loop_stack: Vec<usize> = Vec::new();

    for c in filtered {
        match c {
            '<' | '>' => {
                let arg = if c == '<' { -1 } else { 1 };
                if let Some(last) = output.last_mut() {
                    if let Op::Mov = last.op {
                        last.arg += arg;
                        continue;
                    }
                }
                output.push(Ins { op: Op::Mov, arg });
            }
            '+' | '-' => {
                let arg = if c == '+' { 1 } else { -1 };
                if let Some(last) = output.last_mut() {
                    if let Op::Add = last.op {
                        last.arg += arg;
                        continue;
                    }
                }
                output.push(Ins { op: Op::Add, arg });
            }
            '.' => output.push(Ins { op: Op::Prt, arg: 0 }),
            ',' => output.push(Ins { op: Op::Inp, arg: 0 }),
            '[' => {
                output.push(Ins { op: Op::Lst, arg: 0 });
                loop_stack.push(output.len() - 1);
            }
            ']' => {
                let start = loop_stack.pop().expect("Unmatched ]");
                output.push(Ins { op: Op::Led, arg: start as i64 });
                output[start].arg = output.len() as i64 - 1;
            }
            _ => panic!("Invalid character {}", c),
        }
    }

    if !loop_stack.is_empty() {
        panic!("Unmatched [");
    }

    output
}
