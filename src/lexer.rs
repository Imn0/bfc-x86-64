use crate::{Ins, Op};

pub fn lex(input: &str) -> Vec<Ins> {
    let filtered: Vec<char> = input.chars().filter(|c| "<>+-.,[]".contains(*c)).collect();

    let mut output: Vec<Ins> = Vec::new();

    let mut loop_stack: Vec<i64> = Vec::new();

    let mut ip: i64 = 0;
    for c in filtered {
        match c {
            '<' => {
                if !output.last().is_none() && output.last().unwrap().op == Op::Mov {
                    output[(ip - 1) as usize].arg -= 1;
                } else {
                    output.push(Ins {
                        op: Op::Mov,
                        arg: -1,
                    });
                    ip += 1;
                }
            }
            '>' => {
                if !output.last().is_none() && output.last().unwrap().op == Op::Mov {
                    output[(ip - 1) as usize].arg += 1;
                } else {
                    output.push(Ins {
                        op: Op::Mov,
                        arg: 1,
                    });
                    ip += 1;
                }
            }
            '+' => {
                if !output.last().is_none() && output.last().unwrap().op == Op::Add {
                    output[(ip - 1) as usize].arg += 1;
                } else {
                    output.push(Ins {
                        op: Op::Add,
                        arg: 1,
                    });
                    ip += 1;
                }
            }
            '-' => {
                if !output.last().is_none() && output.last().unwrap().op == Op::Add {
                    output[(ip - 1) as usize].arg -= 1;
                } else {
                    output.push(Ins {
                        op: Op::Add,
                        arg: -1,
                    });
                    ip += 1;
                }
            }
            '.' => {
                output.push(Ins {
                    op: Op::Prt,
                    arg: 0,
                });
                ip += 1;
            }
            ',' => {
                output.push(Ins {
                    op: Op::Inp,
                    arg: 0,
                });
                ip += 1;
            }
            '[' => {
                output.push(Ins {
                    op: Op::Lst,
                    arg: 0,
                });
                loop_stack.push(ip);
                ip += 1;
            }
            ']' => {
                let start = loop_stack.pop().expect("Unmatched ]");
                output.push(Ins {
                    op: Op::Led,
                    arg: start,
                });
                output[start as usize].arg = ip;
                ip += 1;
            }
            _ => {
                panic!("invalid character {}", c)
            }
        }
    }

    if loop_stack.len() > 0 {
        panic!("Unmatched [");
    }

    output
}
