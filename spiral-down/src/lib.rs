#![allow(dead_code)]

use chrono::{DateTime, Local, LocalResult, NaiveDateTime};
use clap::Parser;
use color_eyre::eyre::{eyre, Result, WrapErr};
use eframe::egui;
use std::f32::consts::{FRAC_PI_2, TAU};

#[derive(Parser, Debug)]
#[command(author, version, about = "Spiral-down event countdown display")]
struct Args {
    /// Events
    events: Vec<String>,
}

pub fn run() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();
    let mut events = vec![];

    for text in args.events {
        let (label, timetext) = text
            .rsplit_once('@')
            .ok_or(eyre!("expected `title @ timestamp`"))?;

        let label = label.trim().to_string();
        let timetext = timetext.trim();
        let time = parse_timestamp(timetext)
            .wrap_err_with(|| format!("Failed to parse timestamp {timetext:?}"))?;

        events.push(Event { label, time });
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "spiral-down",
        options,
        Box::new(|_cc| Ok(Box::new(SpiralApp::new(events)))),
    )
    .map_err(|e| eyre!("eframe error: {e}"))?;

    Ok(())
}

fn parse_timestamp(s: &str) -> Result<DateTime<Local>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Local));
    }
    for fmt in &["%Y-%m-%d %H:%M:%S", "%Y-%m-%dT%H:%M:%S"] {
        if let Ok(naive) = NaiveDateTime::parse_from_str(s, fmt) {
            match naive.and_local_timezone(Local) {
                LocalResult::Single(dt) | LocalResult::Ambiguous(dt, _) => return Ok(dt),
                LocalResult::None => continue,
            }
        }
    }
    Err(eyre!(
        "Could not parse '{}'. Use RFC3339 or 'YYYY-MM-DD HH:MM:SS'",
        s
    ))
}

struct SpiralApp {
    events: Vec<Event>,
}

struct Event {
    label: String,
    time: LocalTime,
}

type LocalTime = DateTime<Local>;

impl SpiralApp {
    fn new(events: Vec<Event>) -> Self {
        // let timestamp_display = event_time.format("%Y-%m-%d %H:%M:%S").to_string();
        Self { events }
    }
}

impl eframe::App for SpiralApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Repaint frequently enough for a smooth second-tick countdown.
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(250));

        // let now = Local::now();
        // let remaining = self.event_time.signed_duration_since(now);
        // let total_secs = remaining.num_seconds().max(0);

        let rect = ui.max_rect();
        let center = rect.center();
        let radius = rect.width().min(rect.height()) * 0.9;
        let painter = ui.painter();

        {
            let pts: Vec<egui::Pos2> = (0..=1000)
                .map(|i| {
                    let a = i as f32 / 100.0 * TAU - FRAC_PI_2;
                    let r = radius.powi(i);
                    egui::pos2(center.x + r * a.cos(), center.y + r * a.sin())
                })
                .collect();

            painter.add(egui::Shape::line(
                pts,
                egui::Stroke::new(4.0, egui::Color32::from_gray(90)),
            ));
        }
    }
}
