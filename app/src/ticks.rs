use std::collections::BinaryHeap;

use color_eyre::eyre;
use jiff::{RoundMode, Span, ToSpan as _, Unit, Zoned, ZonedRound};
use TickInterval::*;

pub struct Ticks {
    heap: BinaryHeap<Tick>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Tick {
    t: Zoned,
    prior: usize,
    ti: TickInterval,
    label: Option<String>,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum TickInterval {
    Second,
    QuarterMinute,
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

const TICK_INTERVALS: [TickInterval; 9] = [
    Second,
    QuarterMinute,
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

impl Ticks {
    pub fn new(now: Zoned) -> eyre::Result<Self> {
        Ok(Self {
            heap: TICK_INTERVALS
                .into_iter()
                .map(|ti| Tick::new(&now, ti).unwrap())
                .collect(),
        })
    }

    fn next_res(&mut self) -> eyre::Result<Option<Tick>> {
        self.heap
            .pop()
            .map(|tick| {
                if let Some(next) = tick.next()? {
                    self.heap.push(next);
                }
                Ok(tick)
            })
            .transpose()
    }
}

impl Iterator for Ticks {
    type Item = Tick;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_res().unwrap()
    }
}

impl Tick {
    pub fn time(&self) -> &Zoned {
        &self.t
    }

    pub fn prior(&self) -> usize {
        self.prior
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn new(now: &Zoned, ti: TickInterval) -> eyre::Result<Self> {
        let t = now.round(ti.zoned_round())?;
        let label = Some(ti.label_for(&t));
        Ok(Self {
            t,
            prior: 0,
            ti,
            label,
        })
    }

    fn next(&self) -> eyre::Result<Option<Self>> {
        let ti = self.ti;
        let prior = self.prior + 1;
        if prior < ti.count() {
            let t = self.t.checked_add(self.ti.span())?;
            Ok(Some(Self {
                t,
                prior,
                ti,
                label: None,
            }))
        } else {
            Ok(None)
        }
    }
}

impl Ord for Tick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.t, self.prior)
            .cmp(&(&other.t, other.prior))
            .reverse()
    }
}

impl PartialOrd for Tick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TickInterval {
    fn zoned_round(self) -> ZonedRound {
        let zrbase = ZonedRound::new().mode(RoundMode::Ceil);
        match self {
            Second => zrbase.smallest(Unit::Second),
            QuarterMinute => zrbase.smallest(Unit::Second).increment(15),
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
            QuarterMinute => 15.seconds(),
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

    // How many ticks on the spiral for this interval?
    fn count(self) -> usize {
        match self {
            Second => 30,
            QuarterMinute => 8,
            Minute => 120,
            QuarterHour => 8,
            Hour => 24,
            EighthDay => 8,
            QuarterDay => 4,
            HalfDay => 2,
            Day => 30,
        }
    }

    fn label_for(self, t: &Zoned) -> String {
        match self {
            Day => t.date().to_string(),
            _ => t.time().to_string(),
        }
    }
}
