use crate::MarkerData::Scan;
use std::ffi::CString;
use cloudflare_soos::jpeg::*;
use mozjpeg_sys::*;
use std::mem;

fn decode_rgb_file(file_name: &str) -> (Vec<u8>, u32, u32) {
    unsafe {
        let mut err: jpeg_error_mgr = mem::zeroed();
        let mut cinfo: jpeg_decompress_struct = mem::zeroed();
        cinfo.common.err = jpeg_std_error(&mut err);
        jpeg_create_decompress(&mut cinfo);

        let file_name = CString::new(file_name.as_bytes()).unwrap();
        let mode = CString::new("rb").unwrap();
        let fh = libc::fopen(file_name.as_ptr(), mode.as_ptr());
        jpeg_stdio_src(&mut cinfo, fh);
        let res = decode_rgb_cinfo(&mut cinfo);
        libc::fclose(fh);
        res
    }
}

fn decode_rgb_cinfo(cinfo: &mut jpeg_decompress_struct) -> (Vec<u8>, u32, u32) {
    unsafe {
        jpeg_read_header(cinfo, true as boolean);

        let width = cinfo.image_width;
        let height = cinfo.image_height;

        cinfo.out_color_space = J_COLOR_SPACE::JCS_RGB;
        jpeg_start_decompress(cinfo);
        let row_stride = cinfo.image_width as usize * cinfo.output_components as usize;
        let buffer_size = row_stride * cinfo.image_height as usize;
        let mut buffer = vec![0u8; buffer_size];

        while cinfo.output_scanline < cinfo.output_height {
            let offset = cinfo.output_scanline as usize * row_stride;
            let mut jsamparray = [buffer[offset..].as_mut_ptr()];
            jpeg_read_scanlines(cinfo, jsamparray.as_mut_ptr(), 1);
        }

        jpeg_finish_decompress(cinfo);
        jpeg_destroy_decompress(cinfo);

        (buffer, width, height)
    }
}

fn encode_rgb(buffer: &[u8], width: u32, height: u32, quality: i32) -> Vec<u8> {
    unsafe {
        let mut err = mem::zeroed();
        let mut cinfo: jpeg_compress_struct = mem::zeroed();
        cinfo.common.err = jpeg_std_error(&mut err);
        jpeg_create_compress(&mut cinfo);
        let mut buf = std::ptr::null_mut();
        let mut bufsize = 0;
        jpeg_mem_dest(&mut cinfo, &mut buf, &mut bufsize);

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

        assert!(bufsize > 177);
        assert!(!buf.is_null());
        std::slice::from_raw_parts(buf, bufsize as usize).to_vec()
    }
}

#[test]
fn no_green_faces() {
    let (buffer, width, height) = decode_rgb_file("tests/test.jpg");
    let enc = encode_rgb(&buffer, width, height, 60);
    assert!(enc.len() > 20000);
    let m = Decoder::new(&mut enc.as_slice()).decode().unwrap();
    let f = m.into_iter().map(|m| m.marker)
        .find_map(|m| match m { Scan(f) => Some(f), _ => None})
        .unwrap();
    assert_eq!(3, f.component_indices.len());
    assert_eq!(3, f.dc_table_indices.len());
}
