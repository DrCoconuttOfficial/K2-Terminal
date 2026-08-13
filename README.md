# Frameless Slint App (Windows 11)

A minimal desktop app built with **Rust** and the **Slint** UI toolkit that runs as a
**frameless window** in the style of Windows 11:

- No native title bar (`no-frame`)
- Dark `#0A0A0A` background
- Custom **minimize / maximize / close** buttons drawn with the real Windows 11
  caption glyphs from the built-in **Segoe Fluent Icons** font
- Simple **hover / press animations** on the buttons (including the red close
  button, exactly like Windows 11)
- Draggable title bar + double-click to maximize
- Edge resizing of the frameless window

## Prerequisites

- **Windows 11** (the caption glyphs use the Segoe Fluent Icons font; Windows 10
  ships the same code points in Segoe MDL2 Assets)
- A recent stable **Rust** toolchain (install from <https://rustup.rs>)
- MSVC Build Tools (installed automatically by `rustup` when you pick the
  `x86_64-pc-windows-msvc` toolchain, which is the default on Windows)

## Build & run

```powershell
cargo run              # debug build
cargo run --release    # smaller, optimized build
```

The first build takes a while because Slint is a large dependency.

## Project layout

```
slint-frameless-app/
├── Cargo.toml          # slint + slint-build 1.17
├── build.rs            # compiles ui/app.slint into Rust code
├── src/
│   └── main.rs         # window dragging logic (the only Rust-side window call)
└── ui/
    └── app.slint       # the whole UI: title bar, buttons, content
```

## How it works

### Frameless window
`AppWindow inherits Window` declares `no-frame: true;`,
`background: #0A0A0A;` and `resize-border-width: 6px;` (lets the user resize
the frameless window by dragging its edges). On Windows 11 the DWM still
applies the default rounded corners to the frameless window.

### Caption buttons
Each button is a `WindowButton` component — a `Rectangle` with a `TouchArea`
and a `Text` glyph. The glyphs are the authentic Windows 11 ones from
**Segoe Fluent Icons**:

| Button      | Code point |
|-------------|------------|
| Minimize    | `U+E921`   |
| Maximize    | `U+E922`   |
| Restore     | `U+E923`   |
| Close       | `U+E8BB`   |

Buttons are 46×32 px, matching Windows 11. Hover/press colors:

| State    | Min / Max            | Close    |
|----------|----------------------|----------|
| idle     | transparent          | transparent |
| hover    | `#ffffff12` (subtle) | `#c42b1c` (red) |
| pressed  | `#ffffff24`          | `#d6402e` |

The hover animation is done with Slint `states` + `animate` (120 ms, ease).
The maximize button swaps its glyph to "restore" when the window is maximized.

### Window management
Slint 1.17's `Window` element exposes the window controls directly, so the
buttons call them from the `.slint` file:

```slint
clicked => { root.minimized = true; }            // minimize
clicked => { root.maximized = !root.maximized; } // maximize / restore
clicked => { root.hide(); }                      // close (ends the event loop)
```

### Dragging the window
Slint has no built-in "drag window" operation, so the title bar's `TouchArea`
feeds pointer positions into Rust, which repositions the window with
`set_position()`. The grab point is converted from logical to physical pixels
using `scale_factor()` so dragging stays accurate on high-DPI displays.
Double-clicking the bar toggles maximize.

## Notes & limitations

- **Snap layouts**: dragging is implemented manually here (works on every
  platform/backend). If you want Windows 11 snap layouts / snap assist during
  drags, call winit's native `drag_window()` from the pointer-down handler via
  Slint's `Window::with_winit_window` (the `backend-winit` feature is on by
  default) instead of the manual `set_position` approach.
- The app icon in the title bar is a plain colored square; swap it for a
  `@image-url("...")` image if you want a real icon.
- `resize-border-width` is winit-only; set it back to `0` if you prefer a fixed
  size frameless window.
