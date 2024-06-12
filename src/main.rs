mod args;
mod config;
mod java;
mod run;
mod utils;
mod generate;


use crate::config::*;
use crate::run::Run;

fn main() {
    let config: Config = Config::new();
    let mut run = Run::new();
    //    download_jdk(22);
    //    make_file_tree("homiecraft");
    println!("{:?}", config);
}
