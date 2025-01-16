#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use std::fs;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Text Editor",
        options,
        Box::new(|_cc| Ok(Box::new(TextEditor::default()))),
    )
}

struct TextEditor {
    buffer: String,     // Text buffer for editing
    file_path: String,  // File path to save/load
    message: String,    // Message for status updates
}

impl Default for TextEditor {
    fn default() -> Self {
        Self {
            buffer: String::new(),
            file_path: String::from("example.txt"), // Default file name
            message: String::from("Welcome to the Text Editor!"),
        }
    }
}

impl eframe::App for TextEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Text Editor");

            // Text editing area
            ui.add(
                egui::TextEdit::multiline(&mut self.buffer)
                    .hint_text("Start typing here...")
                    .desired_width(f32::INFINITY)
                    .desired_rows(20),
            );

            // File management buttons
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    match fs::write(&self.file_path, &self.buffer) {
                        Ok(_) => self.message = format!("Saved to '{}'", self.file_path),
                        Err(e) => self.message = format!("Error saving file: {}", e),
                    }
                }

                if ui.button("Load").clicked() {
                    match fs::read_to_string(&self.file_path) {
                        Ok(content) => {
                            self.buffer = content;
                            self.message = format!("Loaded from '{}'", self.file_path);
                        }
                        Err(e) => self.message = format!("Error loading file: {}", e),
                    }
                }
            });

            // File path input
            ui.horizontal(|ui| {
                ui.label("File path:");
                ui.text_edit_singleline(&mut self.file_path);
            });

            // Status message
            ui.label(&self.message);
        });
    }
}
