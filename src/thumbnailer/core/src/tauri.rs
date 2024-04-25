use harana_common::anyhow::Result;
use harana_common::tauri::{AppHandle, WindowBuilder, WindowUrl, Wry};

pub fn open_tauri_thumbnail_window(app: &'static AppHandle<Wry>, document_id: u64) -> Result<()> {
    let window = WindowBuilder::new(app, document_id.to_string(), WindowUrl::App("index.html".into())).decorations(false).always_on_top(true).visible(false).resizable(false).build()?;
    window.eval(format!("window.location.replace('/thumbnail/{}')", document_id).as_str()).unwrap();
    window.eval("document.body.style.zoom = '120%'").unwrap();
    Ok(())
}