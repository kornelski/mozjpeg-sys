#Low-level [MozJPEG](https://github.com/mozilla/mozjpeg) bindings for [Rust](http://www.rust-lang.org/)

See [crates.io](https://crates.io/crates/mozjpeg-sys).

This crate uses libjpeg API, so libjpeg usage manual applies. You'll most likely want to wrap it in a higher-level API :)

```rust
let JPOOL_IMAGE:mozjpeg_sys::c_int = 1;

let mut octets:Vec<u8> = Vec::new();
let width:u32;
let height:u32;

let txt_vec8 = nom_fichier_source.into_bytes();
let txt = CString::new(txt_vec8).unwrap();
let mode = CString::new("rb").unwrap();
let f = libc::fopen(txt.as_ptr(), mode.as_ptr() );

let mut err:jpeg_error_mgr = mem::zeroed();
let mut cinfo: jpeg_decompress_struct = mem::zeroed();
let size:size_t = mem::size_of_val(&cinfo);
cinfo.common.err = mozjpeg_sys::jpeg_std_error(&mut err);
mozjpeg_sys::jpeg_CreateDecompress(&mut cinfo, mozjpeg_sys::JPEG_LIB_VERSION, size);
let f2 = f as *mut mozjpeg_sys::FILE;
mozjpeg_sys::jpeg_stdio_src(&mut cinfo, f2);
mozjpeg_sys::jpeg_read_header(&mut cinfo, true as mozjpeg_sys::boolean);
width = cinfo.image_width;
height = cinfo.image_height;
cinfo.out_color_space = mozjpeg_sys::J_COLOR_SPACE::JCS_EXT_BGRA;
mozjpeg_sys::jpeg_start_decompress(&mut cinfo);
let row_stride = (cinfo.image_width as i32 * cinfo.output_components) as u32;
let nbre_octets = (row_stride * cinfo.image_height) as usize;

octets.reserve(nbre_octets);
octets.set_len(nbre_octets);
let octets_ptr = octets.as_mut_ptr();
let mut octet_index:isize = 0;

let jsamparray = ((*cinfo.common.mem).alloc_sarray.unwrap())(&mut cinfo.common,  JPOOL_IMAGE, row_stride, 1);
while cinfo.output_scanline < cinfo.output_height {
    mozjpeg_sys::jpeg_read_scanlines(&mut cinfo, jsamparray, 1);
    let jsamparay_ptr: *const u8 = *jsamparray;
    for i in 0..row_stride {
        *octets_ptr.offset(octet_index) = *jsamparay_ptr.offset(i as isize);
        octet_index += 1;
    }
};

mozjpeg_sys::jpeg_finish_decompress(&mut cinfo);
mozjpeg_sys::jpeg_destroy_decompress(&mut cinfo);
libc::fclose(f);
```

Writing:

```rust
let filename = String::from("result.jpg");
let compression = 98;

let txt_vec8 = filename.into_bytes();
let txt = CString::new(txt_vec8).unwrap();
let mode = CString::new("wb").unwrap();
let f = libc::fopen(txt.as_ptr(), mode.as_ptr() );
let mut err:jpeg_error_mgr = mem::zeroed();
let mut cinfo: mozjpeg_sys::jpeg_compress_struct = mem::zeroed();
let size:size_t = mem::size_of_val(&cinfo);
cinfo.common.err = mozjpeg_sys::jpeg_std_error(&mut err);
mozjpeg_sys::jpeg_CreateCompress(&mut cinfo, mozjpeg_sys::JPEG_LIB_VERSION, size);
let f2 = f as *mut mozjpeg_sys::FILE;
mozjpeg_sys::jpeg_stdio_dest(&mut cinfo, f2);
cinfo.image_width = width;
cinfo.image_height = height;
cinfo.input_components = 4;
cinfo.in_color_space = mozjpeg_sys::J_COLOR_SPACE::JCS_EXT_BGRA;
mozjpeg_sys::jpeg_set_defaults(&mut cinfo);
let row_stride = (cinfo.image_width as i32 * cinfo.input_components) as u32;
cinfo.dct_method = mozjpeg_sys::J_DCT_METHOD::JDCT_ISLOW;
mozjpeg_sys::jpeg_set_quality(&mut cinfo, compression as mozjpeg_sys::c_int, true as mozjpeg_sys::boolean);
let octets_ptr = octets.as_mut_ptr();
let mut octet_index:isize = 0;
mozjpeg_sys::jpeg_start_compress(&mut cinfo, true as mozjpeg_sys::boolean);

let jsamparray_mut = ((*cinfo.common.mem).alloc_sarray.unwrap())(&mut cinfo.common,  JPOOL_IMAGE, row_stride, 1);
let jsamparray = jsamparray_mut as mozjpeg_sys::JSAMPARRAY;
while cinfo.next_scanline < cinfo.image_height {
    let jsamparay_ptr: *mut u8 = *jsamparray_mut;
    for i in 0..row_stride {
        *jsamparay_ptr.offset(i as isize) = *octets_ptr.offset(octet_index);
        octet_index += 1;
    }
    mozjpeg_sys::jpeg_write_scanlines(&mut cinfo, jsamparray, 1);
}
mozjpeg_sys::jpeg_finish_compress(&mut cinfo);
mozjpeg_sys::jpeg_destroy_compress(&mut cinfo);
libc::fclose(f);
```
