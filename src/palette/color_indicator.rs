use druid::widget::prelude::*;
use druid::{Color, Rect};
use crate::*;

#[derive(Clone,Debug)]
pub struct ColorIndicatorWidget {
    color: StyleColor,
}

impl ColorIndicatorWidget {
    pub fn new(color: StyleColor) -> Self {
        ColorIndicatorWidget { color }
    }
}
impl Widget<Color> for ColorIndicatorWidget {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        _data: &mut Color,
        _env: &Env) {
        
    }
    
    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Color,
        _env: &Env
    ) {
        
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &Color,
        _data: &Color,
        _env: &Env
    ) {

    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &Color,
        _env: &Env,
    ) -> Size {
        Size::new(50.0, 50.0)
    }

    fn paint(
        &mut self,
        ctx: &mut PaintCtx,
        data: &Color,
        _env: &Env
    ) {
        let b = ctx.region().bounding_box();
        ctx.fill(Rect::new(3.0, 3.0, 47.0, 47.0), data);
    }
}
