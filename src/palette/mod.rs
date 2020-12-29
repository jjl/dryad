use druid::kurbo::{Circle, Line};
use druid::widget::prelude::*;
use druid::{AppLauncher, Color, Data, LocalizedString, Point, Rect, Vec2, WidgetPod, WindowDesc};
use crate::drawing_window::*;
use crate::*;

mod color_indicator;
use color_indicator::*;

mod tool_button;
use tool_button::*;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Palette {
    pub selected_tool: Tool,
}

impl Data for Palette {
    fn same(&self, other: &Palette) -> bool {
        self.selected_tool == other.selected_tool
    }
}

// We put the tools in an array because they're a uniform type and we
// can just iterate over them this way.

pub const TOOLS: usize = 6;

// yay, c-style enums for indexing an array
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(usize)]
pub enum Tool {
    ARROW = 0,
    RECTANGLE = 1,
    CIRCLE = 2,
    TEXT = 3,
    LINE = 4,
    PATH = 5,
}

impl Default for Tool {
    fn default() -> Tool { Tool::ARROW }
}

pub struct PaletteWidget {
    colors: [WidgetPod<Color,   ColorIndicatorWidget>; 2],
    tools:  [WidgetPod<Palette, ToolButtonWidget>;     TOOLS],
}
impl Default for PaletteWidget {
    fn default() -> Self {
        PaletteWidget {
            colors: [
                WidgetPod::new(ColorIndicatorWidget::new(StyleColor::LINE)),
                WidgetPod::new(ColorIndicatorWidget::new(StyleColor::FILL)),
            ],
            tools: [
                WidgetPod::new(ToolButtonWidget::new(Tool::ARROW)),
                WidgetPod::new(ToolButtonWidget::new(Tool::RECTANGLE)),
                WidgetPod::new(ToolButtonWidget::new(Tool::CIRCLE)),
                WidgetPod::new(ToolButtonWidget::new(Tool::TEXT)),
                WidgetPod::new(ToolButtonWidget::new(Tool::LINE)),
                WidgetPod::new(ToolButtonWidget::new(Tool::PATH)),
            ],
        }
    }
}

impl Widget<DrawingWindow> for PaletteWidget {

    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut DrawingWindow,
        env: &Env
    ) {
        for i in 0..2 {
            self.colors[i].event(ctx, event, &mut data.style.colors[i], env);
        }
        for i in 0..TOOLS {
            self.tools[i].event(ctx, event, &mut data.palette, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &DrawingWindow,
        env: &Env
    ) {
        for i in 0..2 {
            self.colors[i].lifecycle(ctx, event, &data.style.colors[i], env);
        }
        for i in 0..TOOLS {
            self.tools[i].lifecycle(ctx, event, &data.palette, env);
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &DrawingWindow,
        data: &DrawingWindow,
        env: &Env
    ) {
        for i in 0..2 {
            self.colors[i].update(ctx, &data.style.colors[i], env);
        }
        for i in 0..TOOLS {
            self.tools[i].update(ctx, &data.palette, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &DrawingWindow,
        env: &Env,
    ) -> Size {
        let min = Size::new(0.0, 0.0);
        let max = Size::new(bc.max().width, 50.0);
        let mut bc2 = BoxConstraints::new(min, max);
        let mut offset = 0.0;
        for i in 0..2 {
            let size = self.colors[i].layout(ctx, bc, &data.style.colors[i], env);
            bc2 = bc2.shrink(Size::new(size.width, 0.0));
            let r = Rect::new(offset, 0.0, offset + size.width, size.height);
            self.colors[i].set_layout_rect(ctx, &data.style.colors[i], env, r);
            offset += size.width;
        }
        for i in 0..TOOLS {
            let size = self.tools[i].layout(ctx, bc, &data.palette, env);
            bc2 = bc2.shrink(Size::new(size.width, 0.0));
            let r = Rect::new(offset, 0.0, offset + size.width, size.height);
            self.tools[i].set_layout_rect(ctx, &data.palette, env, r);
            offset += size.width;
        }
        Size::new(bc.max().width, 50.0)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &DrawingWindow, env: &Env) {
        for i in 0..2 {
            self.colors[i].paint(ctx, &data.style.colors[i], env);
        }
        for i in 0..TOOLS {
            self.tools[i].paint(ctx, &data.palette, env);
        }
    }
}
