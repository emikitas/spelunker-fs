pub mod libs;

use clap::Parser;
use crate::libs::walk;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String, // TODO: make cwd the default value somehow
}

fn main(){
    let args = Args::parse();
    println!("Starting at {}", args.path);
    match walk::walk::list_dir(args.path) {
        Ok(v) => v,
        Err((err, v)) => {
            println!("mapping failed with: {:?} but here's what we got: {:?}", err, v);
            v
        },
    }.iter().for_each(|entity| println!("{}", entity));
}
