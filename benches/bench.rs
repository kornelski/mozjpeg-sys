#![feature(test)]
extern crate test;

use mozjpeg_sys::*;
use std::mem;
use test::Bencher;

#[bench]
pub fn decompress(bencher: &mut Bencher) {
    let w = 1920;
    let h = 1080;
    let img = random_pixels(w, h);
    let jpeg = unsafe { encode(&img, w, h) };
    bencher.iter(|| {
        unsafe { decode(&jpeg)  }
    });
}

#[bench]
pub fn compress(bencher: &mut Bencher) {
    let w = 640;
    let h = 480;
    let img = random_pixels(w, h);
    bencher.iter(|| {
        unsafe { encode(&img, w, h) }
    });
}

fn random_pixels(w: u32, h: u32) -> Vec<[u8; 3]> {
    (0..h).flat_map(move |h| {
        (0..w).map(move |w| {
            let x = f64::from(w)/13.;
            let y = f64::from(h)/17.;
            let r = ((x.sin() * y.cos() + 1.) * 140.) as u8;
            let g = ((x.cos() * y.sin() + 1.) * 128.) as u8;
            let b = ((x.sin() * y.sin() + 1.) * 100.) as u8;
            [r,g,b]
        })
    }).collect()
}

unsafe fn decode(data: &[u8]) -> (Vec<[u8; 3]>, u32, u32) {
    let mut err: jpeg_error_mgr = mem::zeroed();
    let mut cinfo: jpeg_decompress_struct = mem::zeroed();
    cinfo.common.err = jpeg_std_error(&mut err);
    jpeg_create_decompress(&mut cinfo);


    jpeg_mem_src(&mut cinfo, data.as_ptr(), data.len() as _);
    jpeg_read_header(&mut cinfo, true as boolean);

    let width = cinfo.image_width;
    let height = cinfo.image_height;

    cinfo.out_color_space = J_COLOR_SPACE::JCS_RGB;
    jpeg_start_decompress(&mut cinfo);
    assert_eq!(3, cinfo.output_components);
    let buffer_size = width as usize * height as usize;
    let mut buffer = vec![[0u8; 3]; buffer_size];

    while cinfo.output_scanline < cinfo.output_height {
        let offset = cinfo.output_scanline as usize * width as usize;
        let mut jsamparray = [buffer[offset..].as_mut_ptr()];
        jpeg_read_scanlines(&mut cinfo, jsamparray.as_mut_ptr().cast(), 1);
    }

    jpeg_finish_decompress(&mut cinfo);
    jpeg_destroy_decompress(&mut cinfo);

    (buffer, width, height)
}

unsafe fn encode(buffer: &[[u8; 3]], width: u32, height: u32) -> Vec<u8> {
    let quality = 77;

    let mut err = mem::zeroed();
    let mut cinfo: jpeg_compress_struct = mem::zeroed();
    cinfo.common.err = jpeg_std_error(&mut err);
    jpeg_create_compress(&mut cinfo);
    let mut outbuffer = std::ptr::null_mut();
    let mut outsize = 0;
    jpeg_mem_dest(&mut cinfo, &mut outbuffer, &mut outsize);

    cinfo.image_width = width;
    cinfo.image_height = height;

    cinfo.in_color_space = J_COLOR_SPACE::JCS_RGB;
    cinfo.input_components = 3;
    jpeg_set_defaults(&mut cinfo);

    assert_eq!(3, cinfo.input_components);
    cinfo.dct_method = J_DCT_METHOD::JDCT_ISLOW;
    jpeg_set_quality(&mut cinfo, quality, true as boolean);

    jpeg_start_compress(&mut cinfo, true as boolean);

    while cinfo.next_scanline < height {
        let offset = cinfo.next_scanline as usize * width as usize;
        let jsamparray = [buffer[offset..].as_ptr()];
        jpeg_write_scanlines(&mut cinfo, jsamparray.as_ptr().cast(), 1);
    }

    jpeg_finish_compress(&mut cinfo);
    assert!(!outbuffer.is_null());
    let vec = std::slice::from_raw_parts(outbuffer, outsize as usize).to_vec();
    libc::free(outbuffer.cast());
    jpeg_destroy_compress(&mut cinfo);
    vec
}
