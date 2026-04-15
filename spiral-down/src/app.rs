use color_eyre::eyre::{eyre, Result};
use eframe::egui::{Color32, Shape, Stroke, Ui, ViewportBuilder, ViewportCommand};
use eframe::epaint::CircleShape;
use jiff::{ToSpan as _, Zoned};

use crate::{SpiralProjection as _, Ticks};

const DELTA_T_GROWTH_FACTOR: f32 = 1.01;

pub struct SpiralApp {}

impl SpiralApp {
    pub fn run<I: IntoIterator<Item = (Zoned, String)>>(events: I) -> Result<()> {
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

    fn new<I: IntoIterator<Item = (Zoned, String)>>(events: I) -> Self {
        let events: Vec<_> = events.into_iter().collect::<Vec<_>>();
        if events.is_empty() {
            Self {}
        } else {
            todo!("add custom event support: {events:#?}")
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
            .request_repaint_after(std::time::Duration::from_millis(50));

        let rect = ui.max_rect();
        let center = rect.center();
        let maxradius = rect.width().min(rect.height()) / 2.0 * 0.98;
        let painter = ui.painter();

        let now = Zoned::now();

        // Paint underlying spiral:
        {
            let stop = now.checked_add(2.day()).unwrap();

            let mut deltasec: f32 = 1.0;
            let mut t = now.clone();
            let mut pts = vec![];

            while t < stop {
                pts.push((&now, &t).into_spiral_pt_scaled(center, maxradius));

                t += (deltasec as i64).seconds();
                deltasec *= DELTA_T_GROWTH_FACTOR;
            }

            painter.add(Shape::line(pts, Stroke::new(1.0, Color32::from_gray(90))));
        }

        // Paint ticks
        {
            let tickradius = maxradius / 100.0;
            for t in Ticks::new(now.clone()) {
                painter.add(CircleShape::stroke(
                    (&now, &t).into_spiral_pt_scaled(center, maxradius),
                    tickradius,
                    (1.0, Color32::BLUE),
                ));
            }
        }
    }
}
