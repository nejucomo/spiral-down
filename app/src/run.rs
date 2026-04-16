use clap::Parser;
use color_eyre::eyre::{eyre, Result, WrapErr};
use jiff::Zoned;

use crate::SpiralApp;

#[derive(Parser, Debug)]
#[command(author, version, about = "Spiral-down event countdown display")]
struct Args {
    events: Vec<String>,
}

pub fn run() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let mut events = vec![];

    for text in args.events {
        let (label, timetext) = text
            .rsplit_once('@')
            .ok_or(eyre!("expected `title @ timestamp`"))?;

        let label = label.trim().to_string();
        let timetext = timetext.trim();
        let time = parse_timestamp(timetext)
            .wrap_err_with(|| format!("Failed to parse timestamp {timetext:?}"))?;

        events.push((time, label));
    }

    SpiralApp::run(events)
}

fn parse_timestamp(s: &str) -> Result<Zoned> {
    todo!("parse {s:?}")
}
