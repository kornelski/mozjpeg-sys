
use libc;
use mozjpeg_sys::*;
use std::mem;
use std::ffi::CString;

fn main() {
    let file_name = std::env::args().skip(1).next().expect("Specify a JPEG image path");

    unsafe {
        let (buffer, width, height) = decode(&file_name);
        encode(&buffer, width, height);
    }
}

unsafe fn decode(file_name: &str) -> (Vec<u8>, u32, u32) {
    let mut err: jpeg_error_mgr = mem::zeroed();
    let mut cinfo: jpeg_decompress_struct = mem::zeroed();
    cinfo.common.err = jpeg_std_error(&mut err);
    jpeg_create_decompress(&mut cinfo);

    let file_name = CString::new(file_name.as_bytes()).unwrap();
    let mode = CString::new("rb").unwrap();
    let fh = libc::fopen(file_name.as_ptr(), mode.as_ptr());
    jpeg_stdio_src(&mut cinfo, fh);
    jpeg_read_header(&mut cinfo, true as boolean);

    let width = cinfo.image_width;
    let height = cinfo.image_height;
    println!("Image size {}x{}", width, height);

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

    println!("Decoded into {} raw pixel bytes", buffer.len());

    jpeg_finish_decompress(&mut cinfo);
    jpeg_destroy_decompress(&mut cinfo);
    libc::fclose(fh);

    (buffer, width, height)
}

unsafe fn encode(buffer: &[u8], width: u32, height: u32) {
    println!("Writing example_result.jpg");
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
}
