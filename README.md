# Tauri macOS keyboard shortcut repro

Minimal reproduction of a Tauri v2 macOS bug: **the standard editing keyboard
shortcuts (<kbd>Cmd</kbd>+<kbd>C</kbd>/<kbd>V</kbd>/<kbd>X</kbd>/<kbd>A</kbd>/<kbd>Z</kbd>)
do nothing in the webview, even though the equivalent menu items work.**

## The bug

With the default menu (`Menu::default()`), Tauri on macOS builds a standard
`Edit` submenu (`Undo`, `Redo`, `Cut`, `Copy`, `Paste`, `Select All`) whose
items carry the correct key equivalents.

However:

- **Clicking** the menu items works (they dispatch their native selectors
  `cut:`/`copy:`/`paste:`/`selectAll:` to the focused webview).
- **Pressing the keyboard equivalents does nothing** — no beep, no action. The
  key event reaches the app (a local `NSEvent` monitor sees it), but the menu's
  `performKeyEquivalent:` never fires the item.

This affects any webview, including the built-in Web Inspector (devtools).

## Reproduce

```sh
cd src-tauri
cargo run
```

Then, in the window:

1. Type some text into the `<input>` or `<textarea>`.
2. Select it and press <kbd>Cmd</kbd>+<kbd>C</kbd>, <kbd>Cmd</kbd>+<kbd>X</kbd>,
   <kbd>Cmd</kbd>+<kbd>V</kbd>, <kbd>Cmd</kbd>+<kbd>A</kbd>, <kbd>Cmd</kbd>+<kbd>Z</kbd>.

## Expected vs actual

| Action                    | Expected        | Actual          |
| ------------------------- | --------------- | --------------- |
| Edit > Cut (menu click)   | cuts            | cuts            |
| Edit > Copy (menu click)  | copies          | copies          |
| Edit > Paste (menu click) | pastes          | pastes          |
| <kbd>Cmd</kbd>+<kbd>X</kbd> | cuts          | **nothing**     |
| <kbd>Cmd</kbd>+<kbd>C</kbd> | copies        | **nothing**     |
| <kbd>Cmd</kbd>+<kbd>V</kbd> | pastes        | **nothing**     |
| <kbd>Cmd</kbd>+<kbd>A</kbd> | select all    | **nothing**     |
| <kbd>Cmd</kbd>+<kbd>Z</kbd> | undo          | **nothing**     |

## Environment

- macOS (reproduced on Apple Silicon)
- Tauri `2.x` (tested against 2.11.5)
- Rust stable
- Single window, single webview — no custom menu, no plugins

## Notes / workarounds

The keyboard shortcuts can be restored by intercepting key events at the app
level (an `NSEvent` local monitor) and dispatching the selectors yourself, or
by injecting an `execCommand` shim into the webview — but:

- `selectAll:`/`undo:`/`redo:` are **not** direct responder methods on
  `WKWebView` the way `cut:`/`copy:`/`paste:` are, so even `sendAction:` with
  those selectors is a no-op for webview content.
- The Web Inspector runs as a separate WebKit UI, so it can't be driven this
  way at all.

This points to the macOS menu plumbing (`performKeyEquivalent:` → action
dispatch) not working for the webview, which is the underlying issue to fix.
