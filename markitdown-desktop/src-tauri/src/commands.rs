use crate::models::{BatchResult, ConversionData, ConversionResult, HistoryEntry, IpcResponse};
use crate::AppState;
use sqlx::Row;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_shell::ShellExt;

#[tauri::command]
pub async fn convert_file(
    app: AppHandle,
    state: State<'_, AppState>,
    file_path: String,
) -> Result<IpcResponse<ConversionData>, String> {
    let script_path = get_python_script_path(&app)?;

    let output = app
        .shell()
        .command(get_python_command())
        .args([&script_path, "convert", &file_path])
        .output()
        .await
        .map_err(|e| format!("Failed to spawn sidecar: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Ok(IpcResponse::err(format!("Sidecar error: {}", stderr)));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let result: ConversionResult = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse sidecar output: {}", e))?;

    if result.success {
        if let Some(data) = result.data {
            save_to_history(&state, &data).await?;
            Ok(IpcResponse::ok(data))
        } else {
            Ok(IpcResponse::err("No data returned from sidecar"))
        }
    } else {
        Ok(IpcResponse::err(result.error.unwrap_or_default()))
    }
}

#[tauri::command]
pub async fn batch_convert(
    app: AppHandle,
    state: State<'_, AppState>,
    file_paths: Vec<String>,
) -> Result<IpcResponse<BatchResult>, String> {
    let script_path = get_python_script_path(&app)?;

    let mut args = vec![script_path, "batch".to_string()];
    args.extend(file_paths.clone());

    let output = app
        .shell()
        .command(get_python_command())
        .args(&args)
        .output()
        .await
        .map_err(|e| format!("Failed to spawn sidecar: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Ok(IpcResponse::err(format!("Sidecar error: {}", stderr)));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let result: BatchResult = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse sidecar output: {}", e))?;

    if result.success {
        if let Some(ref data) = result.data {
            for item in &data.results {
                if item.success {
                    if let Some(ref conv_data) = item.data {
                        save_to_history(&state, conv_data).await.ok();
                    }
                }
            }
        }
        Ok(IpcResponse::ok(result))
    } else {
        Ok(IpcResponse::err(result.error.unwrap_or_default()))
    }
}

#[tauri::command]
pub async fn get_history(
    state: State<'_, AppState>,
) -> Result<IpcResponse<Vec<HistoryEntry>>, String> {
    let pool = state.db.lock().map_err(|e| e.to_string())?;
    let rows = sqlx::query(
        "SELECT id, filename, source_path, output_path, status, error_message, markdown_content, image_paths, file_size, created_at FROM conversions ORDER BY created_at DESC"
    )
    .fetch_all(&*pool)
    .await
    .map_err(|e| format!("DB query failed: {}", e))?;

    let entries: Vec<HistoryEntry> = rows
        .iter()
        .map(|row| HistoryEntry {
            id: row.get("id"),
            filename: row.get("filename"),
            source_path: row.get("source_path"),
            output_path: row.get("output_path"),
            status: row.get("status"),
            error_message: row.get("error_message"),
            markdown_content: row.get("markdown_content"),
            image_paths: row.get("image_paths"),
            file_size: row.get("file_size"),
            created_at: row.get("created_at"),
        })
        .collect();

    Ok(IpcResponse::ok(entries))
}

#[tauri::command]
pub async fn delete_history(
    state: State<'_, AppState>,
    id: i64,
) -> Result<IpcResponse<()>, String> {
    let pool = state.db.lock().map_err(|e| e.to_string())?;
    sqlx::query("DELETE FROM conversions WHERE id = ?1")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| format!("DB delete failed: {}", e))?;

    Ok(IpcResponse::ok(()))
}

#[tauri::command]
pub async fn open_file_dialog(app: AppHandle) -> Result<IpcResponse<Vec<String>>, String> {
    use tauri_plugin_dialog::DialogExt;

    let file_path = app
        .dialog()
        .file()
        .add_filter("Documents", &["pdf", "docx", "xlsx", "pptx", "html", "htm"])
        .blocking_pick_files();

    match file_path {
        Some(paths) => {
            let paths: Vec<String> = paths.iter().map(|p| p.to_string()).collect();
            Ok(IpcResponse::ok(paths))
        }
        None => Ok(IpcResponse::ok(vec![])),
    }
}

#[tauri::command]
pub async fn open_folder_dialog(app: AppHandle) -> Result<IpcResponse<Option<String>>, String> {
    use tauri_plugin_dialog::DialogExt;

    let folder_path = app.dialog().file().blocking_pick_folder();

    match folder_path {
        Some(path) => Ok(IpcResponse::ok(Some(path.to_string()))),
        None => Ok(IpcResponse::ok(None)),
    }
}

/// Returns the absolute path to the venv Python interpreter.
/// Tries python-core/.venv first, falls back to system python.
fn get_python_command(app: &AppHandle) -> String {
    let resource_dir = app
        .path()
        .resource_dir()
        .unwrap_or_default();

    let project_root = resource_dir
        .join("..")
        .join("..");

    let venv_python = if cfg!(target_os = "windows") {
        project_root
            .join("python-core")
            .join(".venv")
            .join("Scripts")
            .join("python.exe")
    } else {
        project_root
            .join("python-core")
            .join(".venv")
            .join("bin")
            .join("python")
    };

    if venv_python.exists() {
        return venv_python.to_str().unwrap_or("python").to_string();
    }

    // Fallback: system python
    if cfg!(target_os = "windows") {
        "python".to_string()
    } else {
        "python3".to_string()
    }
}

fn get_python_script_path(app: &AppHandle) -> Result<String, String> {
    // In dev mode, use the python-core/ directory relative to the project root
    // In production, it would be bundled with the app
    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| format!("Failed to get resource dir: {}", e))?;

    // Dev mode: resource_dir is src-tauri/target/debug/
    // Production: resource_dir is alongside the binary
    let script_path = resource_dir
        .join("..")
        .join("..")
        .join("python-core")
        .join("convert.py");

    if script_path.exists() {
        return Ok(script_path
            .to_str()
            .ok_or("Invalid script path".to_string())?
            .to_string());
    }

    // Fallback: try the current working directory
    let cwd_script = std::env::current_dir()
        .map_err(|e| format!("Failed to get cwd: {}", e))?
        .join("python-core")
        .join("convert.py");

    if cwd_script.exists() {
        return Ok(cwd_script
            .to_str()
            .ok_or("Invalid script path".to_string())?
            .to_string());
    }

    Err(format!(
        "Could not find python-core/convert.py (tried {:?} and {:?})",
        script_path, cwd_script
    ))
}

async fn save_to_history(state: &AppState, data: &ConversionData) -> Result<(), String> {
    let pool = state.db.lock().map_err(|e| e.to_string())?;

    let image_paths_json = data
        .image_paths
        .as_ref()
        .map(|p| serde_json::to_string(p).unwrap_or_default())
        .unwrap_or_default();

    let file_size_val = data.file_size;

    sqlx::query(
        "INSERT INTO conversions (filename, source_path, output_path, status, markdown_content, image_paths, file_size) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)"
    )
    .bind(&data.filename)
    .bind(&data.source_path)
    .bind(&data.output_path)
    .bind(&data.status)
    .bind(&data.markdown_content)
    .bind(&image_paths_json)
    .bind(file_size_val)
    .execute(&*pool)
    .await
    .map_err(|e| format!("DB insert failed: {}", e))?;

    Ok(())
}
