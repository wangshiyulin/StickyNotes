#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{api::dialog::blocking::FileDialogBuilder, Manager, State, Window, WindowEvent};

#[derive(Serialize, Deserialize, Clone)]
struct Bounds { width: f64, height: f64, x: Option<f64>, y: Option<f64> }

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    storage_dir: String,
    theme: String,
    always_on_top: bool,
    bounds: Bounds,
    #[serde(default = "default_close_action")]
    close_action: String, // "minimize" | "close"
}

fn default_close_action() -> String { "minimize".into() }

impl Default for Config {
    fn default() -> Self {
        let dir = dirs::home_dir().unwrap_or(PathBuf::from(".")).join("StickyNotes");
        Self {
            storage_dir: dir.to_string_lossy().to_string(),
            theme: "light".into(),
            always_on_top: true,
            bounds: Bounds { width: 420.0, height: 540.0, x: None, y: None },
            close_action: "minimize".into(),
        }
    }
}

struct AppState { cfg: Mutex<Config>, cfg_path: PathBuf }

fn config_path() -> PathBuf {
    dirs::config_dir().unwrap_or(PathBuf::from(".")).join("StickyNotes").join("config.json")
}

fn load_config() -> Config {
    let p = config_path();
    fs::read_to_string(&p).ok()
        .and_then(|s| serde_json::from_str::<Config>(&s).ok())
        .unwrap_or_default()
}

fn write_config(cfg: &Config, path: &PathBuf) {
    if let Some(parent) = path.parent() { let _ = fs::create_dir_all(parent); }
    if let Ok(s) = serde_json::to_string_pretty(cfg) { let _ = fs::write(path, s); }
}

fn notes_file(cfg: &Config) -> PathBuf { PathBuf::from(&cfg.storage_dir).join("notes.json") }

fn ensure_storage(cfg: &Config) {
    let _ = fs::create_dir_all(&cfg.storage_dir);
    let f = notes_file(cfg);
    if !f.exists() { let _ = fs::write(&f, "[]"); }
}

fn read_notes(cfg: &Config) -> serde_json::Value {
    ensure_storage(cfg);
    fs::read_to_string(notes_file(cfg)).ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .filter(|v| v.is_array())
        .unwrap_or(serde_json::json!([]))
}

