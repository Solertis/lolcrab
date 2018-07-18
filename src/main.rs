extern crate rand;
#[macro_use]
extern crate structopt;
extern crate termcolor;

mod ansi_escape;
mod lol;

use ansi_escape::AnsiEscaper;
use lol::{LolOpts, RainbowWriter};
use std::fs::File;
use std::io;
use std::io::BufReader;
use structopt::StructOpt;
use termcolor::{ColorChoice, StandardStream};

use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(name = "lolcat", about = "Terminal rainbows.")]
struct Cmdline {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(flatten)]
    lol_options: LolOpts,
}

fn main() -> Result<(), io::Error> {
    let opt = Cmdline::from_args();

    let outstream = StandardStream::stdout(ColorChoice::Always);
    let mut out = RainbowWriter::with_lol_opts(AnsiEscaper::new(outstream.lock()), &opt.lol_options);

    if let Some(path) = opt.input {
        let f = File::open(path)?;
        let mut file = BufReader::new(&f);
        io::copy(&mut file, &mut out)?;
    } else {
        let stdin = io::stdin();
        let mut input = stdin.lock();
        io::copy(&mut input, &mut out)?;
    }

    Ok(())
}
