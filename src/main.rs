#[allow(unused_imports)]
use std::env;
use std::ffi::CStr;
#[allow(unused_imports)]
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use anyhow::Context;
//use anyhow::Ok;
use clap::{Parser,Subcommand};
use flate2::bufread::ZlibDecoder;
use hex::decode;
use std::io::Write;
use anyhow::Error as AnyhowError;
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
    }  
}

enum Kind { 
    Blob
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
        Command::CatFile { pretty_print, object_hash } => { 
            anyhow::ensure!(pretty_print, "p flag must be there");
            let file = std::fs::File::open(format!(".git/objects/{}/{}", &object_hash[..2], &object_hash[2..])).context("read the hash file")?;
            let reader = BufReader::new(file);
            let mut z = ZlibDecoder::new(reader);
            let mut decoder_reader = BufReader::new(z);
            let mut buf = Vec::new();
            decoder_reader.read_until(0, &mut buf).context("read from .git/objects")?;
            let header = CStr::from_bytes_until_nul(&buf).expect("there is exactly one null bytes");
            let header = header.to_str().context("not valid utf-8 header")?;
            let Some((kind , size)) = header.split_once(' ') else {
                anyhow::bail!("")
            };
            let kind = match kind { 
                "blob" => {
                    Kind::Blob
                },
                _ =>  { 
                    anyhow::bail!("dont know how to print other kinds");
                }
            };


            let size = size.parse::<usize>().context("not valid size")?;
            let mut z = decoder_reader.take(size as u64);
            match kind { 
                Kind::Blob => { 
                    let mut stdout = std::io::stdout();
                    let n = std::io::copy(&mut z, &mut stdout)?;
                    anyhow::ensure!(n as usize == size, "git/objects are not of expected size : expected {size} actual: {n}");
                },
                _ => { 
                    anyhow::bail!("not our case to handle");
                }
            }
            
            
        }
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

