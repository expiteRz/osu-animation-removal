use std::fmt::Error;
use std::fs;
use std::process::exit;
use clap::Parser;
use question::{Answer, Question};
use regex::Regex;

#[derive(Parser)]
#[clap(name = "osu-animation-frame-removal")]
#[clap(author = "Rz")]
#[clap(version = "0.1.0")]
#[clap(about = "Removes animation frames to create lite version for your osu! skin", long_about = None)]
struct Cli {
    /// Specify the path to animated elements
    #[clap(value_parser)]
    path: String,

    /// Includes 'followpoint' element into removal candidate
    #[clap(short, long, action)]
    followpoint: bool,
}

const REGEX: &str = "(hit0|hit50|hit100|hit100k|hit300|hit300k|hit300g|menu-back|play-skip|scorebar-colour)-.*(\\d+)";
const REGEX_F: &str = "(followpoint|hit0|hit50|hit100|hit100k|hit300|hit300k|hit300g|menu-back|play-skip|scorebar-colour)-.*(\\d+)";

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    if !attention_removal() {
        println!("Abort!");
        exit(0);
    }

    remove_files(args.path, args.followpoint).await.expect("Failed to remove animated elements");

    println!("Completed!")
}

async fn remove_files(p: String, f: bool) -> Result<(), Error> {
    let r: Regex;
    if !f {
        r = Regex::new(REGEX).unwrap();
    } else {
        r = Regex::new(REGEX_F).unwrap();
    }

    let dir = fs::read_dir(p).expect("Failed to read specified directory");

    for d in dir {
        let f = d.expect("Failed to read file in specified directory");
        if r.is_match(f.file_name().to_str().unwrap()) {
            fs::remove_file(f.path().to_str().unwrap()).expect("Failed to remove file");
        }
    }

    Ok(())
}

fn attention_removal() -> bool {
    loop {
        let q = Question::new("Before proceeding, make sure non-animated element has been prepared.\nDo you want to continue?")
            .yes_no()
            .until_acceptable()
            .ask();

        match q.unwrap() {
            Answer::YES => return true,
            Answer::NO => return false,
            _ => continue
        }
    }
}