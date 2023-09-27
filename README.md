
# Tauri Plugin: tauri-plugin-printing-ext

<!-- [![Build Status](https://img.shields.io/github/workflow/status/yourusername/tauri-plugin-printing-ext/CI?style=flat-square)](https://github.com/yourusername/tauri-plugin-printing-ext/actions) -->
<!-- [![Latest Version](https://img.shields.io/crates/v/tauri-plugin-printing-ext.svg?style=flat-square)](https://crates.io/crates/tauri-plugin-printing-ext) -->
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

A Tauri plugin for extending printing functionality. This plugin provides a workaround for printing specific HTML on macOS.

## Prerequisites

- Minimum macOS version: 10.15. This is due to the use of PDFKit PDFDocument. Perhaps it can be substituted for NSDocument.


## Example

```html
<script lang="ts">
  import { onMount } from 'svelte';
  import { printDialog } from './path/to/your/module';

  let printableArea: HTMLElement;

  onMount(() => {
    printDialog(printableArea);
  });
</script>

<div bind:this={printableArea}>
  <h1>Hello, world!</h1>
  <p>This is some content that will be printed.</p>
</div>
```

## How it Works

The goal is to get closer to the native print functionality of Safari that is missing by default in the tauri-apps/Wry crate.

1. **Turn HTML into a base64**: Using the jsPDF library, we can pass in any HTMLElement and have that produce a base64 string. Potentially discovering a similar method that is native to the WebKitWebView API. *I'm tired and need to move forward with another project*.
2. **Send to Rust for binary conversion**: Using the base64 crate to generate bytes that will load into the print window.

## Other Goals

Currently, **Silent** printing is the only other specific printing feature that I've come across. It is possible to accomplish this in macOS with a derivative of the current implementation, but for Windows and Linux, I am less sure.

## Contributing

PRs are welcome! 

<!-- ## License -->

<!-- This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. -->

