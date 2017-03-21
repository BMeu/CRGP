//! Execute the graph-parallel cascade reconstruction from the command line.

extern crate crgp;
extern crate getopts;

use std::env;
use std::process;

use getopts::{Matches, Options};

use crgp::algorithm;

/// The exit codes returned by the program.
pub enum ExitCode {
    /// Successful (i.e. expected) execution (Code: `0`).
    Success = 0,

    /// Invalid program parameters (Code: `1`).
    IncorrectUsage = 1
}

/// Execute the program.
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let program: &str = &arguments[0].clone();

    // Define defaults.
    let default_batch_size: usize = 500;

    // Define the program options.
    let mut options = Options::new();
    options.optopt("b", "batch-size",
                   &format!("Size of retweet batches. [Default: {default}]", default = default_batch_size), "SIZE");
    options.optopt("f", "hostfile", "A text file specifying \"hostname:port\" per line in order of process identity.",
                   "FILE");
    options.optflag("h", "help", "Show this usage message and exit.");
    options.optopt("n", "processes", "Number of processes involved in the computation.", "PROCESSES");
    options.optopt("p", "process", "Identity of this process; from 0 to n-1.", "ID");
    options.optflag("r", "print-result", "Print the computed influence edges to stdout.");
    options.optopt("w", "workers", "Number of per-process workers threads.", "WORKERS");

    // Parse the options.
    let specified_options: Matches = match options.parse(&arguments[1..]) {
        Ok(option_matches) => option_matches,
        Err(message) => {
            println!("Error: {error}", error = message.to_string());
            print_usage(program, options);
            process::exit(ExitCode::IncorrectUsage as i32);
        }
    };

    // If the help option is specified, show the usage information and exit the program.
    if specified_options.opt_present("h") {
        print_usage(program, options);
        process::exit(ExitCode::Success as i32);
    }

    // Get the parsed options.
    let print_result: bool = specified_options.opt_present("r");
    let batch_size: usize = match specified_options.opt_str("b") {
        Some(batch_size) => {
            match batch_size.parse::<usize>() {
                Ok(batch_size) => batch_size,
                Err(message) => {
                    println!("Error: {error}", error = message.to_string());
                    print_usage(&program, options);
                    process::exit(ExitCode::IncorrectUsage as i32);
                }
            }
        },
        None => default_batch_size
    };

    // Create the arguments for the timely execution.
    let mut timely_arguments: Vec<String> = Vec::new();
    if let Some(hostfile) = specified_options.opt_str("f") {
        timely_arguments.push("-h".to_string());
        timely_arguments.push(hostfile.to_string());
    }
    if let Some(processes) = specified_options.opt_str("n") {
        timely_arguments.push("-n".to_string());
        timely_arguments.push(processes.to_string());
    }
    if let Some(process) = specified_options.opt_str("p") {
        timely_arguments.push("-p".to_string());
        timely_arguments.push(process.to_string());
    }
    if let Some(workers) = specified_options.opt_str("w") {
        timely_arguments.push("-w".to_string());
        timely_arguments.push(workers.to_string());
    }
    let timely_arguments = timely_arguments.into_iter();

    // Get the paths to the data sets.
    if specified_options.free.len() < 2 {
        let message: &str = match specified_options.free.len() {
            0 => "Neither friends data set nor retweet data set are given.",
            1 => "No retweet data set is given.",
            _ => ""
        };
        println!("Error: {error}", error = message.to_string());
        print_usage(program, options);
        process::exit(ExitCode::IncorrectUsage as i32);
    }
    let friendship_dataset: String = specified_options.free[0].clone();
    let retweet_dataset: String = specified_options.free[1].clone();

    // Execute the reconstruction.
    algorithm::execute(friendship_dataset, retweet_dataset, batch_size, print_result, timely_arguments);
    process::exit(ExitCode::Success as i32);
}

/// Print a usage message.
fn print_usage(program: &str, options: Options) {
    let brief = &format!("Usage: {executable} <options> [FRIENDS] [RETWEETS]", executable = program);
    println!("{}", options.usage(brief));
}
