use std::{process::Command, thread};

use tauri::{AppHandle, Emitter, Manager};
#[tauri::command]
async fn steamandothers(app: AppHandle) {
    let thread = thread::spawn(move || {
        let cmd = Command::new("sh")
            .args([
                "-c",
                "paru -S steamtinkerlaunch protonup-qt --sudo pkexec --noconfirm",
            ])
            .status()
            .unwrap();
        if cmd.code().unwrap() != 0 {
            app.get_webview_window("main")
                .unwrap()
                .emit("failed_install", ())
                .unwrap();
        }
        let cmd = Command::new("pkexec")
            .args(["pacman", "-S", "steam", "--noconfirm"])
            .status()
            .unwrap();
        if cmd.code().unwrap() != 0 {
            app.get_webview_window("main")
                .unwrap()
                .emit("failed_install", ())
                .unwrap();
        }
    });
    thread.join().unwrap();
}
#[tauri::command]
async fn update_flatpaks(app: AppHandle) {
    let thread = thread::spawn(move || {
        let cmd = Command::new("sh")
            .args(["-c", format!("flatpak update --assumeyes").as_str()])
            .status()
            .unwrap();
        if cmd.code().unwrap() != 0 {
            app.get_webview_window("main")
                .unwrap()
                .emit("failed_install", ())
                .unwrap();
        }
    });
    thread.join().unwrap();
}
#[tauri::command]
fn backend_msg(message: String) {
    println!("{}", message);
}
#[tauri::command]
async fn install_app(application: String, app: AppHandle) {
    let thread = thread::spawn(move || {
        let cmd = Command::new("sh")
            .args([
                "-c",
                format!("flatpak install {} -y --user", &application).as_str(),
            ])
            .status()
            .unwrap();
        if cmd.code().unwrap() != 0 {
            app.get_webview_window("main")
                .unwrap()
                .emit("failed_install", ())
                .unwrap();
        }
    });
    thread.join().unwrap();
}
#[tauri::command]
async fn get_apps(app: AppHandle) -> Vec<String> {
    let cmd = match Command::new("sh")
        .args(["-c", "flatpak remote-ls flathub -u --app | sort"])
        .output()
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{e}");
            app.get_webview_window("main")
                .unwrap()
                .emit("getapps_err", e.to_string())
                .unwrap();
            return vec![];
        }
    };
    if cmd.status.code().unwrap() != 0 {
        app.get_webview_window("main")
            .unwrap()
            .emit("getapps_err", "Generic backend error")
            .unwrap();
    }
    let mut returnvec: Vec<String> = Vec::new();
    for app in String::from_utf8(cmd.stdout).unwrap().split("\n") {
        if app.replace(" ", "") != "" {
            let app_vec = app.split("\t").collect::<Vec<&str>>();
            let app_name = app_vec[0];
            let app_id = app_vec[1];
            returnvec.push(format!("{app_name}|{app_id}"));
        }
    }
    returnvec
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_apps,
            install_app,
            backend_msg,
            update_flatpaks,
            steamandothers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
