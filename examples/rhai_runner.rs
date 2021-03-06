use rhai::{Engine, EvalAltResult, Position};

#[cfg(not(feature = "no_optimize"))]
use rhai::OptimizationLevel;

use std::{env, fs::File, io::Read, iter, process::exit};

fn padding(pad: &str, len: usize) -> String {
    iter::repeat(pad).take(len).collect::<String>()
}

fn eprint_error(input: &str, err: EvalAltResult) {
    fn eprint_line(lines: &[&str], line: usize, pos: usize, err: &str) {
        let line_no = format!("{}: ", line);
        let pos_text = format!(" (line {}, position {})", line, pos);

        eprintln!("{}{}", line_no, lines[line - 1]);
        eprintln!(
            "{}^ {}",
            padding(" ", line_no.len() + pos - 1),
            err.replace(&pos_text, "")
        );
        eprintln!("");
    }

    let lines: Vec<_> = input.split('\n').collect();

    // Print error
    let pos = if err.position().is_eof() {
        let last = lines[lines.len() - 1];
        Position::new(lines.len(), last.len() + 1)
    } else {
        err.position()
    };

    match pos {
        p if p.is_eof() => panic!("should not be EOF"),
        p if p.is_none() => {
            // No position
            eprintln!("{}", err);
        }
        p => {
            // Specific position
            let err_text = match err {
                EvalAltResult::ErrorRuntime(err, _) if !err.is_empty() => {
                    format!("Runtime error: {}", err)
                }
                err => err.to_string(),
            };

            eprint_line(&lines, p.line().unwrap(), p.position().unwrap(), &err_text)
        }
    }
}

fn main() {
    for filename in env::args().skip(1) {
        let mut engine = Engine::new();

        #[cfg(not(feature = "no_optimize"))]
        engine.set_optimization_level(OptimizationLevel::Full);

        let mut f = match File::open(&filename) {
            Err(err) => {
                eprintln!("Error reading script file: {}\n{}", filename, err);
                exit(1);
            }
            Ok(f) => f,
        };

        let mut contents = String::new();

        if let Err(err) = f.read_to_string(&mut contents) {
            eprintln!("Error reading script file: {}\n{}", filename, err);
            exit(1);
        }

        if let Err(err) = engine.consume(false, &contents) {
            eprintln!("{}", padding("=", filename.len()));
            eprintln!("{}", filename);
            eprintln!("{}", padding("=", filename.len()));
            eprintln!("");

            eprint_error(&contents, err);
        }
    }
}
