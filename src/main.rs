#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use midir::{Ignore, MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

const PORT_REFRESH_INTERVAL: Duration = Duration::from_secs(1);

struct RouterState {
    note_counts: [[u32; 128]; 16],
    output: Option<MidiOutputConnection>,
}

impl Default for RouterState {
    fn default() -> Self {
        Self {
            note_counts: [[0; 128]; 16],
            output: None,
        }
    }
}

impl RouterState {
    fn handle_message(&mut self, message: &[u8]) {
        if should_forward(&mut self.note_counts, message)
            && let Some(output) = &mut self.output
        {
            let _ = output.send(message);
        }
    }

    fn silence(&mut self) {
        if let Some(output) = &mut self.output {
            for channel in 0..16 {
                for note in 0..128 {
                    if self.note_counts[channel][note] > 0 {
                        let _ = output.send(&[0x80 | channel as u8, note as u8, 0]);
                    }
                }

                let _ = output.send(&[0xB0 | channel as u8, 123, 0]);
            }
        }

        self.note_counts = [[0; 128]; 16];
    }
}

fn should_forward(note_counts: &mut [[u32; 128]; 16], message: &[u8]) -> bool {
    if message.len() < 3 {
        return true;
    }

    let status = message[0];
    let message_type = status & 0xF0;
    let channel = (status & 0x0F) as usize;
    let note = message[1] as usize;

    if note >= 128 {
        return true;
    }

    let is_note_on = message_type == 0x90 && message[2] != 0;
    let is_note_off = message_type == 0x80 || (message_type == 0x90 && message[2] == 0);

    if is_note_on {
        let count = &mut note_counts[channel][note];
        let first = *count == 0;
        *count = count.saturating_add(1);
        first
    } else if is_note_off {
        let count = &mut note_counts[channel][note];
        if *count == 0 {
            false
        } else {
            *count -= 1;
            *count == 0
        }
    } else {
        true
    }
}

struct MidiMergerApp {
    router: Arc<Mutex<RouterState>>,
    input_connection: Option<MidiInputConnection<()>>,
    input_ports: Vec<String>,
    output_ports: Vec<String>,
    selected_input: Option<usize>,
    selected_output: Option<usize>,
    last_refresh: Instant,
}

impl Default for MidiMergerApp {
    fn default() -> Self {
        let mut app = Self {
            router: Arc::new(Mutex::new(RouterState::default())),
            input_connection: None,
            input_ports: Vec::new(),
            output_ports: Vec::new(),
            selected_input: None,
            selected_output: None,
            last_refresh: Instant::now() - PORT_REFRESH_INTERVAL,
        };
        app.refresh_ports();
        app
    }
}

impl MidiMergerApp {
    fn refresh_ports(&mut self) {
        let previous_input = self
            .selected_input
            .and_then(|index| self.input_ports.get(index).cloned());
        let previous_output = self
            .selected_output
            .and_then(|index| self.output_ports.get(index).cloned());

        self.input_ports = list_input_ports();
        self.output_ports = list_output_ports();

        self.selected_input = previous_input
            .as_ref()
            .and_then(|name| self.input_ports.iter().position(|port| port == name));
        self.selected_output = previous_output
            .as_ref()
            .and_then(|name| self.output_ports.iter().position(|port| port == name));

        if previous_input.is_some() && self.selected_input.is_none() {
            self.disconnect_input();
        }
        if previous_output.is_some() && self.selected_output.is_none() {
            self.disconnect_output();
        }

        self.last_refresh = Instant::now();
    }

    fn select_input(&mut self, selection: Option<usize>) {
        self.disconnect_input();
        self.selected_input = None;

        let Some(index) = selection else {
            return;
        };

        let Ok(mut midi_input) = MidiInput::new("MIDI Merger input") else {
            return;
        };
        midi_input.ignore(Ignore::None);
        let ports = midi_input.ports();
        let Some(port) = ports.get(index) else {
            return;
        };

        let router = Arc::clone(&self.router);
        if let Ok(connection) = midi_input.connect(
            port,
            "MIDI Merger input connection",
            move |_timestamp, message, _| {
                if let Ok(mut router) = router.lock() {
                    router.handle_message(message);
                }
            },
            (),
        ) {
            self.input_connection = Some(connection);
            self.selected_input = Some(index);
        }
    }

