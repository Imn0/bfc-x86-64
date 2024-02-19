use std::io::Write;

use crate::{Ins, Op};

pub fn interpret(instructions: Vec<Ins>) {
    let mut memory: Vec<i8> = vec![0; 30000];
    let mut ip: i64 = 0; // instruction pointer
    let mut mp: usize = 0; // memory pointer

    loop {
        match instructions[ip as usize].op {
            Op::Mov => {
                let _mp: i64 = mp as i64 + instructions[ip as usize].arg;
                if _mp < 0 {
                    mp = memory.len() - 1;
                } else if _mp >= memory.len() as i64 {
                    mp = 0;
                } else {
                    mp = _mp as usize;
                }
            }
            Op::Add => {
                memory[mp] += instructions[ip as usize].arg as i8;
            }

            Op::Prt => {
                print!("{}", memory[mp] as u8 as char);
                std::io::stdout().flush().unwrap();
            }
            Op::Inp => {
                let mut input = String::new();
                print!("\n>");
                std::io::stdout().flush().unwrap();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Error reading input");
                if input.as_bytes().is_ascii() {
                    memory[mp] = input.as_bytes()[0] as i8;
                }
            }
            Op::Lst => {
                if memory[mp] == 0 {
                    ip = instructions[ip as usize].arg;
                }
            }
            Op::Led => {
                if memory[mp] != 0 {
                    ip = instructions[ip as usize].arg;
                }
            }
        }
        ip += 1;
        if ip >= instructions.len() as i64 {
            break;
        }
    }
}
