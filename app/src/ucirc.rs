use color_eyre::eyre;
use eframe::egui::{Pos2, Rect};
use typed_floats::tf32::{PositiveFinite, StrictlyPositiveFinite};

const RADIUS_DIVISOR: StrictlyPositiveFinite =
    unsafe { StrictlyPositiveFinite::new_unchecked(2.0 * 1.05) };

#[derive(Debug)]
pub struct UnitCircleProjector {
    center: Pos2,
    radius: PositiveFinite,
}

impl UnitCircleProjector {
    pub fn new(rect: Rect) -> eyre::Result<Self> {
        let center = rect.center();
        let mindim = PositiveFinite::new(rect.width().min(rect.height()))?;
        let radius = unsafe { PositiveFinite::new_unchecked((mindim / RADIUS_DIVISOR).get()) };
        Ok(Self { center, radius })
    }

    pub fn project(&self, unitpt: Pos2) -> Pos2 {
        let r = self.radius.get();
        Pos2 {
            x: unitpt.x * r + self.center.x,
            y: unitpt.y * r + self.center.y,
        }
    }
}
