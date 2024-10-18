use egui::RichText;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::sync::{Arc, Mutex};
use web_time::{Duration, SystemTime};

// secs sin 1970.1.1 0:0:0
fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// keep sync with backend project
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct BitcoinInfo {
    latest_blocks: u64,
    difficulty: String,
    synchronized: String,
    disk_usage: String,
    prune_mode: String,
    connections: u64,
    connections_in: u64,
    connections_out: u64,
    mempool: String,
    hash_rate: String,
}

impl Default for BitcoinInfo {
    fn default() -> Self {
        Self {
            latest_blocks: 0,
            difficulty: "0 TH/s".to_string(),
            synchronized: "0%".to_string(),
            disk_usage: "0 GB".to_string(),
            prune_mode: "No".to_string(),
            connections: 0,
            connections_in: 0,
            connections_out: 0,
            mempool: "0 MB".to_string(),
            hash_rate: "0 EH/s".to_string(),
        }
    }
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    title_size: f32,
    font_size: f32,
    bitcoin_info: Arc<Mutex<BitcoinInfo>>,
    update_interval: u64,
    //resp_msg: Arc<Mutex<String>>,
    start_time: u64,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            title_size: 32.0,
            font_size: 24.0,
            bitcoin_info: Arc::new(Mutex::new(BitcoinInfo::default())),
            update_interval: 10,
            //resp_msg: Arc::new(Mutex::new("response: ".to_string())),
            start_time: unix_now(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        if unix_now() - self.start_time > self.update_interval {
            self.start_time = unix_now();
            let bitcoin_info_store = self.bitcoin_info.clone();
            let ctx_clone = ctx.clone();
            //let resp_msg_clone = self.resp_msg.clone();
            let update_interval = self.update_interval;

            let request = ehttp::Request::get("/bitcoin");
            ehttp::fetch(request, move |response| {
                if let Ok(resp) = response {
                    if let Some(rsp) = resp.text() {
                        //{
                        //    *resp_msg_clone.lock().unwrap() = format!("response {:?}", rsp);
                        //    ctx_clone.request_repaint_after(Duration::from_secs(update_interval));
                        //}
                        if let Ok(v) = serde_json::from_str(rsp) as Result<BitcoinInfo> {
                            *bitcoin_info_store.lock().unwrap() = v;
                            ctx_clone.request_repaint_after(Duration::from_secs(update_interval));
                        }
                    }
                }
            });
        }

        let bitcoin_info = self.bitcoin_info.lock().unwrap().clone();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.image(egui::include_image!("../assets/icon.png"));
                ui.label(RichText::new("Bitcoin").size(self.title_size));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Latest Blocks").size(self.font_size));
                        ui.label(
                            RichText::new(format!("{}", bitcoin_info.latest_blocks))
                                .size(self.font_size),
                        );
                    });
                });

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Synchronized").size(self.font_size));
                        ui.label(RichText::new(&bitcoin_info.synchronized).size(self.font_size));
                    });
                });

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Connections").size(self.font_size));
                        ui.label(
                            RichText::new(format!("{}", bitcoin_info.connections))
                                .size(self.font_size),
                        );
                    });
                });

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Connections In").size(self.font_size));
                        ui.label(
                            RichText::new(format!("{}", bitcoin_info.connections_in))
                                .size(self.font_size),
                        );
                    });
                });

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Connections Out").size(self.font_size));
                        ui.label(
                            RichText::new(format!("{}", bitcoin_info.connections_out))
                                .size(self.font_size),
                        );
                    });
                });

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Difficulty").size(self.font_size));
                        ui.label(RichText::new(&bitcoin_info.difficulty).size(self.font_size));
                    });
                });

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Prune Mode").size(self.font_size));
                        ui.label(RichText::new(&bitcoin_info.prune_mode).size(self.font_size));
                    });
                });

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Disk Usage").size(self.font_size));
                        ui.label(RichText::new(&bitcoin_info.disk_usage).size(self.font_size));
                    });
                });

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Mempool").size(self.font_size));
                        ui.label(RichText::new(&bitcoin_info.mempool).size(self.font_size));
                    });
                });

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        ui.label(RichText::new("Hash Rate").size(self.font_size));
                        ui.label(RichText::new(&bitcoin_info.hash_rate).size(self.font_size));
                    });
                });
            });

            //ui.separator();
            //ui.label(RichText::new(format!("{}", self.resp_msg.lock().unwrap())));
        });
    }
}
