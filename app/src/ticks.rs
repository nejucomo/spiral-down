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
pub enum TickInterval {
    Second,
    HalfMinute,
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
    HalfMinute,
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
        self.pop_and_reinsert()?
            .map(|mut tick| {
                // Deduplicate ticks at the same time:
                while self.peek_time() == Some(&tick.t) {
                    let nexttick = self.pop_and_reinsert()?.unwrap();
                    if nexttick.ti > tick.ti {
                        tick = nexttick;
                    }
                }
                Ok(tick)
            })
            .transpose()
    }

    fn pop_and_reinsert(&mut self) -> eyre::Result<Option<Tick>> {
        if let Some(tick) = self.heap.pop() {
            if let Some(next) = tick.next()? {
                self.heap.push(next);
            }
            Ok(Some(tick))
        } else {
            Ok(None)
        }
    }

    // Used for deduplicating same-timed ticks:
    fn peek_time(&self) -> Option<&Zoned> {
        self.heap.peek().map(|tick| &tick.t)
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

    pub fn interval(&self) -> TickInterval {
        self.ti
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn new(now: &Zoned, ti: TickInterval) -> eyre::Result<Self> {
        let t = now.round(ti.zoned_round())?;
        let prior = 0;
        let label = Some(ti.label_for(&t, prior));

        Ok(Self {
            t,
            prior,
            ti,
            label,
        })
    }

    fn next(&self) -> eyre::Result<Option<Self>> {
        let ti = self.ti;
        let prior = self.prior + 1;
        if prior < ti.count() {
            let t = self.t.checked_add(self.ti.span())?;
            let label = if prior < ti.label_count() {
                Some(ti.label_for(&t, prior))
            } else {
                None
            };

            Ok(Some(Self {
                t,
                prior,
                ti,
                label,
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
            HalfMinute => zrbase.smallest(Unit::Second).increment(30),
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
            HalfMinute => 30.seconds(),
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
            Second => 10,
            HalfMinute => 4,
            Minute => 120,
            QuarterHour => 8,
            Hour => 24,
            EighthDay => 8,
            QuarterDay => 4,
            HalfDay => 2,
            Day => 30,
        }
    }

    fn label_for(self, t: &Zoned, _prior: usize) -> String {
        match self {
            Second => t.strftime(":%S").to_string(),
            HalfMinute => t.time().to_string(),
            Minute | QuarterHour => t.strftime("%H:%M").to_string(),
            Hour | EighthDay | QuarterDay | HalfDay => t.strftime("%H").to_string(),
            Day => t.date().to_string(),
        }
    }

    fn label_count(self) -> usize {
        match self {
            Second => 30,
            HalfMinute => 3,
            Minute => 10,
            QuarterHour => 3,
            Hour => 2,
            EighthDay => 1,
            QuarterDay => 1,
            HalfDay => 1,
            Day => 31, // should be one month
        }
    }
}
