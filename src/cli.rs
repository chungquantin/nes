use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,

    #[structopt(long)]
    #[allow(unused)]
    print_asm: bool, // Future feature
}
