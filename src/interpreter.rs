use std::io::Write;

use crate::{Ins, Op};

pub fn interpret(instructions: Vec<Ins>) {
    let mut memory: Vec<i8> = vec![0; 30000];
    let mut ip: usize = 0;
    let mut mp: usize = 0;

    loop {
        match instructions[ip].op {
            Op::Lft => {
                let _mp: i64 = mp as i64 - 1;
                if _mp < 0 {
                    mp = memory.len() - 1;
                } else {
                    mp = _mp as usize;
                }
            },
            Op::Rit => {
                let _mp: i64 = mp as i64 + 1;
                if _mp >= memory.len() as i64 {
                    mp = 0;
                } else {
                    mp = _mp as usize;
                }

            },
            Op::Inc => {
                memory[mp] += 1;
            },
            Op::Dec => {
                memory[mp] -= 1;
            },
            Op::Prt => {
                println!("{}", memory[mp] as u8 as char);
                std::io::stdout().flush().unwrap();

            },
            Op::Inp => {
                let mut input = String::new();
                print!(">");
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut input).expect("Error reading input");
                if input.as_bytes().is_ascii() {
                    memory[mp] = input.as_bytes()[0] as i8;
                }
            },
            Op::Lst => {
                if memory[mp] == 0 {
                    ip = instructions[ip].arg;
                }
            },
            Op::Led => {
                if memory[mp] != 0 {
                    ip = instructions[ip].arg;
                }
            }
        }
        ip += 1;
        if ip >= instructions.len() {
            break;
        }
    }
}
