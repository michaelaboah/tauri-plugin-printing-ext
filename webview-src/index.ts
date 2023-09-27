import { invoke } from '@tauri-apps/api/tauri'
import { jsPDF } from "jspdf"

export async function printDialog(htmlTarget: HTMLElement) {

  const doc = new jsPDF({orientation: "p", unit: "px", format: "letter", hotfixes: ["px_scaling"] });

    doc.html(htmlTarget, {

      callback: (doc) => {
        let base64 = doc.output("datauristring").split(",")[1]
        invoke('plugin:printing-ext|print_dialog', { base64 })
      },

    })

}
