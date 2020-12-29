use druid::{Color, Rect, Size};

pub mod palette;
pub mod drawing;
pub mod drawing_window;

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum StyleColor {
    LINE = 0,
    FILL = 1,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StyleManual {
    pub colors: [Color; 2],
}

impl Default for StyleManual {
    fn default() -> Self {
        StyleManual {
            colors: [Color::BLACK, Color::BLUE],
        }
    }
}    

// loldruid

pub trait RectExt {
    fn translate(&self, by: Size) -> Rect;
}

impl RectExt for Rect {
    fn translate(&self, by: Size) -> Rect {
        Rect::new(self.x0 + by.width, self.y0 + by.height, self.x1 + by.width, self.y1 + by.height)
    }
}

pub trait SizeExt {
    fn invert(&self) -> Size;
}

impl SizeExt for Size {
    fn invert(&self) -> Size {
        Size::new(self.width * -1.0, self.height * -1.0)
    }
}
