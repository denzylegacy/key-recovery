#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod theme;

use eframe::{egui, egui::ViewportBuilder};
use anyhow::Result;
use std::io;

#[cfg(target_os = "windows")]
use wmi::{COMLibrary, WMIConnection};

const APP_NAME: &str = "KEY RECOVERY";

struct KeyRecoveryApp {
    key: Option<String>,
    error: Option<String>,
}

impl Default for KeyRecoveryApp {
    fn default() -> Self {
        Self {
            key: None,
            error: None,
        }
    }
}

impl eframe::App for KeyRecoveryApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.heading(APP_NAME);
                });

                ui.add_space(20.0);

                if ui.add_sized([ui.available_width(), 40.0], egui::Button::new("RECOVER KEY")).clicked() {
                    match get_key() {
                        Ok(key) => {
                            self.key = Some(key);
                            self.error = None;
                        }
                        Err(err) => {
                            self.error = Some(err.to_string());
                            self.key = None;
                        }
                    }
                }

                ui.add_space(20.0);

                if let Some(key) = &self.key {
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.label("RECOVERED KEY:");
                        ui.add_space(5.0);
                        ui.add_enabled_ui(false, |ui| {
                            ui.text_edit_singleline(&mut key.clone());
                        });
                        ui.add_space(10.0);
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                            if ui.add_sized([180.0, 35.0], egui::Button::new("COPY TO CLIPBOARD")).clicked() {
                                ui.output_mut(|o| o.copied_text = key.clone());
                            }
                            ui.add_space(5.0);
                            if ui.add_sized([180.0, 35.0], egui::Button::new("SAVE TO KEY.TXT")).clicked() {
                                if let Err(e) = std::fs::write("key.txt", key) {
                                    self.error = Some(format!("Failed to save key: {}", e));
                                }
                            }
                        });
                    });
                }

                if let Some(error) = &self.error {
                    ui.add_space(10.0);
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        ui.colored_label(egui::Color32::RED, error);
                    });
                }

                ui.add_space(ui.available_height() - 50.0); // Push content up

                ui.separator();
                ui.add_space(10.0);

                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new("Consider donating to support development:").text_style(egui::TextStyle::Small));
                    if ui.link(egui::RichText::new("bc1qzvrarhktufp062nws8ak5mkje4meymku3hgxn9").text_style(egui::TextStyle::Small)).clicked() {
                        let url = "https://www.blockchain.com/btc/address/bc1qzvrarhktufp062nws8ak5mkje4meymku3hgxn9";
                        // Check if running as root
                        if let Ok(user) = std::env::var("SUDO_USER") {
                            // If SUDO_USER is set, try to open as that user
                            println!("Attempting to open browser via sudo -u {} xdg-open...", user);
                            if let Err(e) = std::process::Command::new("sudo")
                                .arg("-u")
                                .arg(&user)
                                .arg("xdg-open")
                                .arg(url)
                                .spawn()
                            {
                                eprintln!("Failed to open browser with sudo -u: {}", e);
                            }
                        } else {
                            // Otherwise, just use xdg-open directly
                            println!("Attempting to open browser via xdg-open...");
                            if let Err(e) = std::process::Command::new("xdg-open").arg(url).spawn() {
                                eprintln!("Failed to open browser with xdg-open: {}", e);
                            }
                        }
                    }
                    ui.add_space(5.0);
                });
            });
        });
    }
}

#[cfg(target_os = "windows")]
fn get_key() -> Result<String> {
    let com_lib = COMLibrary::new()?;
    let wmi_con = WMIConnection::new(com_lib.into())?;
    let results: Vec<std::collections::HashMap<String, wmi::Variant>> = wmi_con.raw_query("SELECT OA3xOriginalProductKey FROM SoftwareLicensingService")?;

    for res in results {
        if let Some(wmi::Variant::String(key)) = res.get("OA3xOriginalProductKey") {
            return Ok(key.to_string());
        }
    }

    Err(anyhow::anyhow!("Could not find the product key."))
}

#[cfg(target_os = "linux")]
fn get_key() -> Result<String> {
    const MSDM_PATH: &str = "/sys/firmware/acpi/tables/MSDM";
    if std::path::Path::new(MSDM_PATH).exists() {
        match std::fs::read(MSDM_PATH) {
            Ok(msdm_data) => {
                let key = String::from_utf8_lossy(&msdm_data);
                let re = regex::Regex::new(r"[A-Z0-9]{5}-[A-Z0-9]{5}-[A-Z0-9]{5}-[A-Z0-9]{5}-[A-Z0-9]{5}").unwrap();
                if let Some(mat) = re.find(&key) {
                    return Ok(mat.as_str().to_string());
                }
                Err(anyhow::anyhow!("No key found in the MSDM table"))
            }
            Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                Err(anyhow::anyhow!("Permission denied. Please run with sudo."))
            }
            Err(e) => Err(e.into()),
        }
    } else {
        Err(anyhow::anyhow!("MSDM table not found - System does not have an embedded OEM key"))
    }
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
fn get_key() -> Result<String> {
    Err(anyhow::anyhow!("Unsupported OS"))
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size(egui::vec2(350.0, 350.0)) // Smaller size
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|cc| {
            theme::apply_theme(&cc.egui_ctx);
            Ok(Box::<KeyRecoveryApp>::default())
        }),
    )
}
