use harana_common::tauri;
use harana_common::tauri::{AppHandle, LogicalSize, Size};

pub fn window(handle: AppHandle, name: &str, html_file: &str) {
    tauri::WindowBuilder::new(&handle, name, tauri::WindowUrl::App(html_file.into()))
        .visible(false)
        .initialization_script(include_str!("../../../../scripts/html2canvas.js"))
        .initialization_script(include_str!("../../../../scripts/jspdf.js"))
        .build()
        .unwrap()
        .set_size(Size::from(LogicalSize::new(400, 400)))
        .unwrap()
}