    fn select_output(&mut self, selection: Option<usize>) {
        self.disconnect_output();
        self.selected_output = None;

        let Some(index) = selection else {
            return;
        };

        let Ok(midi_output) = MidiOutput::new("MIDI Merger output") else {
            return;
        };
        let ports = midi_output.ports();
        let Some(port) = ports.get(index) else {
            return;
        };

        if let Ok(connection) = midi_output.connect(port, "MIDI Merger output connection")
            && let Ok(mut router) = self.router.lock()
        {
            router.output = Some(connection);
            self.selected_output = Some(index);
        }
    }

    fn disconnect_input(&mut self) {
        self.input_connection = None;
        if let Ok(mut router) = self.router.lock() {
            router.silence();
        }
    }

    fn disconnect_output(&mut self) {
        if let Ok(mut router) = self.router.lock() {
            router.silence();
            router.output = None;
        }
    }
}

impl Drop for MidiMergerApp {
    fn drop(&mut self) {
        self.disconnect_input();
        self.disconnect_output();
    }
}

impl eframe::App for MidiMergerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.last_refresh.elapsed() >= PORT_REFRESH_INTERVAL {
            self.refresh_ports();
        }
        ui.ctx().request_repaint_after(PORT_REFRESH_INTERVAL);

        egui::CentralPanel::default().show(ui, |ui| {
            let mut input_selection = self.selected_input;
            let mut output_selection = self.selected_output;

            ui.horizontal(|ui| {
                ui.label("Entrée");
                egui::ComboBox::from_id_salt("midi_input")
                    .width(280.0)
                    .selected_text(selected_name(&self.input_ports, input_selection))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut input_selection, None, "Aucune");
                        for (index, name) in self.input_ports.iter().enumerate() {
                            ui.selectable_value(&mut input_selection, Some(index), name);
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Sortie");
                egui::ComboBox::from_id_salt("midi_output")
                    .width(280.0)
                    .selected_text(selected_name(&self.output_ports, output_selection))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut output_selection, None, "Aucune");
                        for (index, name) in self.output_ports.iter().enumerate() {
                            ui.selectable_value(&mut output_selection, Some(index), name);
                        }
                    });
            });

            if input_selection != self.selected_input {
                self.select_input(input_selection);
            }
            if output_selection != self.selected_output {
                self.select_output(output_selection);
            }
        });
    }
}

fn selected_name(ports: &[String], selection: Option<usize>) -> &str {
    selection
        .and_then(|index| ports.get(index))
        .map_or("Aucune", String::as_str)
}

fn list_input_ports() -> Vec<String> {
    let Ok(input) = MidiInput::new("MIDI Merger input scan") else {
        return Vec::new();
    };

    input
        .ports()
        .iter()
        .map(|port| {
            input
                .port_name(port)
                .unwrap_or_else(|_| "Entrée MIDI inconnue".into())
        })
        .collect()
}

fn list_output_ports() -> Vec<String> {
    let Ok(output) = MidiOutput::new("MIDI Merger output scan") else {
        return Vec::new();
    };

    output
        .ports()
        .iter()
        .map(|port| {
            output
                .port_name(port)
                .unwrap_or_else(|_| "Sortie MIDI inconnue".into())
        })
        .collect()
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 100.0])
            .with_resizable(false),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native(
        "MIDI Merger",
        options,
        Box::new(|_cc| Ok(Box::new(MidiMergerApp::default()))),
    )
}

#[cfg(test)]
mod tests {
    use super::should_forward;

    #[test]
    fn keeps_only_first_note_on_and_last_note_off() {
        let mut counts = [[0; 128]; 16];

        for index in 0..5 {
            assert_eq!(should_forward(&mut counts, &[0x92, 60, 100]), index == 0);
        }
        for index in 0..5 {
            assert_eq!(should_forward(&mut counts, &[0x82, 60, 64]), index == 4);
        }
    }

    #[test]
    fn separates_channels_and_treats_zero_velocity_as_note_off() {
        let mut counts = [[0; 128]; 16];

        assert!(should_forward(&mut counts, &[0x90, 60, 100]));
        assert!(should_forward(&mut counts, &[0x91, 60, 100]));
        assert!(should_forward(&mut counts, &[0x90, 60, 0]));
        assert!(should_forward(&mut counts, &[0x81, 60, 0]));
    }

    #[test]
    fn forwards_other_messages_unchanged() {
        let mut counts = [[0; 128]; 16];

        assert!(should_forward(&mut counts, &[0xB0, 1, 127]));
        assert!(should_forward(&mut counts, &[0xE4, 0, 64]));
        assert!(should_forward(&mut counts, &[0xF8]));
    }

    #[test]
    fn ignores_unmatched_note_off() {
        let mut counts = [[0; 128]; 16];

        assert!(!should_forward(&mut counts, &[0x80, 60, 0]));
    }
}
