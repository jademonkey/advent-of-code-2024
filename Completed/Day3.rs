use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Turn debugging information on
    #[arg(short, long, default_value_t = false)]
    debug: bool,

    /// Run with full output
    #[arg(short, long, default_value_t = false)]
    full: bool,
}

// let mut debug: bool = false;
fn main() {
    let cli = Args::parse();
    match cli.debug {
        false => println!("Debug mode is off"),
        true => println!("Debug mode is on")
    }
    match cli.full {
        false => println!("full mode is off"),
        true => println!("full mode is on")
    }


    let mut v_lines = Vec::new();
    let filename;
    if cli.full {
        filename = "./Inputs/Day3";
    } else {
        filename = "./Inputs/Example3";
    }

    /* Read input */
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        // read the lines and the numbers into their lists
        for line in lines.flatten() {
            if cli.debug {
              println!("Line in: {}", line);
            }
            v_lines.push(line);
        }
        if cli.debug {
            println!("Lines: {:?}\n", v_lines);
        }
    }

    /* Action time! */
    let mut p1_answer = 0;
    let mut p2_answer = 0;
    let mut include_mul = true;
    let re = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\)|don't\(\)|do\(\))").unwrap();
    for line in v_lines {
        if cli.debug {
            println!("line examined = {0}", line);
        }
        let figures: Vec<&str> = re.captures_iter(&line).map(|caps| {
            let (_, [result]) = caps.extract();
            result
        }).collect();
        if cli.debug {
            println!("Mathes: {:?}\n", figures);
        }
        for match_value in figures {
            if match_value == "don't()" {
                if cli.debug {
                    println!("Deactivaing");
                }
                include_mul = false;
                continue;
            } 
            if match_value == "do()" {
                if cli.debug {
                    println!("Activating");
                }
                include_mul = true;
                continue;
            } 
            // otherwise we do the sum
            let re2 = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
            let nums_to_mul: Vec<(&str, &str)> = re2.captures_iter(&match_value).map(|caps| {
                let (_, [first, second]) = caps.extract();
                (first, second)
            }).collect();
            if cli.debug {
            println!("{0} * {1}", nums_to_mul[0].0, nums_to_mul[0].1);
            }
            p1_answer += nums_to_mul[0].0.parse::<i32>().unwrap() * nums_to_mul[0].1.parse::<i32>().unwrap();
            if include_mul {
                p2_answer+= nums_to_mul[0].0.parse::<i32>().unwrap() * nums_to_mul[0].1.parse::<i32>().unwrap();
            }
        }
    }

    println!("STAGE 1 answer. p1 answer is {0}", p1_answer);
    println!("STAGE 2 answer. p2 answer is {0}", p2_answer);

}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
