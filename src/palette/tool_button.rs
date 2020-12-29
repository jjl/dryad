use druid::widget::prelude::*;
use druid::{Color, Point, Rect};
use super::*;
use crate::*;

#[derive(Clone,Debug)]
pub struct ToolButtonWidget {
    pub tool: Tool,
}

impl ToolButtonWidget {
    pub fn new(tool: Tool) -> Self {
        ToolButtonWidget { tool }
    }
}

impl Widget<Palette> for ToolButtonWidget {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut Palette,
        env: &Env) {
        match event {
            Event::MouseUp(e) => {
                data.selected_tool = self.tool;
                ctx.request_paint();
            }
            _ => {}
        }
    }
    
    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Palette,
        _env: &Env
    ) {
        
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &Palette,
        _data: &Palette,
        _env: &Env
    ) {

    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &Palette,
        _env: &Env,
    ) -> Size {
        Size::new(50.0, 50.0)
    }

    fn paint(
        &mut self,
        ctx: &mut PaintCtx,
        data: &Palette,
        _env: &Env
    ) {
        match self.tool {
            Tool::ARROW => {
                ctx.fill(
                    Rect::new(3.0, 3.0, 47.0, 47.0),
                    self.color(data, self.tool)
                );
            }
            Tool::RECTANGLE => {
                ctx.fill(
                    Rect::new(10.0, 10.0, 40.0, 40.0),
                    self.color(data, self.tool)
                );
            }
            Tool::CIRCLE => {
                ctx.fill(
                    Circle::new(Point::new(25.0, 25.0), 15.0),
                    self.color(data, self.tool)
                );
            }
            Tool::TEXT => {
                ctx.stroke(
                    Line::new(Point::new(17.5, 10.0), Point::new(32.5, 10.0)),
                    self.color(data, self.tool),
                    3.0
                );
                ctx.stroke(
                    Line::new(Point::new(25.0, 10.0), Point::new(25.0, 40.0)),
                    self.color(data, self.tool),
                    3.0
                );
                ctx.stroke(
                    Line::new(Point::new(17.5, 40.0), Point::new(32.5, 40.0)),
                    self.color(data, self.tool),
                    3.0
                );
            }
            Tool::LINE => {
                ctx.stroke(
                    Line::new(Point::new(35.0, 10.0), Point::new(15.0, 40.0)),
                    self.color(data, self.tool),
                    3.0
                );
            }
            Tool::PATH => {
                ctx.stroke(
                    Line::new(Point::new(10.0, 40.0), Point::new(20.0, 15.0)),
                    self.color(data, self.tool),
                    3.0
                );
                ctx.stroke(
                    Line::new(Point::new(20.0, 15.0), Point::new(30.0, 35.0)),
                    self.color(data, self.tool),
                    3.0
                );
                ctx.stroke(
                    Line::new(Point::new(30.0, 35.0), Point::new(40.0, 10.0)),
                    self.color(data, self.tool),
                    3.0
                );
            }
        }
    }
}

impl ToolButtonWidget {
    fn color(&self, data: &Palette, me: Tool) -> &'static Color {
        if data.selected_tool == me {
            &Color::YELLOW
        } else {
            &Color::GREEN
        }
    }
}
