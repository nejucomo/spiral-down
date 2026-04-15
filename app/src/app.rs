use color_eyre::eyre::{self, eyre};
use eframe::egui::{Ui, ViewportBuilder, ViewportCommand};
use jiff::Zoned;

use crate::{SpiralWidget, TryWidget as _};

pub struct SpiralApp {}

impl SpiralApp {
    pub fn run<I: IntoIterator<Item = (Zoned, String)>>(events: I) -> eyre::Result<()> {
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

        ui.add(SpiralWidget::default().into_unwrap_widget());
    }
}
