use crate::{Ins, Op};

pub fn lex(input: &str) -> Vec<Ins> {
    let filtered: Vec<char> = input.chars().filter(|c| "<>+-.,[]".contains(*c)).collect();

    let mut output = Vec::new();

    let mut loop_stack: Vec<usize> = Vec::new();

    let mut ip = 0;
    for c in filtered {
        match c {
            '<' => output.push(Ins {
                op: Op::Lft,
                arg: 0,
            }),
            '>' => output.push(Ins {
                op: Op::Rit,
                arg: 0,
            }),
            '+' => output.push(Ins {
                op: Op::Inc,
                arg: 0,
            }),
            '-' => output.push(Ins {
                op: Op::Dec,
                arg: 0,
            }),
            '.' => output.push(Ins {
                op: Op::Prt,
                arg: 0,
            }),
            ',' => output.push(Ins {
                op: Op::Inp,
                arg: 0,
            }),
            '[' => {
                output.push(Ins {
                    op: Op::Lst,
                    arg: 0,
                });
                loop_stack.push(ip);
            }
            ']' => {
                let start = loop_stack.pop().expect("Unmatched ]");
                output.push(Ins {
                    op: Op::Led,
                    arg: start,
                });
                output[start].arg = ip;
            }
            _ => {
                panic!("invalid character {}", c)
            }
        }
        ip += 1;
    }

    if loop_stack.len() > 0 {
        panic!("Unmatched [");
    }

    output
}
