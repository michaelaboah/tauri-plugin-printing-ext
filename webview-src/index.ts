import { invoke } from '@tauri-apps/api/tauri'
/**
 * Opens a print dialog for a PDF with base64 data prodivded.
 * This function is specific to Darwin or macOS platforms.
 * 
 * Note: *jsPDF to transform html into base64 pdf* 
 *
 * @param {string} base64 - transformed pdf-data
 * @returns {void} If the platform is not Darwin or macOS, logs an error and returns.
 *
 * @example
 * printDialog(document.getElementById('printableArea'));
 */
export async function printDialog(base64: string) {
 
  if (!navigator.userAgent.includes("Mac")) {
    return console.error("Darwin or macOs specific operation")
  }

  invoke('plugin:printing-ext|print_dialog', { base64 })
}


