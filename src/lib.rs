mod cmd;
mod qr;

use eframe::{self, egui};
use regex::Regex;

pub struct App {
	devices: String,
	address: String,
	pairing_port: String,
	pairing_code: String,
	connecting_port: String,
	my_device: String,
	qr: qr::QRServer,
	qr_size: egui::Vec2,
	pass: String
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		let num: u32 = {
			loop {
				let val: u32 = rand::prelude::random();
				if val >= 100000 && val <= 999999 {
					break val
				}
			}
		};
		let pass = num.to_string();

        let mut obj = Self {
			devices: String::new(),
			address: String::new(),
			pairing_port: String::new(),
			pairing_code: String::new(),
			connecting_port: String::new(),
			my_device: String::new(),
			qr: qr::QRServer::new(cc.egui_ctx.clone(), &pass),
			qr_size: egui::vec2(200.0, 200.0),
			pass
		};
		obj.address = String::from("192.168.0.10");
		obj.update_devices();
        obj
    }

	fn update_devices(&mut self) {
		self.devices = cmd::list_devices();
		let re = Regex::new(r"\n\S+\s").unwrap();
		self.my_device = re.find(&self.devices).and_then(|v| Some(v.as_str().to_owned())).unwrap_or_default().trim().to_owned();
	}
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
			if let Some(data) = self.qr.recv() {
				println!("Received from channel: {}:{}", &data.0, &data.1);
				self.address = data.0;
				self.pairing_port = data.1;
				self.pairing_code = self.pass.clone();
				cmd::pair(&(self.address.clone() + ":" + self.pairing_port.as_str()), &self.pairing_code);
			}

			ui.horizontal(|ui| {
				if ui.button("🔃").clicked() {
					self.update_devices();
				}
				ui.label(&self.devices);
			});

			egui::Grid::new("grid")
			.num_columns(3)
			.min_col_width(120.0)
			.show(ui, |ui| {
				ui.text_edit_singleline(&mut self.address);
				ui.end_row();

				ui.add(egui::TextEdit::singleline(&mut self.pairing_port).hint_text("Port"));
				ui.add(egui::TextEdit::singleline(&mut self.pairing_code).hint_text("Code"));
				if ui.button("Pair").clicked() {
					cmd::pair(&(self.address.clone() + ":" + self.pairing_port.as_str()), &self.pairing_code);
					self.update_devices();
				}
				ui.end_row();

				ui.add(egui::TextEdit::singleline(&mut self.connecting_port).hint_text("Port"));
				if ui.button("Connect").clicked() {
					cmd::connect(&(self.address.clone() + ":" + self.connecting_port.as_str()));
					self.update_devices();
				}
				ui.end_row();

				if ui.button("Disconnect").clicked() && !self.my_device.is_empty() {
					cmd::disconnect(&self.my_device);
					self.update_devices();
				}
			});

			ui.separator();
			ui.centered_and_justified(|ui| {
				ui.image(&self.qr.texture, self.qr_size);
			});
        });
    }
}