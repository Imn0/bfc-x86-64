extern crate getopts;
use std::os::unix::fs::PermissionsExt;
mod compiler;
mod interpreter;
mod lexer;
use std::{env::args, fs};

macro_rules! either {
    ($test:expr => $true_expr:expr; $false_expr:expr) => {
        if $test {
            $true_expr
        }
        else {
            $false_expr
        }
    }
}


#[derive(Debug)]
pub enum Op {
    Lft,
    Rit,
    Inc,
    Dec,
    Prt,
    Inp,
    Lst,
    Led,
}

pub struct Ins {
    op: Op,
    arg: usize,
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optflag("i", "interpret", "interpret the program");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("D", "debug", "print debug information");
    opts.optopt("o", "", "set output file name", "NAME");


    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string()),
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: {} [options] FILE", program);
        print!("{}", opts.usage(&brief));
        return;
    }

    let mut output: String = "a.out".to_owned();
    if matches.opt_present("o") {
        output = matches.opt_str("o").unwrap();
    }

    let input_path = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        let brief = format!("Usage: {} [options] FILE", program);
        print!("{}", opts.usage(&brief));
        return;
    };

    let input: String;
    match fs::read_to_string(&input_path) {
        Ok(s) => input = s,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    }

    let ins = lexer::lex(&input);
    if matches.opt_present("D") {
        print_deug(&ins);
    }

    if matches.opt_present("i") {
        interpreter::interpret(ins);
    } else {
        compiler::compile(ins, &output);

        let metadata = fs::metadata(&output).expect("Error setting permissions");
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&output, permissions).expect("Error setting permissions");
    }
}


fn print_deug(instructions: &Vec<Ins>) {
    for (i, ins) in instructions.iter().enumerate() {
        println!("{}: {:?} {}", format!("{:0>width$}", i, width = 8), ins.op, either!(ins.arg  != 0 => format!("{}", ins.arg); "".to_owned()));
    }

}