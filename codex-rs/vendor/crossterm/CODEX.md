# Local Crossterm patch

Source: https://github.com/openai-oss-forks/crossterm
Revision: ed1cdab335221515706178d68495bba2aed1924f
License: MIT (see LICENSE).

This copy preserves the pinned input fixes and adds DEC mode 2031 reporting:
`EnableColorSchemeChange`, `DisableColorSchemeChange`, and
`Event::ColorSchemeChanged(ColorScheme)` for exact `CSI ?997;1n` and
`CSI ?997;2n` reports. Other private status reports remain ignored. Live Escape prefixes use
the existing bounded ambiguity window so a fragmented notification is not emitted as typing.
The upstream color-format tests are serialized because they mutate shared ANSI
color settings and now run as part of this workspace.
Source files are formatted to the repository Rust style.

Remove this copy and restore the Git dependency once these changes are available
in a compatible upstream revision.
