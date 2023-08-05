use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::sync::mpsc;
use eframe::egui;

pub struct QRServer {
	receiver: mpsc::Receiver<(String, String)>,
	ctx: egui::Context,
	pub texture: egui::TextureHandle
}

impl QRServer {
	pub fn new(ctx: egui::Context, pass: &str) -> Self {
		let data = qrcode_generator::to_png_to_vec(format!("WIFI:T:ADB;S:adb-helper;P:{};;", pass), qrcode_generator::QrCodeEcc::Low, 1024).expect("Failed to generate a qr-code!");
		let texture = ctx.load_texture(
			"qr-code",
			load_image_from_memory(&data).expect("Failed to create image from data!"),
			Default::default()
		);

		let (snd, rec) = mpsc::channel();
		let obj = Self {
			receiver: rec,
			ctx,
			texture
		};
		obj.start(snd);
		obj
	}

	pub fn recv(&self) -> Option<(String, String)> {
		match self.receiver.recv_timeout(std::time::Duration::from_millis(10)) {
			Ok(data) => Some(data),
			Err(_) => None
		}
	}

	fn start(&self, snd: std::sync::mpsc::Sender<(String, String)>) {
		let mdns = ServiceDaemon::new().expect("Failed to create daemon");

		let service_type = "_adb-tls-pairing._tcp.local.";
		let receiver = mdns.browse(service_type).expect("Failed to browse");

		let ctx_copy = self.ctx.clone();
		std::thread::spawn(move || {
			loop {
				let event = receiver.recv();
				match event {
					Ok(event) => {
						match event {
							ServiceEvent::ServiceResolved(info) => {
								let addr = info.get_addresses().into_iter().next().unwrap().to_string();
								println!("Resolved a new service: {}: {}, {}", info.get_fullname(), &addr, info.get_port());
								snd.send((addr, info.get_port().to_string())).unwrap();
								ctx_copy.request_repaint();
							}
							other_event => {
								println!("Received other event: {:?}", &other_event);
							}
						}
					}
					Err(e) => {
						println!("Error: {}", e);
					}
				}
			}
		});
	}
}

fn load_image_from_memory(image_data: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}