// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::BufReader;
use pdf::file::File as PdfFile;
use pdf::error::PdfError;
use pdf::objects::Dictionary;

#[tauri::command]
fn split_pdf(_source_file: &str, _destination: &str) -> Result<PdfError>{
    //open the source file
    let file = File::open(_source_file);
    let file_reader =BufReader::new(file);
    let pdf_file = PdfFile::open(file_reader);

    for (page_number, page) in pdf_file.pages.iter().enumerate() {

    }

    return Result::Ok(());
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![split_pdf])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
