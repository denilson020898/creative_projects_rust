use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    #[structopt(short = "r", long = "result", parse(from_os_str))]
    result_file: PathBuf,

    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    println!("{:#?}", Opt::from_args());
}
