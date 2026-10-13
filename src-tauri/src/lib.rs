use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Command};
use tauri::Manager;

#[derive(Serialize, Deserialize)]
struct Message { role: String, content: String, created_at: String }

fn local_db(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let mut path = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    path.push("history.sqlite3"); Ok(path)
}

#[tauri::command]
fn save_message(app: tauri::AppHandle, role: String, content: String) -> Result<(), String> {
    if content.trim().is_empty() { return Err("El mensaje está vacío".into()); }
    let path = local_db(&app)?;
    let db = Connection::open(path).map_err(|e| e.to_string())?;
    db.execute("CREATE TABLE IF NOT EXISTS messages (id INTEGER PRIMARY KEY, role TEXT NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL)", []).map_err(|e| e.to_string())?;
    db.execute("INSERT INTO messages (role, content, created_at) VALUES (?1, ?2, ?3)", params![role, content, Utc::now().to_rfc3339()]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn history(app: tauri::AppHandle) -> Result<Vec<Message>, String> {
    let path = local_db(&app)?; let db = Connection::open(path).map_err(|e| e.to_string())?;
    db.execute("CREATE TABLE IF NOT EXISTS messages (id INTEGER PRIMARY KEY, role TEXT NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL)", []).map_err(|e| e.to_string())?;
    let mut stmt = db.prepare("SELECT role, content, created_at FROM messages ORDER BY id DESC LIMIT 100").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |r| Ok(Message { role:r.get(0)?, content:r.get(1)?, created_at:r.get(2)? })).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

// Hermes CLI provides the supported session/history integration. No direct writes into private state.db.
#[tauri::command]
fn ask_hermes(prompt: String) -> Result<String, String> {
    let mut child = Command::new("hermes").args(["chat", "-q", &prompt]).output().map_err(|e| format!("No se pudo ejecutar Hermes: {e}"))?;
    if !child.status.success() { return Err(String::from_utf8_lossy(&child.stderr).to_string()); }
    Ok(String::from_utf8_lossy(&child.stdout).trim().to_string())
}

#[tauri::command]
fn open_app(target: String) -> Result<(), String> {
    let (program, args): (&str, Vec<&str>) = match target.as_str() {
        "browser" => if cfg!(target_os="windows") { ("cmd", vec!["/C", "start", "", "https://hermes-agent.nousresearch.com"]) } else if cfg!(target_os="macos") { ("open", vec!["https://hermes-agent.nousresearch.com"]) } else { ("xdg-open", vec!["https://hermes-agent.nousresearch.com"]) },
        "terminal" => if cfg!(target_os="windows") { ("cmd", vec!["/C", "start", "cmd"]) } else if cfg!(target_os="macos") { ("open", vec!["-a", "Terminal"]) } else { ("x-terminal-emulator", vec![]) },
        _ => return Err("Aplicación no permitida".into()),
    };
    Command::new(program).args(args).spawn().map(|_| ()).map_err(|e| e.to_string())
}

#[tauri::command]
fn run_local_command(command: String) -> Result<String, String> {
    // Deliberately narrow allowlist: never invoke a shell or accept arbitrary command strings.
    let output = match command.as_str() {
        "pwd" => if cfg!(target_os="windows") { Command::new("cmd").args(["/C","cd"]).output() } else { Command::new("pwd").output() },
        "date" => if cfg!(target_os="windows") { Command::new("cmd").args(["/C","date /T"]).output() } else { Command::new("date").output() },
        _ => return Err("Comando bloqueado. Permitidos: pwd, date".into()),
    }.map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default().plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![save_message, history, ask_hermes, open_app, run_local_command])
        .run(tauri::generate_context!()).expect("error while running Tauri application");
}
