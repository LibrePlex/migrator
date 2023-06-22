use anyhow::Result;
use clap::Parser;
use migraboss::entrypoint::{entry, Opts};

fn main() -> Result<()> {
    entry(Opts::parse())
}