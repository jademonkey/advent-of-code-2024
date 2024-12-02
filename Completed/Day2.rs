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


    let mut reports = Vec::new();
    let filename;
    if cli.full {
        filename = "./Inputs/Day2";
    } else {
        filename = "./Inputs/Example2";
    }

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        // read the lines and the numbers into their lists
        for line in lines.flatten() {
            if cli.debug {
              println!("Line in: {}", line);
            }
            let mut levels  = Vec::new();
            let mut nums = line.split_whitespace();
            let mut it_val = nums.next();
            while it_val != None {
                levels.push(it_val.unwrap().parse::<i32>().unwrap());
                it_val = nums.next();
            }
            if cli.debug {
              println!("Level generated: {:?}", levels);
            }
            reports.push(levels);
        }
        if cli.debug {
            println!("Reports: {:?}\n", reports);
        }
    }

    let mut p1_safes = 0;
    let mut p2_safes = 0;
    for n in 0..reports.len() {
        if cli.debug {
            print!("level: {:?} = ", reports[n]);
        }
        let index_to_remove = is_safe(reports[n].clone(), cli.debug);
        if index_to_remove == 0 {
            p1_safes += 1;
            p2_safes += 1;
            if cli.debug {
                println!(" SAFE!");
            }
        } else {
            if cli.debug {
                println!(" UNSAFE because of index {0}!", index_to_remove);
            }
            reports[n].remove(index_to_remove);
            
            if cli.debug {
                print!("   Level now {:?}. Attempt 2 =  ", reports[n]);
            }

            if is_safe(reports[n].clone(), cli.debug) == 0 {
              p2_safes += 1;
              if cli.debug {
                println!(" now Safe!");
            }
            } else {
                if cli.debug {
                    println!(" still UNSAFE!");
                }
            }
        }
    }

    println!("STAGE 1 answer. p1_safes is {0}", p1_safes);
    println!("STAGE 2 answer. p2_safes is {0}", p2_safes);

}


// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_safe(levels: Vec<i32>, debug: bool) -> usize {
    let descending;
    if levels[0] > levels[1] {
        if debug {
            print!("descending");
        }
        descending = true;
    } else if levels[0] < levels[1] {
        if debug {
            print!("Ascending");
        }
        descending = false;
    } else {
        /* Huh? */
        if debug {
            print!("equal thus bad");
        }
        return 1;
    }
    for n in 0..levels.len() {
        if n + 1 == levels.len() {
            /* no more checks */
            break;
        }
        /* descending check */
        let diff;
        if levels[n] == levels[n+1] {
            if debug {
                print!(" failed, {0} is equal to {1}",levels[n], levels[n+1] );
            }
            // not ok!
            return n;
        }
        if descending {
            if levels[n] < levels[n+1] {
                if debug {
                    print!(" failed, {0} is smaller than {1}",levels[n], levels[n+1] );
                }
                // not ok!
                return n;
            } 
            diff = levels[n] - levels[n+1];
        } else {
            if levels[n] > levels[n+1] {
                if debug {
                    print!(" failed, {0} is bigger than {1}",levels[n], levels[n+1] );
                }
                // not ok!
                return n;
            }
            diff = levels[n+1] - levels[n];
        }
        /* range check */
        if diff < 1 || diff > 3 {
            if debug {
                print!(" failed, {0} and {1} have a diff of {2}",levels[n], levels[n+1], diff );
            }
            return n+1;
        }
    }
    0
}
