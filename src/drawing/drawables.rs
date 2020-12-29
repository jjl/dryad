use druid::kurbo::{Circle, Ellipse, Rect},
use druid::{Color, LinearGradient, RadialGradient, UnitPoint};
use druid::widget::prelude::*;

pub enum Action {
    Insert(Shape),
    Resize(Shape, Geom),
}

pub enum Paint {
    Flat(Color),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
}

pub struct LineStyle {
    paint: Paint,
    width: f64,
}

pub struct ShapeStyle {
    line: LineStyle,
    fill: Paint,
}

pub enum Geom {
    Rect(Rect),
    Square(Rect),
    Circle(Circle),
    Ellipse(Ellipse),
}

pub struct Shape {
    style: ShapeStyle,
    geom: Geom,
}

pub struct ShapeWidget {
    interaction: Option<Interaction>,
}

pub enum Interaction {
    Resize(UnitPoint, Point),
}

impl Widget<Shape> for ShapeWidget {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        _data: &mut Shape,
        _env: &Env) {
        
    }
    
    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &Shape,
        _env: &Env
    ) {
        
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &Shape,
        _data: &Shape,
        _env: &Env
    ) {

    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &Shape,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(
        &mut self,
        ctx: &mut PaintCtx,
        _data: &Shape,
        _env: &Env
    ) {
        
    }
}
