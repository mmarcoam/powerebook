use std::fs;

// Esta é a função que o JavaScript vai chamar
#[tauri::command]
fn scan_folder(path: String) -> Vec<String> {
    let mut books = Vec::new();
    
    // Tenta ler a pasta
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name().to_string_lossy().to_string();
                
                // Verifica se é um ebook (EPUB ou PDF)
                if file_name.to_lowercase().ends_with(".epub") || file_name.to_lowercase().ends_with(".pdf") {
                    books.push(file_name);
                }
            }
        }
    }
    
    books // Retorna a lista de livros encontrados
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Aqui nós registramos o nosso novo comando
        .invoke_handler(tauri::generate_handler![scan_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}