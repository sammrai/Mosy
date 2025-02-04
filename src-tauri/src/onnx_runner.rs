use onnxruntime::{environment::Environment, session::Session};

#[tauri::command]
pub fn run_inference(model_path: String, input_data: Vec<f32>) -> Result<Vec<f32>, String> {
    let environment = Environment::builder()
        .build()
        .map_err(|e| e.to_string())?;
    let session = Session::builder(&environment)
        .with_model_from_file(model_path)
        .map_err(|e| e.to_string())?;
    
    // TODO: 入出力テンソルのセットアップと推論の実行
    Ok(vec![]) // 実際の出力に合わせて実装してください
}
