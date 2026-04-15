use std::f32::consts::{FRAC_PI_2, TAU};

use eframe::egui::{lerp, Pos2};
use jiff::{Timestamp, ToSpan as _};

const RAD_RAT_DENOM_DELTA: f32 = 0.1;

pub trait SpiralProjection: Sized + Copy {
    fn into_spiral_pt_scaled(self, center: Pos2, maxradius: f32) -> Pos2 {
        let Pos2 { x, y } = self.into_spiral_pt();
        Pos2 {
            x: lerp(0f32..=maxradius, x) + center.x,
            y: lerp(0f32..=maxradius, y) + center.y,
        }
    }

    fn into_spiral_pt(self) -> Pos2 {
        f32_into_spiral_pt(self.into_norm_f32())
    }

    fn into_norm_f32(self) -> f32;
}

impl SpiralProjection for Timestamp {
    fn into_norm_f32(self) -> f32 {
        let now = Timestamp::now();
        let max = seconds_from_now(now, now + 1.year());
        let cur = seconds_from_now(now, self);
        cur / max
    }
}

impl SpiralProjection for f32 {
    fn into_norm_f32(self) -> f32 {
        self
    }
}

fn seconds_from_now(now: Timestamp, t: Timestamp) -> f32 {
    assert!(t >= now, "{:#?}", (now, t));
    (t - now).total(jiff::Unit::Second).unwrap() as f32
}

fn f32_into_spiral_pt(f: f32) -> Pos2 {
    let angle = f * TAU - FRAC_PI_2;
    let radius = f / (f + RAD_RAT_DENOM_DELTA);

    Pos2::new(radius * angle.cos(), radius * angle.sin())
}
