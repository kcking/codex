#![allow(clippy::cognitive_complexity)]

use crossterm::cursor::position;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::read;
use crossterm::execute;
use std::io::Write;
use std::io::{self};

fn test_event<W>(w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    execute!(w, EnableMouseCapture)?;

    loop {
        // Blocking read
        let event = read()?;

        println!("Event::{:?}\r", event);

        if event == Event::Key(KeyCode::Char('c').into()) {
            println!("Cursor position: {:?}\r", position());
        }

        if event == Event::Key(KeyCode::Char('q').into()) {
            break;
        }
    }

    execute!(w, DisableMouseCapture)?;

    Ok(())
}

pub fn run<W>(w: &mut W) -> std::io::Result<()>
where
    W: Write,
{
    run_tests!(w, test_event);
    Ok(())
}
