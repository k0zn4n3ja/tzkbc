use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(long, default_value = "8000")]
    pub port: u16,

    #[structopt(long, use_delimiter = true)]
    pub peers: Vec<String>,
}
