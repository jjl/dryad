use druid::{AppLauncher, Color, LocalizedString, Point, Vec2, WindowDesc};
use dryad::drawing_window::*;

pub fn main() {
    let window = WindowDesc::new(|| DrawingWindowWidget::default()).title(
        LocalizedString::new("app-title")
            .with_placeholder("Dryad")
    );
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch(DrawingWindow::default())
        .expect("it's even more broken than it's supposed to be.");
}
