use std::collections::BTreeMap;

use jiff::Timestamp;

use crate::Event;

#[derive(Debug, Default)]
pub struct Schedule(BTreeMap<Timestamp, Vec<Event>>);

impl Schedule {
    pub fn new<I: IntoIterator<Item = (Timestamp, Event)>>(events: I) -> Self {
        let mut sched = Self(BTreeMap::default());

        for (t, ev) in Event::standard_intervals().chain(events) {
            sched.insert(t, ev);
        }

        sched
    }

    pub fn insert(&mut self, t: Timestamp, event: Event) {
        let events: &mut Vec<Event> = self.0.entry(t).or_default();
        events.push(event);
    }

    /// Drop all events prior to `cutoff`
    pub fn drop_past_events(&mut self, cutoff: Timestamp) {
        while let Some((t, evs)) = self.0.pop_first() {
            if cutoff <= t {
                // Put it back; we're done:
                self.0.insert(t, evs);
                return;
            } else {
                // These are dropping off, so let's iterate on IntervalMarkers:
                for ev in evs {
                    if let Some((t, ev)) = ev.next_iteration_from(cutoff) {
                        self.insert(t, ev);
                    }
                }
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Timestamp, &Event)> {
        self.0
            .iter()
            .flat_map(|(t, evs)| evs.iter().map(|ev| (*t, ev)))
    }
}
