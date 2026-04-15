use color_eyre::eyre;
use eframe::egui::{Color32, Response, Sense, Ui};
use jiff::{ToSpan as _, Zoned};
use typed_floats::tf32::PositiveFinite;

use crate::{Interval, SpiralProjector, Ticks, TryWidget, UnitCircleProjector};

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

        let ptcnt = 1000;
        let ptcntf = ptcnt as f32;

        let mut pts = vec![];
        for i in 0..ptcnt {
            let f = PositiveFinite::new((i as f32) / ptcntf)?;
            let pt = self.sproj.project(f);
            let pt = ucp.project(pt);
            pts.push(pt);
        }

        painter.line(pts, (spiral_stroke_width, Color32::from_gray(90)));

        let maxspan = 24.hours();
        let interval = Interval::new(self.now.clone(), maxspan)?;
        for t in Ticks::new(self.now) {
            let f = interval.progress(&t)?;
            let f = PositiveFinite::new(f.get())?;
            let (pt, norm) = self.sproj.project_with_norm(f);
            let a = pt + norm;
            let b = pt - norm;
            dbg!(pt, norm, a, b);
            let a = ucp.project(a);
            let b = ucp.project(b);

            painter.line_segment([a, b], (spiral_stroke_width, Color32::BLUE));
        }

        Ok(ui.interact(rect, ui.id(), Sense::hover()))
    }
}
