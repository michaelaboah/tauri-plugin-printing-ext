use std::fs::{self, File};

use gtk::prelude::{IsA, PrintOperationExt};
use gtk::{PrintOperation, PrintOperationAction, PrintOperationResult, PrintSettings, Window};

#[tauri::command]
pub fn print_dialog(window: tauri::Window, base64: String) {}

#[test]
fn test_gtk_print() {
    let job_path = "gtk-printjob.pdf";
    gtk::init().unwrap();
    std::fs::write(job_path, b"").unwrap();
    let print_ops = PrintOperation::new();

    let ops_result = print_ops
        .run(PrintOperationAction::Preview, None::<&Window>)
        .unwrap();

    match dbg!(ops_result) {
        PrintOperationResult::Error | PrintOperationResult::Cancel => {
            fs::remove_file(job_path).unwrap()
        }
        PrintOperationResult::Apply => todo!(),
        PrintOperationResult::InProgress => todo!(),
        _ => todo!(),
    };
}
