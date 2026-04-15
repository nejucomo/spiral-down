use jiff::{RoundMode, Span, ToSpan as _, Unit, Zoned, ZonedRound};
use TickInterval::*;

#[derive(Copy, Clone, Debug)]
enum TickInterval {
    Second,
    Minute,
    QuarterHour,
    Hour,
    EighthDay,
    QuarterDay,
    HalfDay,
    Day,
    // Rounding is broken:
    // Week,
    // Month,
    // Year,
}

const TICK_INTERVALS: [TickInterval; 8] = [
    Second,
    Minute,
    QuarterHour,
    Hour,
    EighthDay,
    QuarterDay,
    HalfDay,
    Day,
    // Rounding is broken:
    // Week,
    // Month,
    // Year,
];

pub struct Ticks {
    idx: usize,
    zoneds: [Zoned; 8],
}

impl Ticks {
    pub fn new(now: Zoned) -> Self {
        Self {
            idx: 0,
            zoneds: TICK_INTERVALS.map(|ti| now.round(ti.zoned_round()).unwrap()),
        }
    }
}

impl Iterator for Ticks {
    type Item = Zoned;

    fn next(&mut self) -> Option<Self::Item> {
        let zlen = self.zoneds.len();
        if self.idx < zlen {
            let t = self.zoneds[self.idx].clone();

            if self.idx + 1 == zlen || t >= self.zoneds[self.idx + 1] {
                self.idx = zlen.min(self.idx + 1);
            }

            if self.idx < zlen {
                let nextt = &mut self.zoneds[self.idx];
                *nextt = nextt.checked_add(TICK_INTERVALS[self.idx].span()).unwrap();
            }

            Some(t)
        } else {
            None
        }
    }
}

impl TickInterval {
    fn zoned_round(self) -> ZonedRound {
        let zrbase = ZonedRound::new().mode(RoundMode::Ceil);
        match self {
            Second => zrbase.smallest(Unit::Second),
            Minute => zrbase.smallest(Unit::Minute),
            QuarterHour => zrbase.smallest(Unit::Minute).increment(15),
            Hour => zrbase.smallest(Unit::Hour),
            EighthDay => zrbase.smallest(Unit::Hour).increment(3),
            QuarterDay => zrbase.smallest(Unit::Hour).increment(6),
            HalfDay => zrbase.smallest(Unit::Hour).increment(12),
            Day => zrbase.smallest(Unit::Day),
            // This rounding is broken:
            // Week => zrbase.smallest(Unit::Week),
            // Month => zrbase.smallest(Unit::Month),
            // Year => zrbase.smallest(Unit::Year),
        }
    }

    fn span(self) -> Span {
        match self {
            Second => 1.second(),
            Minute => 1.minute(),
            QuarterHour => 15.minute(),
            Hour => 1.hour(),
            EighthDay => 3.hour(),
            QuarterDay => 6.hour(),
            HalfDay => 12.hour(),
            Day => 1.day(),
            // Rounding is broken:
            // Week => 1.week(),
            // Month => 1.month(),
            // Year => 1.year(),
        }
    }
}
