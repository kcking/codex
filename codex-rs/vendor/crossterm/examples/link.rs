//! Demonstrates OSC 8 hyperlink support.
//!
//! cargo run --example link

use std::io;

use crossterm::execute;
use crossterm::style::EndHyperlink;
use crossterm::style::Print;
use crossterm::style::StartHyperlink;

fn main() -> io::Result<()> {
    let mut out = io::stdout();

    execute!(
        out,
        Print("Visit: "),
        StartHyperlink::new("https://github.com/crossterm-rs/crossterm"),
        Print("crossterm"),
        EndHyperlink,
        Print("\n"),
    )
}
