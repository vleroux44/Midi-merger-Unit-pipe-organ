#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use midir::{Ignore, MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use std::{
    ffi::c_void,
    ptr::{null, null_mut},
    sync::{Arc, Mutex},
};
use windows_sys::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, WPARAM},
    Graphics::Gdi::{COLOR_WINDOW, GetSysColorBrush, SetBkMode, SetTextColor, TRANSPARENT},
    System::LibraryLoader::GetModuleHandleW,
    UI::{
        Shell::ShellExecuteW,
        WindowsAndMessaging::{
            CB_ADDSTRING, CB_GETCURSEL, CB_RESETCONTENT, CB_SETCURSEL, CBN_SELCHANGE,
            CBS_DROPDOWNLIST, CREATESTRUCTW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW,
            DispatchMessageW, GWLP_USERDATA, GetMessageW, GetWindowLongPtrW, IDC_ARROW, IDC_HAND,
            LoadCursorW, LoadIconW, MSG, PostQuitMessage, RegisterClassW, STN_CLICKED, SW_SHOW,
            SendMessageW, SetCursor, SetTimer, SetWindowLongPtrW, ShowWindow, TranslateMessage,
            WM_CLOSE, WM_COMMAND, WM_CREATE, WM_CTLCOLORSTATIC, WM_DESTROY, WM_NCCREATE,
            WM_SETCURSOR, WM_TIMER, WNDCLASSW, WS_CAPTION, WS_CHILD, WS_MINIMIZEBOX, WS_OVERLAPPED,
            WS_SYSMENU, WS_TABSTOP, WS_VISIBLE, WS_VSCROLL,
        },
    },
};

