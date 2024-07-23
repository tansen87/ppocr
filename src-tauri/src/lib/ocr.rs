use std::{
  error::Error,
  path::{Path, PathBuf},
};
use tauri::Window;

fn image_ocr(ocr_path: &str, window: Window) -> Result<String, Box<dyn Error>> {
  let start = std::time::Instant::now();
  let mut p = paddleocr::Ppocr::new(
    PathBuf::from("PaddleOCR-json/PaddleOCR-json.exe"),
    Default::default(),
  )?;

  let json_data = p.ocr(Path::new(ocr_path).into())?;

  let end = std::time::Instant::now();
  let elapsed = end - start;
  let elapsed_str = format!("{:.2}", elapsed.as_secs_f64());
  window.emit("time", elapsed_str)?;

  Ok(json_data)
}

fn screen_ocr(window: Window) -> Result<String, Box<dyn Error>> {
  let start = std::time::Instant::now();
  let mut p = paddleocr::Ppocr::new(
    PathBuf::from("PaddleOCR-json/PaddleOCR-json.exe"),
    Default::default(),
  )?;

  let json_data = p.ocr_clipboard()?;

  let end = std::time::Instant::now();
  let elapsed = end - start;
  let elapsed_str = format!("{:.2}", elapsed.as_secs_f64());
  window.emit("ps_time", elapsed_str)?;

  Ok(json_data)
}

#[tauri::command]
pub async fn image(file: String, window: Window) -> String {
  let ocr_results = match (async { image_ocr(&file, window) }).await {
    Ok(res) => res,
    Err(err) => {
      eprintln!("ocr error: {err}");
      err.to_string()
    }
  };

  ocr_results
}

#[tauri::command]
pub async fn screen(window: Window) -> String {
  let ocr_results = match (async { screen_ocr(window) }).await {
    Ok(res) => res,
    Err(err) => {
      eprintln!("ocr error: {err}");
      err.to_string()
    }
  };

  ocr_results
}
