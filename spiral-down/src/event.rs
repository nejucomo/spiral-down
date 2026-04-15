use jiff::{Span, Timestamp};

#[derive(Clone, Debug)]
pub enum Event {
    User(String),
    Ivm(IntervalMarker),
}

#[derive(Copy, Clone, Debug)]
pub enum IntervalMarker {
    Minute,
    QuarterHour,
    Hour,
    EighthDay,
    QuarterDay,
    HalfDay,
    Day,
    Week,
    Month,
    Year,
}

impl Event {
    pub fn standard_intervals() -> impl Iterator<Item = (Timestamp, Event)> {
        let now = Timestamp::now();
        IntervalMarker::enumerate().map(move |ivm| (now + ivm.span(), Event::Ivm(ivm)))
    }

    pub fn next_iteration_from(self, now: Timestamp) -> Option<(Timestamp, Self)> {
        use Event::Ivm;

        match self {
            Ivm(ivm) => Some((now + ivm.span(), Ivm(ivm))),
            _ => None,
        }
    }
}

impl IntervalMarker {
    fn enumerate() -> impl Iterator<Item = Self> {
        use IntervalMarker::*;

        [
            Minute,
            QuarterHour,
            Hour,
            EighthDay,
            QuarterDay,
            HalfDay,
            Day,
            Week,
            Month,
            Year,
        ]
        .into_iter()
    }

    fn span(self) -> Span {
        use jiff::ToSpan;
        use IntervalMarker::*;

        match self {
            Minute => 1.minute(),
            QuarterHour => 15.minutes(),
            Hour => 1.hour(),
            EighthDay => 3.hours(),
            QuarterDay => 6.hours(),
            HalfDay => 12.hours(),
            Day => 1.day(),
            Week => 1.week(),
            Month => 1.month(),
            Year => 1.year(),
        }
    }
}
