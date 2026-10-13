#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;
use std::env;
use std::process::Command;

fn get_app_dir() -> PathBuf {
    env::current_exe().unwrap().parent().unwrap().to_path_buf()
}

fn get_library_dir() -> PathBuf {
    get_app_dir().join("Biblioteca")
}

#[tauri::command]
fn get_local_library() -> Vec<String> {
    let lib_dir = get_library_dir();
    if !lib_dir.exists() { fs::create_dir_all(&lib_dir).unwrap(); }
    
    let mut books = Vec::new();
    if let Ok(entries) = fs::read_dir(&lib_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if file_name.to_lowercase().ends_with(".epub") || file_name.to_lowercase().ends_with(".pdf") {
                    books.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }
    books
}

#[tauri::command]
fn import_books(source_path: String) -> Result<String, String> {
    let lib_dir = get_library_dir();
    fs::create_dir_all(&lib_dir).map_err(|e| e.to_string())?;
    
    let mut count = 0;
    if let Ok(entries) = fs::read_dir(&source_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if file_name.to_lowercase().ends_with(".epub") || file_name.to_lowercase().ends_with(".pdf") {
                    let dest = lib_dir.join(&file_name);
                    fs::copy(entry.path(), dest).map_err(|e| e.to_string())?;
                    count += 1;
                }
            }
        }
    }
    Ok(format!("{} livros importados com sucesso!", count))
}

// NOVO COMANDO: Deletar o arquivo do disco
#[tauri::command]
fn delete_book(file_path: String) -> Result<String, String> {
    let path = PathBuf::from(&file_path);
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("Erro ao deletar arquivo: {}", e))?;
        Ok("Livro deletado com sucesso!".to_string())
    } else {
        Err("Arquivo não encontrado.".to_string())
    }
}

#[tauri::command]
fn abrir_com_sumatra(file_path: String) -> Result<(), String> {
    let app_dir = get_app_dir();
    let possiveis_caminhos = vec![
        app_dir.join("SumatraPDF").join("SumatraPDF-3.4.6-64.exe"),
        app_dir.join("SumatraPDF").join("SumatraPDF.exe"),
        app_dir.join("SumatraPDF-3.4.6-64.exe"),
        app_dir.join("SumatraPDF.exe"),
    ];
    
    let mut sumatra_exe: Option<PathBuf> = None;
    for caminho in &possiveis_caminhos {
        if caminho.exists() {
            sumatra_exe = Some(caminho.clone());
            break;
        }
    }
    
    match sumatra_exe {
        Some(exe) => {
            Command::new(exe).arg(&file_path).spawn().map_err(|e| format!("Erro ao abrir SumatraPDF: {}", e))?;
            Ok(())
        }
        None => Err("Não foi possível encontrar o SumatraPDF na pasta do aplicativo.".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        // Registramos o novo comando delete_book aqui
        .invoke_handler(tauri::generate_handler![get_local_library, import_books, abrir_com_sumatra, delete_book])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}