use crate::*;
use crate::drawing_window::*;
use crate::palette::*;
use druid::kurbo::{Circle, Line, Rect};
use druid::widget::prelude::*;
use druid::{Color, Cursor, Data, MouseButton, Point};
use std::cmp;

pub mod commands {
    use druid::Selector;
    pub const INSERT: Selector = Selector::new("dryad.drawing.insert");
}

pub enum Geometry {
    Rectangle,
    // Square,
    // Oval,
    // Circle,
    // Line,
    // Path,
}

pub enum InsertCommand {
    Geometry(Geometry),
    Text,
}

#[derive(Clone, Default, Debug)]
pub struct Drawing {
    
}

impl Data for Drawing {
    fn same(&self, _other: &Self) -> bool { true }
}

pub struct DrawingState {
    
}

#[derive(Clone, Copy, Debug)]
enum Work {
    Panning(Point),
    DrawingRect(Rect),
    DrawingCircle(Circle),
    DrawingLine(Line),
    EditingText,
}

#[derive(Default)]
pub struct DrawingWidget {
    offset: Size,
    work: Option<Work>,
}

impl Widget<DrawingWindow> for DrawingWidget {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut DrawingWindow,
        _env: &Env
    ) {
        match event {
            // Event::Command(command) => {
            // }
            Event::MouseDown(e) => {
                match e.button {
                    MouseButton::Left => {
                        match data.palette.selected_tool {
                            Tool::ARROW => {
                                self.work = Some(Work::Panning(e.pos));
                                ctx.set_cursor(&Cursor::OpenHand);
                            }
                            Tool::RECTANGLE => {
                                let rect = Rect::from_points(e.pos, e.pos);
                                self.work = Some(Work::DrawingRect(rect));
                                ctx.set_cursor(&Cursor::Crosshair);
                            }
                            Tool::CIRCLE => {
                                let circle = Circle::new(e.pos, 0.0);
                                self.work = Some(Work::DrawingCircle(circle));
                                ctx.set_cursor(&Cursor::Crosshair);
                            }
                            Tool::TEXT => {
                                self.work = Some(Work::EditingText);
                                ctx.set_cursor(&Cursor::Crosshair);
                            }
                            Tool::LINE => {
                                let line = Line::new(e.pos, e.pos);
                                self.work = Some(Work::DrawingLine(line));
                                ctx.set_cursor(&Cursor::Crosshair);
                            }
                            Tool::PATH => {
                                let line = Line::new(e.pos, e.pos);
                                self.work = Some(Work::DrawingLine(line));
                                ctx.set_cursor(&Cursor::Crosshair);
                            }
                        }
                        ctx.set_active(true);
                        ctx.set_handled();
                    }
                    MouseButton::Middle => {
                        self.work = Some(Work::Panning(e.pos));
                        ctx.set_active(true);
                        ctx.set_handled();
                    }
                    _ => {
                    }
                }
            }
            Event::MouseUp(e) => {
                if let Some(work) = &mut self.work {
                    self.work.take();
                    ctx.resign_focus();
                    ctx.set_active(false);
                    ctx.set_cursor(&Cursor::Arrow);
                    ctx.set_handled();
                }
            }
            Event::MouseMove(e) => {
                if let Some(work) = &mut self.work {
                    match work {
                        Work::Panning(point) => {
                            println!("dx: {}, dy: {}", e.pos.x - point.x, e.pos.y - point.y);
                        }
                        Work::DrawingRect(rect) => {
                            *rect = Rect::new(rect.x0, rect.y0, e.pos.x, e.pos.y);
                            ctx.request_paint();
                        }
                        Work::DrawingCircle(circle) => {
                            let c = circle.center;
                            let p = e.pos;
                            let x = (c.x - p.x).abs();
                            let y = (c.y - p.y).abs();
                            let radius = if x > y { x } else { y };
                            *circle = Circle::new(circle.center, radius);
                            ctx.request_paint();
                        }
                        Work::DrawingLine(line) => {
                            *line = Line::new(line.p0, e.pos);
                            ctx.request_paint();
                        }
                        _ => ()
                    }
                }
            }
           // Event::AnimFrame(interval) => {
            //     ctx.request_paint();
            //     self.t += (*interval as f64) * 1e-9;
            //     if self.t < 1.0 {
            //         ctx.request_anim_frame();
            //     } else {
            //         // We might have t>1.0 at the end of the animation,
            //         // we want to make sure the line points up at the
            //         // end of the animation.
            //         self.t = 0.0;
            //     }
            // }
            _ => (),
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &DrawingWindow, _env: &Env) {
        
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &DrawingWindow, _data: &DrawingWindow, _env: &Env) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &DrawingWindow,
        _env: &Env,
    ) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &DrawingWindow, _env: &Env) {
        let b = ctx.region().bounding_box();
        ctx.fill(b, &Color::WHITE);
        if let Some(work) = &self.work {
            match work {
                Work::DrawingRect(rect) => {
                    ctx.fill(rect, &data.style.colors[StyleColor::FILL as usize]);
                }
                Work::DrawingCircle(circle) => {
                    ctx.fill(circle, &data.style.colors[StyleColor::FILL as usize]);
                }
                Work::DrawingLine(line) => {
                    ctx.stroke(line, &data.style.colors[StyleColor::FILL as usize], 3.0);
                }
                _ => {}
            }
        }
    }
}
