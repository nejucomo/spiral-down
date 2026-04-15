use color_eyre::eyre::{eyre, Result};
use eframe::egui::{Color32, Shape, Stroke, Ui, ViewportBuilder, ViewportCommand};
use jiff::Zoned;
use typed_floats::tf32::PositiveFinite;

use crate::{SpiralProjector, UnitCircleProjector};

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

        let spiral = SpiralProjector::default();
        let ucp = UnitCircleProjector::new(ui.max_rect()).unwrap();
        let painter = ui.painter();

        // let now = Zoned::now();

        // Paint underlying spiral:
        {
            let ptcnt = 1000;
            let ptcntf = ptcnt as f32;

            let mut pts = vec![];
            for i in 0..ptcnt {
                let f = PositiveFinite::new((i as f32) / ptcntf).unwrap();
                let pt = spiral.project(f);
                let pt = ucp.project(pt);
                pts.push(pt);
            }

            painter.add(Shape::line(pts, Stroke::new(0.5, Color32::from_gray(90))));
        }

        // Paint ticks
        /*
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
        */
    }
}
