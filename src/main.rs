pub mod command;
pub mod cat_file;
pub mod hash_object;
#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
use std::path::PathBuf;
//use anyhow::Ok;
use clap::{Parser,Subcommand};
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command
}
#[derive(Subcommand, Debug)]
pub enum Command { 
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,
        object_hash: String
    },
    HashObject {
        #[clap(short = 'w')]
        write: bool,
        file: PathBuf 
    }, 
}



fn main() -> anyhow::Result<()>{
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    let args = Args::parse();
    match args.command { 
        Command::Init => { 
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        },
        Command::CatFile { pretty_print, object_hash } => return cat_file::CatFile(pretty_print, object_hash),
        Command::HashObject { write, file } => hash_object::hash_object(&file, write)?,
    }
    // Uncomment this block to pass the first stage
    // let args: Vec<String> = env::args().collect();
    // if args[1] == "init" {
    //     
    // } else {
    //     println!("unknown command: {}", args[1])
    // }
    Ok(())
}
