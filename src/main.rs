use std::cell::RefCell;
use std::rc::Rc;

use slint::ComponentHandle;

slint::include_modules!();

/// State captured when the user starts dragging the window by the title bar.
struct DragState {
    /// Window position when the drag started (physical pixels).
    origin: slint::PhysicalPosition,
    /// Grab point inside the title bar (logical pixels).
    grab: (f32, f32),
    /// Display scale factor (physical px per logical px).
    scale: f32,
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    // ---- window dragging ----------------------------------------------
    //
    // Minimize / maximize / restore / close are handled directly in the
    // Slint code via the Window element's built-in `minimized`, `maximized`
    // properties and `hide()` function. Dragging still needs Rust because
    // Slint has no built-in "drag window" operation.

    let drag: Rc<RefCell<Option<DragState>>> = Rc::new(RefCell::new(None));

    {
        let weak = app.as_weak();
        let drag = drag.clone();
        app.on_begin_drag(move |x, y| {
            if let Some(app) = weak.upgrade() {
                let win = app.window();
                *drag.borrow_mut() = if win.is_maximized() {
                    None // dragging a maximized window does nothing
                } else {
                    Some(DragState {
                        origin: win.position(),
                        grab: (x, y),
                        scale: win.scale_factor(),
                    })
                };
            }
        });
    }

    {
        let weak = app.as_weak();
        let drag = drag.clone();
        app.on_update_drag(move |x, y| {
            if let Some(app) = weak.upgrade() {
                if let Some(state) = drag.borrow().as_ref() {
                    // Keep the grabbed point under the cursor, converting the
                    // logical delta to physical pixels via the scale factor.
                    let dx = (x - state.grab.0) as f64 * state.scale as f64;
                    let dy = (y - state.grab.1) as f64 * state.scale as f64;
                    app.window().set_position(slint::PhysicalPosition::new(
                        (state.origin.x as f64 + dx).round() as i32,
                        (state.origin.y as f64 + dy).round() as i32,
                    ));
                }
            }
        });
    }

    {
        let drag = drag.clone();
        app.on_end_drag(move || {
            *drag.borrow_mut() = None;
        });
    }

    app.run()
}
