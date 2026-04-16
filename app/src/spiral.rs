use color_eyre::eyre;
use eframe::egui::{Color32, Response, Sense, Ui};
use jiff::{ToSpan as _, Zoned};
use typed_floats::tf32::PositiveFinite;

use crate::{Interval, SpiralProjector, Ticks, TryWidget, UnitCircleProjector};

const SPIRAL_HOURS_PER_ROTATION: i32 = 24;
const SPIRAL_ROTATIONS: i32 = 30;
const TIME_WARP_POWER: f32 = 0.5;
const SPIRAL_POINTS_PER_LEVEL: usize = 800;
const SPIRAL_POINT_LEVELS: usize = 16;
const SPIRAL_POINTS_TOTAL_F: f32 = (SPIRAL_POINTS_PER_LEVEL * SPIRAL_POINT_LEVELS) as f32;
const SPIRAL_LEVEL_GAMMA_FACTOR: f32 = 0.6;
const SPINE_COLOR: Color32 = Color32::from_gray(90);
const TICK_COLOR: Color32 = Color32::from_gray(120);
const TICK_ATTENUATION_FACTOR: f32 = 0.9;

#[derive(Debug)]
pub struct SpiralWidget {
    now: Zoned,
    sproj: SpiralProjector,
}

impl Default for SpiralWidget {
    fn default() -> Self {
        Self {
            now: Zoned::now(),
            sproj: SpiralProjector::new(SPIRAL_ROTATIONS as f32),
        }
    }
}

impl TryWidget for SpiralWidget {
    fn try_ui(self, ui: &mut Ui) -> eyre::Result<Response> {
        let rect = ui.max_rect();
        let painter = ui.painter();

        let ucp = UnitCircleProjector::new(ui.max_rect())?;
        let spiral_stroke_width = 0.5;

        for level in 0..SPIRAL_POINT_LEVELS {
            let mut pts = vec![];

            for lvli in 0..SPIRAL_POINTS_PER_LEVEL {
                let i = level * SPIRAL_POINTS_PER_LEVEL + lvli;
                let f = PositiveFinite::new((i as f32) / SPIRAL_POINTS_TOTAL_F)?;
                let pt = self.sproj.project(f);
                let pt = ucp.project(pt);
                pts.push(pt);
            }

            painter.line(
                pts,
                (
                    spiral_stroke_width,
                    SPINE_COLOR.gamma_multiply(SPIRAL_LEVEL_GAMMA_FACTOR.powi(level as i32)),
                ),
            );
        }

        let interval = Interval::new(
            self.now.clone(),
            (SPIRAL_ROTATIONS * SPIRAL_HOURS_PER_ROTATION).hours(),
        )?;
        for tick in Ticks::new(self.now)? {
            let f = interval.progress(tick.time())?;
            let f = PositiveFinite::new(f.get().powf(TIME_WARP_POWER))?;
            let (pt, norm) = self.sproj.project_with_norm(f);
            let norm = norm * TICK_ATTENUATION_FACTOR.powi(tick.prior().try_into().unwrap());
            let a = pt + norm;
            let b = pt - norm;
            let a = ucp.project(a);
            let b = ucp.project(b);

            painter.line_segment([a, b], (spiral_stroke_width, TICK_COLOR));
        }

        Ok(ui.interact(rect, ui.id(), Sense::hover()))
    }
}
