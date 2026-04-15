use color_eyre::eyre::{eyre, Result};
use eframe::egui::{Color32, Shape, Stroke, Ui, ViewportBuilder, ViewportCommand};
use eframe::epaint::CircleShape;
use jiff::{Timestamp, ToSpan as _};

use crate::{Event, Schedule, SpiralProjection as _};

const DELTA_T_GROWTH_FACTOR: f32 = 1.01;

pub struct SpiralApp {
    sched: Schedule,
}

impl SpiralApp {
    pub fn run<I: IntoIterator<Item = (Timestamp, Event)>>(events: I) -> Result<()> {
        eframe::run_native(
            env!("CARGO_PKG_NAME"),
            eframe::NativeOptions {
                viewport: ViewportBuilder::default().with_maximized(true),
                persist_window: false,
                ..Default::default()
            },
            Box::new(|_cc| Ok(Box::new(Self::new(events)))),
        )
        .map_err(|e| eyre!("eframe error: {e}"))
    }

    fn new<I: IntoIterator<Item = (Timestamp, Event)>>(events: I) -> Self {
        Self {
            sched: Schedule::new(events),
        }
    }
}

impl eframe::App for SpiralApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        if !ui.input(|i| i.keys_down.is_empty()) {
            ui.ctx().send_viewport_cmd(ViewportCommand::Close);
        }

        // Repaint frequently enough for a smooth second-tick countdown.
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(250));

        let rect = ui.max_rect();
        let center = rect.center();
        let maxradius = rect.width().min(rect.height()) / 2.0 * 0.98;
        let evradius = maxradius / 100.0;
        let painter = ui.painter();

        let now = Timestamp::now();

        // Paint underlying spiral:
        {
            let stop = now + 1.year();

            let mut deltasec: f32 = 1.0;
            let mut t = now;
            let mut pts = vec![];

            while t < stop {
                pts.push(t.into_spiral_pt_scaled(center, maxradius));

                t += (deltasec as i64).seconds();
                deltasec *= DELTA_T_GROWTH_FACTOR;
            }

            painter.add(Shape::line(pts, Stroke::new(1.0, Color32::from_gray(90))));
        }

        // Paint the events:
        {
            self.sched.drop_past_events(now);
            for (t, _ev) in self.sched.iter() {
                painter.add(CircleShape::stroke(
                    t.into_spiral_pt_scaled(center, maxradius),
                    evradius,
                    (1.0, Color32::BLUE),
                ));
            }
        }
    }
}
