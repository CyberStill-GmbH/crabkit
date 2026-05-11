/*
Parser the command line in the CLI input as
the first important task in main function.

NEXT SUNDAY: Probaré si funciona el cliente para una solicitud
Ando cansado ya son las 3a.m y de leer pura docu de Rust asu...
haré eso e implementar Reporter para visualizar las salidas en CLI
*/

// import the function modules
mod io;
use io::streamer::read_wordlist;

use std::path::PathBuf;
use clap::{Parser};




#[derive(Parser, Debug)]
#[command(name = "crabkit", version, about = "",)]

// Define the flags on CLI
struct Cli {
    /// Objective target
    #[arg(short, long)]
    target: String,
    
    /// Path to dictionary
    #[arg(short, long, value_name = "FILE")]
    wordlist: Option<PathBuf>,

    /// Thread number or concurrent tasks
    #[arg(short, long)]
    concurrency: Option<usize>,



}

fn main() -> std::io::Result<()> {
   let args: Cli = Cli::parse();
   println!("target: {}", args.target);

   if let Some(w) = args.wordlist {
    println!("Wordlist: {}", w.display());
   }

   let final_concurrency: usize = args.concurrency.unwrap_or( 10);
   println!("Concurrency: {}", final_concurrency);

   let entries = read_wordlist("src/wordlist.txt")?;

   for entry in entries {
    println!("{}", entry);
   }
   Ok(())
} 