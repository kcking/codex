//! Shared coverage for live reports in both Unix input backends.

use super::Instant;
use super::Parser;
use crate::event::ColorScheme;
use crate::event::Event;
use crate::event::KeyCode;
use crate::event::internal::InternalEvent;

#[test]
fn live_color_scheme_reports_survive_every_read_boundary() {
    for (report, scheme) in [
        (b"\x1b[?997;1n", ColorScheme::Dark),
        (b"\x1b[?997;2n", ColorScheme::Light),
    ] {
        for split in 1..report.len() {
            let mut parser = Parser::default();
            parser.advance(&report[..split], false);
            assert_eq!(parser.next(), None, "split at {split}");
            parser.advance(&report[split..], false);
            parser.advance(b"x", false);
            assert_eq!(
                parser.next(),
                Some(InternalEvent::Event(Event::ColorSchemeChanged(scheme)))
            );
            assert_eq!(
                parser.next(),
                Some(InternalEvent::Event(Event::Key(KeyCode::Char('x').into())))
            );
            assert_eq!(parser.next(), None);
        }
    }
}

#[test]
fn malformed_color_reports_do_not_swallow_following_input() {
    for report in [
        b"\x1b[?997;0n".as_slice(),
        b"\x1b[?997;3n",
        b"\x1b[?997;1;2n",
        b"\x1b[?997;01n",
        b"\x1b[?996;1n",
    ] {
        let mut parser = Parser::default();
        parser.advance(report, false);
        parser.advance(b"x", false);
        assert_eq!(
            parser.next(),
            Some(InternalEvent::Event(Event::Key(KeyCode::Char('x').into())))
        );
        assert_eq!(parser.next(), None);
    }
}

#[test]
fn standalone_and_repeated_escape_still_emit_keys() {
    let mut parser = Parser::default();
    parser.advance(b"\x1b", false);
    assert_eq!(parser.next(), None);
    parser.advance(b"\x1b", false);
    let escape = Some(InternalEvent::Event(Event::Key(KeyCode::Esc.into())));
    assert_eq!(parser.next(), escape);
    parser.pending_escape_deadline = Some(Instant::now());
    assert_eq!(parser.finish_pending_escape(), escape);
}

#[cfg(feature = "bracketed-paste")]
#[test]
fn pasted_color_reports_remain_literal_paste() {
    let mut parser = Parser::default();
    parser.advance(b"\x1b[200~\x1b[?997;2n\x1b[201~", false);
    assert_eq!(
        parser.next(),
        Some(InternalEvent::Event(Event::Paste("\x1b[?997;2n".into())))
    );
    assert_eq!(parser.next(), None);
}