fn write_notes(cfg: &Config, notes: &serde_json::Value) -> Result<(), String> {
    ensure_storage(cfg);
    let s = serde_json::to_string_pretty(notes).map_err(|e| e.to_string())?;
    fs::write(notes_file(cfg), s).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_config(state: State<AppState>) -> Config { state.cfg.lock().unwrap().clone() }

#[tauri::command]
fn get_notes(state: State<AppState>) -> serde_json::Value {
    read_notes(&state.cfg.lock().unwrap())
}

#[tauri::command]
fn save_notes(state: State<AppState>, notes: serde_json::Value) -> Result<(), String> {
    write_notes(&state.cfg.lock().unwrap(), &notes)
}

#[tauri::command]
fn set_pin(window: Window, state: State<AppState>, value: bool) -> bool {
    let mut c = state.cfg.lock().unwrap();
    c.always_on_top = value;
    let _ = window.set_always_on_top(value);
    write_config(&c, &state.cfg_path);
    value
}

#[tauri::command]
fn set_theme(state: State<AppState>, theme: String) -> String {
    let mut c = state.cfg.lock().unwrap();
    c.theme = if theme == "dark" { "dark".into() } else { "light".into() };
    write_config(&c, &state.cfg_path);
    c.theme.clone()
}

#[tauri::command]
fn set_close_action(state: State<AppState>, action: String) -> String {
    let mut c = state.cfg.lock().unwrap();
    c.close_action = if action == "close" { "close".into() } else { "minimize".into() };
    write_config(&c, &state.cfg_path);
    c.close_action.clone()
}

#[tauri::command]
fn set_window_size(window: Window, state: State<AppState>, width: f64, height: f64) -> Result<(), String> {
    let w = width.clamp(280.0, 2000.0);
    let h = height.clamp(360.0, 2000.0);
    window.set_size(tauri::LogicalSize::new(w, h)).map_err(|e| e.to_string())?;
    let mut c = state.cfg.lock().unwrap();
    c.bounds.width = w; c.bounds.height = h;
    write_config(&c, &state.cfg_path);
    Ok(())
}

#[tauri::command]
fn pick_storage_dir(state: State<AppState>) -> Option<String> {
    let current = state.cfg.lock().unwrap().storage_dir.clone();
    let picked = FileDialogBuilder::new().set_directory(&current).pick_folder()?;
    let new_dir = picked.to_string_lossy().to_string();
    let existing = read_notes(&state.cfg.lock().unwrap());
    {
        let mut c = state.cfg.lock().unwrap();
        c.storage_dir = new_dir.clone();
        write_config(&c, &state.cfg_path);
        ensure_storage(&c);
        let nf = notes_file(&c);
        let empty = fs::read_to_string(&nf).map(|s| s.trim() == "[]").unwrap_or(true);
        if empty { let _ = write_notes(&c, &existing); }
    }
    Some(new_dir)
}

#[tauri::command]
fn export_notes(state: State<AppState>) -> Option<String> {
    let path = FileDialogBuilder::new()
        .set_file_name(&format!("stickynotes-backup-{}.json", chrono_ts()))
        .add_filter("JSON", &["json"])
        .save_file()?;
    let notes = read_notes(&state.cfg.lock().unwrap());
    let s = serde_json::to_string_pretty(&notes).ok()?;
    fs::write(&path, s).ok()?;
    Some(path.to_string_lossy().to_string())
}

#[tauri::command]
fn import_notes(state: State<AppState>, mode: String) -> Result<serde_json::Value, String> {
    let path = FileDialogBuilder::new().add_filter("JSON", &["json"]).pick_file()
        .ok_or_else(|| "canceled".to_string())?;
    let raw = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let imported: serde_json::Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    let imported_arr = imported.as_array().ok_or("格式错误：根节点不是数组")?.clone();
    let cfg = state.cfg.lock().unwrap();
    let merged = if mode == "replace" {
        serde_json::Value::Array(imported_arr)
    } else {
        let mut cur = read_notes(&cfg).as_array().cloned().unwrap_or_default();
        cur.extend(imported_arr);
        serde_json::Value::Array(cur)
    };
    write_notes(&cfg, &merged)?;
    Ok(merged)
}

#[tauri::command]
fn backup_notes(state: State<AppState>) -> Result<String, String> {
    let cfg = state.cfg.lock().unwrap();
    ensure_storage(&cfg);
    let dest = PathBuf::from(&cfg.storage_dir).join(format!("notes-backup-{}.json", chrono_ts()));
    fs::copy(notes_file(&cfg), &dest).map_err(|e| e.to_string())?;
    Ok(dest.to_string_lossy().to_string())
}

#[tauri::command]
fn save_bounds(state: State<AppState>, x: f64, y: f64, width: f64, height: f64) {
    let mut c = state.cfg.lock().unwrap();
    c.bounds = Bounds { width, height, x: Some(x), y: Some(y) };
    write_config(&c, &state.cfg_path);
}

fn chrono_ts() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0)
}

fn main() {
    let cfg = load_config();
    let cfg_path = config_path();

    tauri::Builder::default()
        .setup(move |app| {
            let win = app.get_window("main").unwrap();
            let state = app.state::<AppState>();
            let cur = state.cfg.lock().unwrap().clone();
            let _ = win.set_always_on_top(cur.always_on_top);
            use tauri::{LogicalSize, LogicalPosition};
            let _ = win.set_size(LogicalSize::new(cur.bounds.width, cur.bounds.height));
            if let (Some(x), Some(y)) = (cur.bounds.x, cur.bounds.y) {
                let _ = win.set_position(LogicalPosition::new(x, y));
            }
            Ok(())
        })
        .on_window_event(|event| {
            if let WindowEvent::CloseRequested { api, .. } = event.event() {
                let win = event.window();
                let state = win.state::<AppState>();
                let action = state.cfg.lock().unwrap().close_action.clone();
                if action == "minimize" {
                    api.prevent_close();
                    let _ = win.minimize();
                }
            }
        })
        .manage(AppState { cfg: Mutex::new(cfg), cfg_path })
        .invoke_handler(tauri::generate_handler![
            get_config, get_notes, save_notes, set_pin, set_theme,
            set_close_action, set_window_size,
            pick_storage_dir, export_notes, import_notes, backup_notes, save_bounds
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
