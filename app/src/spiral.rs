use color_eyre::eyre;
use eframe::egui::{Color32, Response, Sense, Ui};
use jiff::{ToSpan as _, Zoned};
use typed_floats::tf32::PositiveFinite;

use crate::{Interval, SpiralProjector, Ticks, TryWidget, UnitCircleProjector};

const TIME_WARP_POWER: f32 = 0.5;
const SPIRAL_POINT_COUNT: usize = 1000;
const SPIRAL_POINT_COUNT_F: f32 = SPIRAL_POINT_COUNT as f32;
const SPINE_COLOR: Color32 = Color32::from_gray(90);
const TICK_COLOR: Color32 = Color32::from_gray(120);

#[derive(Debug)]
pub struct SpiralWidget {
    now: Zoned,
    sproj: SpiralProjector,
}

impl Default for SpiralWidget {
    fn default() -> Self {
        Self {
            now: Zoned::now(),
            sproj: SpiralProjector::default(),
        }
    }
}

impl TryWidget for SpiralWidget {
    fn try_ui(self, ui: &mut Ui) -> eyre::Result<Response> {
        let rect = ui.max_rect();
        let painter = ui.painter();

        let ucp = UnitCircleProjector::new(ui.max_rect())?;
        let spiral_stroke_width = 0.5;

        let mut pts = vec![];
        for i in 0..SPIRAL_POINT_COUNT {
            let f = PositiveFinite::new((i as f32) / SPIRAL_POINT_COUNT_F)?;
            let pt = self.sproj.project(f);
            let pt = ucp.project(pt);
            pts.push(pt);
        }

        painter.line(pts, (spiral_stroke_width, SPINE_COLOR));

        let maxspan = 24.hours();
        let interval = Interval::new(self.now.clone(), maxspan)?;
        for t in Ticks::new(self.now) {
            let f = interval.progress(&t)?;
            let f = PositiveFinite::new(f.get().powf(TIME_WARP_POWER))?;
            let (pt, norm) = self.sproj.project_with_norm(f);
            let a = pt + norm;
            let b = pt - norm;
            let a = ucp.project(a);
            let b = ucp.project(b);

            painter.line_segment([a, b], (spiral_stroke_width, TICK_COLOR));
        }

        Ok(ui.interact(rect, ui.id(), Sense::hover()))
    }
}
