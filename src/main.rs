use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use clap::Parser;

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


    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    let filename;
    if cli.full {
        filename = "./Inputs/Day1";
    } else {
        filename = "./Inputs/Example1";
    }

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        // read the lines and the numbers into their lists
        for line in lines.flatten() {
            if cli.debug {
              println!("{}", line);
            }
            let mut nums = line.split_whitespace();
            left_list.push(nums.next().unwrap().parse::<i32>().unwrap());
            right_list.push(nums.next().unwrap().parse::<i32>().unwrap());
        }
    }
    // Sort the lists
    left_list.sort();
    right_list.sort();
    if cli.debug {
        println!("{:?}", left_list);
        println!("{:?}", right_list);
    }

    let mut diff = 0;
    for n in 0..left_list.len() {
        let c_diff = left_list[n] - right_list[n];
        diff += c_diff.abs();
    }

    println!("Part1: The total diff is {0}", diff);

    let mut total_similarity_score = 0;
    for n in 0..left_list.len() {
        let mut count_right = 0;

        for n2 in 0..right_list.len() {
            if left_list[n] == right_list[n2] {
                count_right += 1;
            }
        }

        total_similarity_score += left_list[n] * count_right;
    }

    println!("Part2: The total similarity score is {0}", total_similarity_score);
}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
