use crate::MarkerData::Scan;
use cloudflare_soos::jpeg::*;
use mozjpeg_sys::*;
use std::ffi::CString;
use std::mem;

fn decode_rgb_data(data: &[u8]) -> (Vec<u8>, u32, u32) {
    unsafe {
        let mut err: jpeg_error_mgr = mem::zeroed();
        let mut cinfo: jpeg_decompress_struct = mem::zeroed();
        cinfo.common.err = jpeg_std_error(&mut err);
        jpeg_create_decompress(&mut cinfo);

        jpeg_mem_src(&mut cinfo, data.as_ptr(), data.len() as _);
        decode_rgb_cinfo(&mut cinfo)
    }
}

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

#[test]
fn roundtrip() {
    let decoded = decode_rgb_data(&std::fs::read("tests/test.jpg").unwrap());
    let enc = encode_subsampled_jpeg(decoded);
    let _ = decode_rgb_data(&enc);
}

fn encode_subsampled_jpeg((data, width, height): (Vec<u8>, u32, u32)) -> Vec<u8> {
    unsafe {
        let mut cinfo: jpeg_compress_struct = mem::zeroed();
        let mut err = mem::zeroed();
        cinfo.common.err = jpeg_std_error(&mut err);

        let s = mem::size_of_val(&cinfo) as usize;
        jpeg_CreateCompress(&mut cinfo, JPEG_LIB_VERSION, s);

        cinfo.in_color_space = JCS_RGB;
        cinfo.input_components = 3 as c_int;
        jpeg_set_defaults(&mut cinfo);

        let mut outsize = 0;
        let mut outbuffer = std::ptr::null_mut();
        jpeg_mem_dest(&mut cinfo, &mut outbuffer, &mut outsize);

        cinfo.image_width = width as JDIMENSION;
        cinfo.image_height = height as JDIMENSION;

        jpeg_set_colorspace(&mut cinfo, JCS_YCbCr);

        let (h, v) = (2, 2);
        (*cinfo.comp_info.add(0)).h_samp_factor = 1;
        (*cinfo.comp_info.add(0)).v_samp_factor = 1;
        (*cinfo.comp_info.add(1)).h_samp_factor = h;
        (*cinfo.comp_info.add(1)).v_samp_factor = v;
        (*cinfo.comp_info.add(2)).h_samp_factor = h;
        (*cinfo.comp_info.add(2)).v_samp_factor = v;

        jpeg_start_compress(&mut cinfo, true as boolean);
        let _ = write_scanlines(&mut cinfo, &data);
        jpeg_finish_compress(&mut cinfo);

        std::slice::from_raw_parts(outbuffer, outsize as _).to_vec()
    }
}

/// Returns true if all lines in `image_src` (not necessarily all lines of the image) were written
pub fn write_scanlines(cinfo: &mut jpeg_compress_struct, image_src: &[u8]) -> bool {
    const MAX_MCU_HEIGHT: usize = 16;

    assert_eq!(0, cinfo.raw_data_in);
    assert!(cinfo.input_components > 0);
    assert!(cinfo.image_width > 0);

    let byte_width = cinfo.image_width as usize * cinfo.input_components as usize;
    for rows in image_src.chunks(MAX_MCU_HEIGHT * byte_width) {
        let mut row_pointers = arrayvec::ArrayVec::<_, MAX_MCU_HEIGHT>::new();
        for row in rows.chunks(byte_width) {
            debug_assert!(row.len() == byte_width);
            row_pointers.push(row.as_ptr());
        }

        let mut rows_left = row_pointers.len() as u32;
        let mut row_pointers = row_pointers.as_ptr();
        while rows_left > 0 {
            unsafe {
                let rows_written = jpeg_write_scanlines(cinfo, row_pointers, rows_left);
                debug_assert!(rows_left >= rows_written);
                if rows_written == 0 {
                    return false;
                }
                rows_left -= rows_written;
                row_pointers = row_pointers.add(rows_written as usize);
            }
        }
    }
    true
}
