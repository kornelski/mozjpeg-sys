# Low-level [MozJPEG](https://github.com/mozilla/mozjpeg) bindings for [Rust](https://www.rust-lang.org/)

See [crates.io](https://crates.io/crates/mozjpeg-sys).

This crate exposes the raw libjpeg API, so [libjpeg usage manual](https://github.com/mozilla/mozjpeg/blob/master/libjpeg.txt) applies. You'll most likely want to wrap it in a higher-level API :)

Many fields in structs are marked as private by default, but if you need to access them, make a pull request marking them `pub`.

## Requirements

* nasm
* build-essentials (gcc, etc.)

## Usage

In Rust add "mozjpeg-sys" as a dependency and use with `extern crate mozjpeg_sys`.

For non-Rust projects you can build the library using `cargo build --release`. It creates `target/release/libmozjpeg_sys.a` and `target/release/libmozjpeg_sys.{dll,so,dylib}`, which can be linked with C and other languages. In case you need the `jpeglib.h`header for C code built with Cargo, required include path**s** (use `env::split_paths()`) are set for Cargo [build scripts](http://doc.crates.io/environment-variables.html#environment-variables-cargo-sets-for-build-scripts) in the `DEP_JPEG_INCLUDE` env var.

By default `nasm_simd` feature is enabled, and this crate will try to compile SIMD support. Additionally, you can set `TARGET_CPU` environmental variable (equivalent to `-march=$TARGET_CPU`) to optimize all of C code for a specific CPU model.

### [Example](examples/reencode.rs)

```rust
let mut err: jpeg_error_mgr = mem::zeroed();
let mut cinfo: jpeg_decompress_struct = mem::zeroed();
cinfo.common.err = jpeg_std_error(&mut err);
jpeg_create_decompress(&mut cinfo);

let file_name = CString::new(file_name.as_bytes()).unwrap();
let mode = CString::new("rb").unwrap();
let fh = libc::fopen(file_name.as_ptr(), mode.as_ptr());
jpeg_stdio_src(&mut cinfo, fh);
jpeg_read_header(&mut cinfo, true as boolean);

// Available only after `jpeg_read_header()`
let width = cinfo.image_width;
let height = cinfo.image_height;

// Output settings be set before calling `jpeg_start_decompress()`
cinfo.out_color_space = J_COLOR_SPACE::JCS_RGB;
jpeg_start_decompress(&mut cinfo);
let row_stride = cinfo.image_width as usize * cinfo.output_components as usize;
let buffer_size = row_stride * cinfo.image_height as usize;
let mut buffer = vec![0u8; buffer_size];

while cinfo.output_scanline < cinfo.output_height {
    let offset = cinfo.output_scanline as usize * row_stride;
    let mut jsamparray = [buffer[offset..].as_mut_ptr()];
    jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr(), 1);
}

jpeg_finish_decompress(&mut cinfo);
jpeg_destroy_decompress(&mut cinfo);
libc::fclose(fh);
```

Writing:

```rust
let quality = 98;
let file_name = CString::new("example_result.jpg").unwrap();
let mode = CString::new("wb").unwrap();
let fh = libc::fopen(file_name.as_ptr(), mode.as_ptr());

let mut err = mem::zeroed();
let mut cinfo: jpeg_compress_struct = mem::zeroed();
cinfo.common.err = jpeg_std_error(&mut err);
jpeg_create_compress(&mut cinfo);
jpeg_stdio_dest(&mut cinfo, fh);

cinfo.image_width = width;
cinfo.image_height = height;
cinfo.in_color_space = J_COLOR_SPACE::JCS_RGB;
cinfo.input_components = 3;
jpeg_set_defaults(&mut cinfo);

let row_stride = cinfo.image_width as usize * cinfo.input_components as usize;
cinfo.dct_method = J_DCT_METHOD::JDCT_ISLOW;
jpeg_set_quality(&mut cinfo, quality, true as boolean);

jpeg_start_compress(&mut cinfo, true as boolean);

while cinfo.next_scanline < cinfo.image_height {
    let offset = cinfo.next_scanline as usize * row_stride;
    let jsamparray = [buffer[offset..].as_ptr()];
    jpeg_write_scanlines(&mut cinfo, jsamparray.as_ptr(), 1);
}

jpeg_finish_compress(&mut cinfo);
jpeg_destroy_compress(&mut cinfo);
libc::fclose(fh);
```
