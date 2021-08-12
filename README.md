# wazip

[![build](https://github.com/nasso/wazip/actions/workflows/rust.yml/badge.svg)](https://github.com/nasso/wazip/actions/workflows/rust.yml)

`wazip` is a ZIP file manipulation library for JavaScript powered by WebAssembly
and written in Rust.

## Usage

```ts
import wazip_init, { ZipWriter } from "wazip";
import { saveAs } from "file-saver";

async function main() {
  // Init wazip by calling its init function (the default export)
  await wazip_init();

  // Create a new ZipWriter to create a new archive
  let writer = new ZipWriter();

  // You can optionally set the ZIP comment for the archive
  writer.set_comment("a nice message!");

  // Add a new file with `start_file`
  writer.start_file("foo.txt");

  // Write data to the current file (foo.txt)
  // ZipWriter.write() takes an array or typed-array. strings must be encoded
  writer.write(new TextEncoder().encode("hello world!"));

  // When you're done, call writer.finish() and you get a Uint8Array (the ZIP)
  let zip_data = writer.finish();

  // This example uses file-saver to download the generated ZIP file
  saveAs(new Blob([zip_data], { type = "application/zip" }), "bar.zip");
}

main();
```

## Features

| Feature                        | Status                       |
| ------------------------------ | ---------------------------- |
| Create an archive from scratch | :white_check_mark: Supported |
| Read an existing archive       | :construction: Planned       |
| Modify an archive              | :construction: Planned       |

Currently only DEFLATE is supported, but support for uncompressed storage is
planned. Other compression methods might be added if they can be ported to WASM.

## License

`wazip` is licensed under the terms of both the MIT license and the Apache
License (Version 2.0), at your choice.

See LICENSE-MIT and LICENSE-APACHE-2.0 files for the full texts.
