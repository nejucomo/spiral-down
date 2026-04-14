use chrono::{DateTime, Local, LocalResult, NaiveDateTime};
use clap::Parser;
use color_eyre::eyre::{eyre, Result, WrapErr};
use eframe::egui;
use std::f32::consts::{FRAC_PI_2, TAU};

#[derive(Parser, Debug)]
#[command(author, version, about = "Spiral-down event countdown display")]
struct Args {
    /// Event title
    title: String,

    /// Event timestamp: RFC3339 (e.g. 2026-06-01T14:30:00+00:00)
    /// or local time  (e.g. "2026-06-01 14:30:00")
    timestamp: String,
}

pub fn run() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let event_time = parse_timestamp(&args.timestamp)
        .wrap_err_with(|| format!("Failed to parse timestamp '{}'", args.timestamp))?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "spiral-down",
        options,
        Box::new(|_cc| Ok(Box::new(SpiralApp::new(args.title, event_time)))),
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
    title: String,
    event_time: DateTime<Local>,
    timestamp_display: String,
}

impl SpiralApp {
    fn new(title: String, event_time: DateTime<Local>) -> Self {
        let timestamp_display = event_time.format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            title,
            event_time,
            timestamp_display,
        }
    }
}

impl eframe::App for SpiralApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Repaint frequently enough for a smooth second-tick countdown.
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(250));

        let now = Local::now();
        let remaining = self.event_time.signed_duration_since(now);
        let total_secs = remaining.num_seconds().max(0);

        {
            let rect = ui.max_rect();
            let center = rect.center();
            let radius = rect.width().min(rect.height()) * 0.35;
            let painter = ui.painter();

            let hours = total_secs / 3600;
            let mins = (total_secs % 3600) / 60;
            let secs = total_secs % 60;

            // Minutes remaining, capped at 60 (so the arc clips at one full revolution).
            let mins_f = if total_secs <= 0 {
                0.0_f32
            } else {
                (total_secs as f32 / 60.0).min(60.0)
            };

            let arc_fraction = mins_f / 60.0;
            // Arc starts at 12 o'clock and sweeps clockwise.
            let start_angle = -FRAC_PI_2;
            let end_angle = start_angle + arc_fraction * TAU;

            let arc_color = if total_secs <= 0 {
                egui::Color32::RED
            } else if mins_f < 10.0 {
                egui::Color32::from_rgb(255, 140, 0)
            } else {
                egui::Color32::from_rgb(64, 196, 255)
            };

            // Dim background ring so the user can see the full circle at a glance.
            {
                let pts: Vec<egui::Pos2> = (0..=100)
                    .map(|i| {
                        let a = i as f32 / 100.0 * TAU - FRAC_PI_2;
                        egui::pos2(center.x + radius * a.cos(), center.y + radius * a.sin())
                    })
                    .collect();
                painter.add(egui::Shape::line(
                    pts,
                    egui::Stroke::new(4.0, egui::Color32::from_gray(50)),
                ));
            }

            // Coloured countdown arc.
            if arc_fraction > 0.0 {
                let n = ((100.0 * arc_fraction).ceil() as usize).max(2);
                let pts: Vec<egui::Pos2> = (0..=n)
                    .map(|i| {
                        let t = i as f32 / n as f32;
                        let a = start_angle + t * (end_angle - start_angle);
                        egui::pos2(center.x + radius * a.cos(), center.y + radius * a.sin())
                    })
                    .collect();
                painter.add(egui::Shape::line(
                    pts,
                    egui::Stroke::new(6.0, arc_color),
                ));
            }

            // Dot at the arc tip.
            let tip = egui::pos2(
                center.x + radius * end_angle.cos(),
                center.y + radius * end_angle.sin(),
            );
            painter.circle_filled(tip, 6.0, arc_color);

            // Event label at the arc tip, offset radially outward.
            let outward = {
                let v = tip - center;
                if v.length_sq() > 0.0 {
                    v.normalized()
                } else {
                    egui::vec2(0.0, -1.0)
                }
            };
            let label_pos = tip + outward * 16.0;
            let align = if outward.x >= 0.0 {
                egui::Align2::LEFT_CENTER
            } else {
                egui::Align2::RIGHT_CENTER
            };
            painter.text(
                label_pos,
                align,
                format!("{}\n{}", self.title, self.timestamp_display),
                egui::FontId::proportional(14.0),
                egui::Color32::WHITE,
            );

            // Countdown timer in the centre.
            let countdown_text = if total_secs <= 0 {
                "Event!".to_string()
            } else {
                format!("{:02}:{:02}:{:02}", hours, mins, secs)
            };
            painter.text(
                center,
                egui::Align2::CENTER_CENTER,
                &countdown_text,
                egui::FontId::proportional(36.0),
                egui::Color32::WHITE,
            );
        }
    }
}