const INPUT_COMBO_ID: usize = 1001;
const OUTPUT_COMBO_ID: usize = 1002;
const WEBSITE_LINK_ID: usize = 1003;
const REFRESH_TIMER_ID: usize = 1;
const SS_NOTIFY_STYLE: u32 = 0x0100;
const LINK_BLUE: u32 = 0x00CC_6600;

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

    if message_type == 0x90 && message[2] != 0 {
        let count = &mut note_counts[channel][note];
        let first = *count == 0;
        *count = count.saturating_add(1);
        first
    } else if message_type == 0x80 || (message_type == 0x90 && message[2] == 0) {
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

struct NativeApp {
    router: Arc<Mutex<RouterState>>,
    input_connection: Option<MidiInputConnection<()>>,
    input_ports: Vec<String>,
    output_ports: Vec<String>,
    selected_input: Option<usize>,
    selected_output: Option<usize>,
    input_combo: HWND,
    output_combo: HWND,
    website_link: HWND,
}

impl Default for NativeApp {
    fn default() -> Self {
        Self {
            router: Arc::new(Mutex::new(RouterState::default())),
            input_connection: None,
            input_ports: Vec::new(),
            output_ports: Vec::new(),
            selected_input: None,
            selected_output: None,
            input_combo: null_mut(),
            output_combo: null_mut(),
            website_link: null_mut(),
        }
    }
}

impl NativeApp {
    unsafe fn create_controls(&mut self, window: HWND) {
        let static_class = wide("STATIC");
        let combo_class = wide("COMBOBOX");
        let input_label = wide("Entrée");
        let output_label = wide("Sortie");
        let copyright = wide("© 2026 Valentin Leroux —");
        let website = wide("www.organologic.fr");
        let instance = unsafe { GetModuleHandleW(null()) };

        unsafe {
            CreateWindowExW(
                0,
                static_class.as_ptr(),
                input_label.as_ptr(),
                WS_CHILD | WS_VISIBLE,
                12,
                17,
                52,
                22,
                window,
                null_mut(),
                instance,
                null(),
            );
            self.input_combo = CreateWindowExW(
                0,
                combo_class.as_ptr(),
                null(),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_VSCROLL | CBS_DROPDOWNLIST as u32,
                68,
                12,
                330,
                240,
                window,
                INPUT_COMBO_ID as *mut c_void,
                instance,
                null(),
            );
            CreateWindowExW(
                0,
                static_class.as_ptr(),
                output_label.as_ptr(),
                WS_CHILD | WS_VISIBLE,
                12,
                54,
                52,
                22,
                window,
                null_mut(),
                instance,
                null(),
            );
            self.output_combo = CreateWindowExW(
                0,
                combo_class.as_ptr(),
                null(),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | WS_VSCROLL | CBS_DROPDOWNLIST as u32,
                68,
                49,
                330,
                240,
                window,
                OUTPUT_COMBO_ID as *mut c_void,
                instance,
                null(),
            );
            CreateWindowExW(
                0,
                static_class.as_ptr(),
                copyright.as_ptr(),
                WS_CHILD | WS_VISIBLE,
                12,
                88,
                160,
                24,
                window,
                null_mut(),
                instance,
                null(),
            );
            self.website_link = CreateWindowExW(
                0,
                static_class.as_ptr(),
                website.as_ptr(),
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | SS_NOTIFY_STYLE,
                176,
                88,
                132,
                24,
                window,
                WEBSITE_LINK_ID as *mut c_void,
                instance,
                null(),
            );
        }

        unsafe {
            fill_combo(self.input_combo, &self.input_ports, None);
            fill_combo(self.output_combo, &self.output_ports, None);
        }
        self.refresh_ports();
    }

    fn refresh_ports(&mut self) {
        let previous_input = self
            .selected_input
            .and_then(|index| self.input_ports.get(index).cloned());
        let previous_output = self
            .selected_output
            .and_then(|index| self.output_ports.get(index).cloned());
        let new_inputs = list_input_ports();
        let new_outputs = list_output_ports();

        if new_inputs != self.input_ports {
            self.input_ports = new_inputs;
            self.selected_input = previous_input
                .as_ref()
                .and_then(|name| self.input_ports.iter().position(|port| port == name));
            if previous_input.is_some() && self.selected_input.is_none() {
                self.disconnect_input();
            }
            unsafe { fill_combo(self.input_combo, &self.input_ports, self.selected_input) };
        }

        if new_outputs != self.output_ports {
            self.output_ports = new_outputs;
            self.selected_output = previous_output
                .as_ref()
                .and_then(|name| self.output_ports.iter().position(|port| port == name));
            if previous_output.is_some() && self.selected_output.is_none() {
                self.disconnect_output();
            }
            unsafe { fill_combo(self.output_combo, &self.output_ports, self.selected_output) };
        }
    }

    fn select_input(&mut self, selection: Option<usize>) {
        self.disconnect_input();
        self.selected_input = None;
        let Some(index) = selection else { return };
        let Ok(mut input) = MidiInput::new("MIDI Merger native input") else {
            return;
        };
        input.ignore(Ignore::None);
        let ports = input.ports();
        let Some(port) = ports.get(index) else { return };
        let router = Arc::clone(&self.router);

        if let Ok(connection) = input.connect(
            port,
            "MIDI Merger native input connection",
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
        let Some(index) = selection else { return };
        let Ok(output) = MidiOutput::new("MIDI Merger native output") else {
            return;
        };
        let ports = output.ports();
        let Some(port) = ports.get(index) else { return };

        if let Ok(connection) = output.connect(port, "MIDI Merger native output connection")
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

    fn shutdown(&mut self) {
        self.disconnect_input();
        self.disconnect_output();
    }
}

unsafe fn fill_combo(combo: HWND, ports: &[String], selection: Option<usize>) {
    unsafe {
        SendMessageW(combo, CB_RESETCONTENT, 0, 0);
        let none = wide("Aucune");
        SendMessageW(combo, CB_ADDSTRING, 0, none.as_ptr() as LPARAM);
        for port in ports {
            let name = wide(port);
            SendMessageW(combo, CB_ADDSTRING, 0, name.as_ptr() as LPARAM);
        }
        SendMessageW(
            combo,
            CB_SETCURSEL,
            selection.map_or(0, |index| index + 1),
            0,
        );
    }
}

unsafe extern "system" fn window_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if message == WM_NCCREATE {
        let create = lparam as *const CREATESTRUCTW;
        unsafe {
            SetWindowLongPtrW(window, GWLP_USERDATA, (*create).lpCreateParams as isize);
        }
    }

    let app_ptr = unsafe { GetWindowLongPtrW(window, GWLP_USERDATA) } as *mut NativeApp;

    match message {
        WM_CREATE if !app_ptr.is_null() => {
            unsafe {
                (*app_ptr).create_controls(window);
                SetTimer(window, REFRESH_TIMER_ID, 1000, None);
            }
            0
        }
        WM_COMMAND
            if !app_ptr.is_null()
                && (wparam & 0xFFFF) == WEBSITE_LINK_ID
                && ((wparam >> 16) as u32) == STN_CLICKED =>
        {
            let operation = wide("open");
            let website = wide("https://www.organologic.fr");
            unsafe {
                ShellExecuteW(
                    window,
                    operation.as_ptr(),
                    website.as_ptr(),
                    null(),
                    null(),
                    SW_SHOW,
                );
            }
            0
        }
        WM_COMMAND if !app_ptr.is_null() && ((wparam >> 16) as u32) == CBN_SELCHANGE => {
            let control_id = wparam & 0xFFFF;
            let app = unsafe { &mut *app_ptr };
            if control_id == INPUT_COMBO_ID {
                let selected = unsafe { SendMessageW(app.input_combo, CB_GETCURSEL, 0, 0) };
                app.select_input((selected > 0).then_some(selected as usize - 1));
                unsafe {
                    SendMessageW(
                        app.input_combo,
                        CB_SETCURSEL,
                        app.selected_input.map_or(0, |index| index + 1),
                        0,
                    );
                }
            } else if control_id == OUTPUT_COMBO_ID {
                let selected = unsafe { SendMessageW(app.output_combo, CB_GETCURSEL, 0, 0) };
                app.select_output((selected > 0).then_some(selected as usize - 1));
                unsafe {
                    SendMessageW(
                        app.output_combo,
                        CB_SETCURSEL,
                        app.selected_output.map_or(0, |index| index + 1),
                        0,
                    );
                }
            }
            0
        }
        WM_TIMER if !app_ptr.is_null() && wparam == REFRESH_TIMER_ID => {
            unsafe { (*app_ptr).refresh_ports() };
            0
        }
        WM_CTLCOLORSTATIC if !app_ptr.is_null() => unsafe {
            SetBkMode(wparam as *mut c_void, TRANSPARENT as i32);
            if lparam as HWND == (*app_ptr).website_link {
                SetTextColor(wparam as *mut c_void, LINK_BLUE);
            }
            GetSysColorBrush(COLOR_WINDOW) as LRESULT
        },
        WM_SETCURSOR
            if !app_ptr.is_null() && wparam as HWND == unsafe { (*app_ptr).website_link } =>
        {
            unsafe { SetCursor(LoadCursorW(null_mut(), IDC_HAND)) };
            1
        }
        WM_CLOSE => {
            unsafe { windows_sys::Win32::UI::WindowsAndMessaging::DestroyWindow(window) };
            0
        }
        WM_DESTROY => {
            if !app_ptr.is_null() {
                unsafe { (*app_ptr).shutdown() };
            }
            unsafe { PostQuitMessage(0) };
            0
        }
        _ => unsafe { DefWindowProcW(window, message, wparam, lparam) },
    }
}

fn list_input_ports() -> Vec<String> {
    let Ok(input) = MidiInput::new("MIDI Merger native input scan") else {
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
    let Ok(output) = MidiOutput::new("MIDI Merger native output scan") else {
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

fn wide(text: &str) -> Vec<u16> {
    text.encode_utf16().chain(Some(0)).collect()
}

fn main() {
    unsafe {
        let instance = GetModuleHandleW(null());
        let class_name = wide("MidiMergerNativeWindow");
        let title = wide("MIDI merger - Unit pipe organ");
        let icon = LoadIconW(instance, 1usize as *const u16);
        let window_class = WNDCLASSW {
            lpfnWndProc: Some(window_proc),
            hInstance: instance,
            hIcon: icon,
            hCursor: LoadCursorW(null_mut(), IDC_ARROW),
            hbrBackground: (COLOR_WINDOW as usize + 1) as *mut c_void,
            lpszClassName: class_name.as_ptr(),
            ..Default::default()
        };

        if RegisterClassW(&window_class) == 0 {
            return;
        }

        let mut app = Box::new(NativeApp::default());
        let window = CreateWindowExW(
            0,
            class_name.as_ptr(),
            title.as_ptr(),
            WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            430,
            165,
            null_mut(),
            null_mut(),
            instance,
            (&mut *app as *mut NativeApp).cast(),
        );

        if window.is_null() {
            return;
        }

        ShowWindow(window, SW_SHOW);
        let mut message = MSG::default();
        while GetMessageW(&mut message, null_mut(), 0, 0) > 0 {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::should_forward;

    #[test]
    fn first_on_and_last_off_only() {
        let mut counts = [[0; 128]; 16];
        for index in 0..5 {
            assert_eq!(should_forward(&mut counts, &[0x90, 60, 100]), index == 0);
        }
        for index in 0..5 {
            assert_eq!(should_forward(&mut counts, &[0x80, 60, 0]), index == 4);
        }
    }
}
