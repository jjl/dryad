use druid::widget::prelude::*;
use druid::{Data, Rect, WidgetPod};
use crate::palette::*;
use crate::drawing::*;
use crate::*;

#[derive(Clone, Default, Debug)]
pub struct DrawingWindow {
    pub style: StyleManual,
    pub palette: Palette,
    pub drawing: Drawing,
}

impl Data for DrawingWindow {
    fn same(&self, other: &Self) -> bool {
        self.style == other.style && self.palette == other.palette
    }
}

pub struct DrawingWindowWidget {
    palette: WidgetPod<DrawingWindow, PaletteWidget>,
    drawing: WidgetPod<DrawingWindow, DrawingWidget>,
}

impl Default for DrawingWindowWidget {
    fn default() -> DrawingWindowWidget {
        DrawingWindowWidget {
            palette: WidgetPod::new(PaletteWidget::default()),
            drawing: WidgetPod::new(DrawingWidget::default()),
        }
    }
}

impl Widget<DrawingWindow> for DrawingWindowWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DrawingWindow, env: &Env) {
        self.palette.event(ctx, event, data, env);
        self.drawing.event(ctx, event, data, env);
        // match event {
        //     Event::MouseDown(_) => {
        //         self.t = 0.0;
        //         ctx.request_anim_frame();
        //     }
        //     Event::AnimFrame(interval) => {
        //         ctx.request_paint();
        //         self.t += (*interval as f64) * 1e-9;
        //         if self.t < 1.0 {
        //             ctx.request_anim_frame();
        //         } else {
        //             // We might have t>1.0 at the end of the animation,
        //             // we want to make sure the line points up at the
        //             // end of the animation.
        //             self.t = 0.0;
        //         }
        //     }
        //     _ => (),
        // }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &DrawingWindow, env: &Env) {
        self.palette.lifecycle(ctx, event, data, env);
        self.drawing.lifecycle(ctx, event, data, env);

    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &DrawingWindow, data: &DrawingWindow, env: &Env) {
        self.palette.update(ctx, data, env);
        self.drawing.update(ctx, data, env);
        ctx.request_paint();
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &DrawingWindow,
        env: &Env,
    ) -> Size {
        let width = bc.max().width;
        let mut available = bc.max().height;
        let palette_size = self.palette.layout(ctx, bc, data, env);
        available -= palette_size.height;
        let bc2 = BoxConstraints::new(Size::new(width, 0.0), Size::new(width, available));
        let drawing_size = self.drawing.layout(ctx, &bc2, data, env);
        let palette_rect = Rect::from_origin_size((0.0, 0.0), palette_size);
        let drawing_rect = Rect::from_origin_size((0.0, palette_size.height), drawing_size);
        self.palette.set_layout_rect(ctx, data, env, palette_rect);
        self.drawing.set_layout_rect(ctx, data, env, drawing_rect);
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &DrawingWindow, env: &Env) {
        self.palette.paint(ctx, data, env);
        self.drawing.paint(ctx, data, env);
    }
}
