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

    /// Trigger to remove animated 'followpoint' element.
    /// If enabled, the tool will remove all 'followpoint' elements with suffix of numbers
    #[clap(short, long, action)]
    followpoint: bool,

    /// Trigger to remove all animated 'hit' related elements.
    /// If enabled, the tool will remove all 'hit' related elements with suffix of numbers
    #[clap(short('H'), long, action)]
    hits: bool,

    /// Specify the frame number to use as non-animated 'followpoint' element
    #[clap(long, value_parser, value_name("untyped int"), default_value_t = 0)]
    pre_followpoint_number: u32,

    /// Specify the frame number to use as non-animated 'hit' related elements
    #[clap(long, value_parser, value_name("untyped int"), default_value_t = 0)]
    pre_hits_number: u32,

    /// Specify the frame number to use as non-animated 'menu-back' element
    #[clap(long, value_parser, value_name("untyped int"), default_value_t = 0)]
    pre_menu_back_number: u32,

    /// Specify the frame number to use as non-animated 'play-skip' element
    #[clap(long, value_parser, value_name("untyped int"), default_value_t = 0)]
    pre_play_skip_number: u32,
}

// Used for finding all animated elements
const REGEX: &str = "(hit0|hit50|hit100|hit100k|hit300|hit300k|hit300g|menu-back|play-skip|scorebar-colour)-.*(\\d+)";
const REGEX_F: &str = "(followpoint|hit0|hit50|hit100|hit100k|hit300|hit300k|hit300g|menu-back|play-skip|scorebar-colour)-.*(\\d+)";

// Used for selecting specified frame of animated elements
const REGEX_H_BASE: &str = "(hit0|hit50|hit100|hit100k|hit300|hit300k|hit300g)-{}.*";
const REGEX_BACK_BASE: &str = "menu-mack-{}.*";
const REGEX_NUMS: &str = r"-[0-9]";

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    if !attention_removal() {
        println!("Abort!");
        exit(0);
    }

    remove_files(args).await.expect("Failed to remove animated elements");

    println!("Completed!")
}

async fn remove_files(s: Cli) -> Result<(), Error> {
    let r: Regex;
    if !s.followpoint {
        r = Regex::new(REGEX).unwrap();
    } else {
        r = Regex::new(REGEX_F).unwrap();
    }

    let r_h = Regex::new(REGEX_H_BASE.replace("{}", format!("{}", s.pre_hits_number).as_str()).as_str()).unwrap();
    let r_num = Regex::new(REGEX_NUMS).unwrap();

    let dir = fs::read_dir(s.path).expect("Failed to read specified directory");

    for d in dir {
        let f = d.expect("Failed to read file in specified directory");
        if r.is_match(f.file_name().to_str().unwrap()) {
            if !s.hits {
                if r_h.is_match(f.file_name().to_str().unwrap()) {
                    fs::rename(f.path(), r_num.replace_all(f.path().to_str().unwrap(), "-0").to_string()/* dest path; must refactor */).expect("Failed to rename file as frame 0");
                    continue;
                }
            }
            fs::remove_file(f.path().to_str().unwrap()).expect("Failed to remove file");
        }
    }

    Ok(())
}

fn attention_removal() -> bool {
    let q = Question::new("Before proceeding, make sure non-animated element has been prepared.\nDo you want to continue?")
        .confirm();

    match q {
        Answer::YES => true,
        Answer::NO => false,
        _ => false
    }
}