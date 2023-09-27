#[cfg(target_os = "macos")]
use base64::{engine::general_purpose, Engine as _};

#[cfg(target_os = "macos")]
use cocoa::base::{id,  BOOL, NO, YES};
#[cfg(target_os = "macos")]
use objc::{
    class,
    msg_send,
    sel, sel_impl,
};


#[cfg(target_os = "macos")]
#[tauri::command]
pub fn print_dialog(window: tauri::Window, base64: String) {

    let _ = window.with_webview(move |view| {

        let bytes: &[u8] = &general_purpose::STANDARD.decode(&base64).unwrap();
        let can_print: BOOL = unsafe {
            msg_send![
              view.inner(),
              respondsToSelector: sel!(printOperationWithPrintInfo:)
            ]
        };

        if can_print != YES {
            return ();
        }
        
        unsafe {
            // https://developer.apple.com/documentation/foundation/nsdata?language=objc
            let data: id = msg_send![class!(NSData), alloc];
            let data: id = msg_send![data, initWithBytes: bytes.as_ptr() length: bytes.len()];

            // https://developer.apple.com/documentation/pdfkit/pdfdocument?language=objc
            let doc: id = msg_send![class!(PDFDocument), alloc];
            let doc: id = msg_send![doc, initWithData: data];


            let print_info: id = msg_send![class!(NSPrintInfo), sharedPrintInfo];
            
            // https://developer.apple.com/documentation/pdfkit/pdfdocument/1436075-printoperationforprintinfo?language=objc
            let print_ops: id =
                msg_send![doc, printOperationForPrintInfo: print_info scalingMode: 2 autoRotate: NO];

            let () = msg_send![print_ops, runOperation];
        }
    });
}
