use std::f32::consts::{FRAC_PI_2, TAU};

use eframe::egui::{Pos2, Vec2};
use typed_floats::tf32::PositiveFinite;

const RAD_RAT_DENOM_DELTA: f32 = 0.1;

#[derive(Debug, Default)]
pub struct SpiralProjector {}

impl SpiralProjector {
    pub fn project(&self, f: PositiveFinite) -> Pos2 {
        self.project_prim(f.get())
    }

    pub fn project_with_norm(&self, f: PositiveFinite) -> (Pos2, Vec2) {
        let f = f.get();
        let pt = self.project_prim(f);
        let pt2 = self.project_prim(f + 0.0001);
        let tangent = pt2 - pt;
        (pt, tangent.rot90())
    }

    fn project_prim(&self, f: f32) -> Pos2 {
        let angle = f * TAU - FRAC_PI_2;
        let radius = f / (f + RAD_RAT_DENOM_DELTA);

        Pos2::new(radius * angle.cos(), radius * angle.sin())
    }
}
