use color_eyre::eyre;
use jiff::Unit::Second;
use jiff::{Span, Zoned};
use typed_floats::NonNaNFinite;

#[derive(Debug)]
pub struct Interval {
    t: Zoned,
    span_seconds: f64,
}

impl Interval {
    pub fn new(t: Zoned, span: Span) -> eyre::Result<Self> {
        let span_seconds = span.total(Second)?;
        Ok(Interval { t, span_seconds })
    }

    pub fn progress(&self, t: &Zoned) -> eyre::Result<NonNaNFinite<f32>> {
        let n = (t - &self.t).total(Second)?;
        let rat64 = n / self.span_seconds;
        let rat32 = NonNaNFinite::<f32>::new(rat64 as f32)?;
        Ok(rat32)
    }
}
