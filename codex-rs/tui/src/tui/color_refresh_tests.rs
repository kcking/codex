//! Focus refreshes keep the prompt shaded and preserve input arriving with terminal replies.

use super::TuiEvent;
use super::event_stream::EventBroker;
use super::event_stream::TuiEventStream;
use crate::terminal_probe::DefaultColors;
use crossterm::event::KeyCode;
use pretty_assertions::assert_eq;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::os::fd::FromRawFd;
use std::process::Command;
use std::process::Stdio;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::time::Duration;
use std::time::Instant;
use tokio_stream::StreamExt;

const CHILD_TEST: &str = "tui::color_refresh_tests::focus_color_refresh_pty_child";
const DARK: DefaultColors = DefaultColors {
    fg: (240, 240, 240),
    bg: (16, 16, 16),
};
const LIGHT: DefaultColors = DefaultColors {
    fg: (16, 16, 16),
    bg: (240, 240, 240),
};

#[test]
fn color_reports_refresh_prompt_without_focus_and_preserve_input() {
    let mut master = -1;
    let mut slave = -1;
    // Safety: openpty initializes both owned descriptors; no initial size or termios is needed.
    assert_eq!(
        unsafe {
            libc::openpty(
                &mut master,
                &mut slave,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        },
        0
    );
    // Safety: openpty returned owned descriptors on success.
    let mut master = unsafe { File::from_raw_fd(master) };
    let slave = unsafe { File::from_raw_fd(slave) };
    let mut child = Command::new(std::env::current_exe().expect("test binary"))
        .args(["--exact", CHILD_TEST, "--ignored", "--nocapture"])
        .env("CODEX_TUI_COLOR_REFRESH_PTY_CHILD", "1")
        .stdin(slave.try_clone().expect("clone terminal"))
        .stdout(slave)
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn terminal child");

    let deadline = Instant::now() + Duration::from_secs(/*secs*/ 10);
    let mut output = Vec::new();
    let mut scanned = 0;
    let mut queries = 0;
    let status = loop {
        if let Some(status) = child.try_wait().expect("poll child") {
            break status;
        }
        if Instant::now() >= deadline {
            child.kill().expect("kill timed out child");
            break child.wait().expect("wait for timed out child");
        }
        let mut fd = libc::pollfd {
            fd: master.as_raw_fd(),
            events: libc::POLLIN,
            revents: 0,
        };
        // Safety: fd remains valid for this bounded poll.
        if unsafe {
            libc::poll(&mut fd, /*nfds*/ 1, /*timeout*/ 10)
        } <= 0
            || fd.revents & libc::POLLIN == 0
        {
            continue;
        }
        let mut buffer = [0; 4096];
        if let Ok(count) = master.read(&mut buffer) {
            output.extend_from_slice(&buffer[..count]);
        }
        let query = b"\x1b]11;?\x1b\\";
        while let Some(start) = output[scanned..]
            .windows(query.len())
            .position(|window| window == query)
        {
            scanned += start + query.len();
            queries += 1;
            let colors: &[u8] = match queries {
                1 | 3 => b"\x1b]10;rgb:1010/1010/1010\x07\x1b]11;rgb:f0f0/f0f0/f0f0\x1b\\",
                2 | 5 => b"\x1b]11;rgb:1010/1010/1010\x07\x1b]10;rgb:f0f0/f0f0/f0f0\x1b\\",
                4 => b"", // A transient timeout must retain the last successful palette.
                _ => panic!("unexpected probe"),
            };
            master
                .write_all(b"a\x1b[200~pasted text\x1b[201~")
                .expect("type during probe");
            master.write_all(colors).expect("reply to probe");
            // Exercise the normal input reader after the probe has closed its handles.
            std::thread::sleep(Duration::from_millis(/*millis*/ 20));
            master.write_all(b"b").expect("type after reply");
            if queries < 5 {
                // Notify while the event reader is idle, without a focus report. Splitting
                // after Escape exercises live framing rather than only buffered input.
                std::thread::sleep(Duration::from_millis(/*millis*/ 20));
                let notification: &[u8] = match queries {
                    1 | 3 => b"\x1b[?997;1nq",
                    2 => b"\x1b[?997;2nq",
                    4 => b"\x1b[Iq", // Focus refresh remains the compatibility fallback.
                    _ => unreachable!(),
                };
                master.write_all(&notification[..1]).expect("start report");
                std::thread::sleep(Duration::from_millis(/*millis*/ 3));
                master.write_all(&notification[1..]).expect("finish report");
            }
        }
    };
    let mut stderr = String::new();
    child
        .stderr
        .take()
        .expect("stderr")
        .read_to_string(&mut stderr)
        .expect("read stderr");
    assert!(
        status.success(),
        "{stderr}\n{}",
        String::from_utf8_lossy(&output)
    );
    assert_eq!(queries, 5);
}

#[test]
#[ignore]
fn focus_color_refresh_pty_child() {
    if std::env::var_os("CODEX_TUI_COLOR_REFRESH_PTY_CHILD").is_none() {
        return;
    }
    crossterm::terminal::enable_raw_mode().expect("raw terminal");
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    crate::terminal_palette::with_test_default_colors(DARK, || {
        runtime.block_on(async {
            let broker: Arc<EventBroker> = Arc::new(EventBroker::new());
            let (_draw_tx, draw_rx) = tokio::sync::broadcast::channel(1);
            let focused = Arc::new(AtomicBool::new(true));
            let mut stream = TuiEventStream::new(
                broker,
                draw_rx,
                focused.clone(),
                super::job_control::SuspendContext::new(),
                Arc::new(AtomicBool::new(false)),
            );
            let dark_style = crate::style::user_message_style();
            crossterm::event::buffer_input(b"\x1b[Iq").expect("queue focus and key");
            for (index, expected) in [LIGHT, DARK, LIGHT, LIGHT, DARK].into_iter().enumerate() {
                let event = stream.next().await;
                if index == 0 || index == 4 {
                    assert!(matches!(event, Some(TuiEvent::FocusGained)));
                    assert!(focused.load(Ordering::Relaxed));
                } else {
                    assert!(matches!(event, Some(TuiEvent::Draw)));
                    assert!(!focused.load(Ordering::Relaxed));
                }
                assert_eq!(crate::terminal_palette::default_bg(), Some(expected.bg));
                assert_eq!(crate::terminal_palette::default_fg(), Some(expected.fg));
                assert_eq!(crate::style::user_message_style() == dark_style, expected == DARK);
                for key in ['q', 'a'] {
                    assert!(matches!(stream.next().await, Some(TuiEvent::Key(event)) if event.code == KeyCode::Char(key)));
                }
                assert!(matches!(stream.next().await, Some(TuiEvent::Paste(text)) if text == "pasted text"));
                assert!(matches!(stream.next().await, Some(TuiEvent::Key(event)) if event.code == KeyCode::Char('b')));
                focused.store(false, Ordering::Relaxed);
            }
        });
    });
    crossterm::terminal::disable_raw_mode().expect("restore terminal");
}
