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
export declare function printDialog(base64: string): Promise<void>;
