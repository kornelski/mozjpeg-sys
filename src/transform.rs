pub type j_common_ptr = *mut jpeg_common_struct;
pub type j_compress_ptr = *mut jpeg_compress_struct;
pub type j_decompress_ptr = *mut jpeg_decompress_struct;
pub type jvirt_barray_ptr = *mut jvirt_barray_control;

pub const JXFORM_CODE_JXFORM_NONE: JXFORM_CODE = 0;
pub const JXFORM_CODE_JXFORM_FLIP_H: JXFORM_CODE = 1;
pub const JXFORM_CODE_JXFORM_FLIP_V: JXFORM_CODE = 2;
pub const JXFORM_CODE_JXFORM_TRANSPOSE: JXFORM_CODE = 3;
pub const JXFORM_CODE_JXFORM_TRANSVERSE: JXFORM_CODE = 4;
pub const JXFORM_CODE_JXFORM_ROT_90: JXFORM_CODE = 5;
pub const JXFORM_CODE_JXFORM_ROT_180: JXFORM_CODE = 6;
pub const JXFORM_CODE_JXFORM_ROT_270: JXFORM_CODE = 7;
pub type JXFORM_CODE = ::std::os::raw::c_uint;
pub const JCROP_CODE_JCROP_UNSET: JCROP_CODE = 0;
pub const JCROP_CODE_JCROP_POS: JCROP_CODE = 1;
pub const JCROP_CODE_JCROP_NEG: JCROP_CODE = 2;
pub const JCROP_CODE_JCROP_FORCE: JCROP_CODE = 3;
pub type JCROP_CODE = ::std::os::raw::c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jpeg_transform_info {
    pub transform: JXFORM_CODE,
    pub perfect: boolean,
    pub trim: boolean,
    pub force_grayscale: boolean,
    pub crop: boolean,
    pub slow_hflip: boolean,
    pub crop_width: JDIMENSION,
    pub crop_width_set: JCROP_CODE,
    pub crop_height: JDIMENSION,
    pub crop_height_set: JCROP_CODE,
    pub crop_xoffset: JDIMENSION,
    pub crop_xoffset_set: JCROP_CODE,
    pub crop_yoffset: JDIMENSION,
    pub crop_yoffset_set: JCROP_CODE,
    pub num_components: ::std::os::raw::c_int,
    pub workspace_coef_arrays: *mut jvirt_barray_ptr,
    pub output_width: JDIMENSION,
    pub output_height: JDIMENSION,
    pub x_crop_offset: JDIMENSION,
    pub y_crop_offset: JDIMENSION,
    pub iMCU_sample_width: ::std::os::raw::c_int,
    pub iMCU_sample_height: ::std::os::raw::c_int,
}

pub unsafe fn jtransform_execute_transformation(
    srcinfo: j_decompress_ptr,
    dstinfo: j_compress_ptr,
    src_coef_arrays: *mut jvirt_barray_ptr,
    info: *mut jpeg_transform_info,
) {
    jtransform_execute_transform(srcinfo, dstinfo, src_coef_arrays, info)
}

extern "C" {
    pub fn jtransform_adjust_parameters(
        srcinfo: j_decompress_ptr,
        dstinfo: j_compress_ptr,
        src_coef_arrays: *mut jvirt_barray_ptr,
        info: *mut jpeg_transform_info,
    ) -> *mut jvirt_barray_ptr;

    pub fn jtransform_execute_transform(
        srcinfo: j_decompress_ptr,
        dstinfo: j_compress_ptr,
        src_coef_arrays: *mut jvirt_barray_ptr,
        info: *mut jpeg_transform_info,
    );

    pub fn jtransform_request_workspace(
        srcinfo: j_decompress_ptr,
        info: *mut jpeg_transform_info,
    ) -> boolean;
}

#[test]
#[cfg(all(target_pointer_width="64", feature="jpegtran"))]
fn bindgen_test_layout_jpeg_transform_info() {
    assert_eq!(
        ::std::mem::size_of::<jpeg_transform_info>(),
        96usize,
        concat!("Size of: ", stringify!(jpeg_transform_info))
    );
    assert_eq!(
        ::std::mem::align_of::<jpeg_transform_info>(),
        8usize,
        concat!("Alignment of ", stringify!(jpeg_transform_info))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jpeg_transform_info>())).transform as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(transform)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jpeg_transform_info>())).perfect as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(perfect)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jpeg_transform_info>())).trim as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(trim)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).force_grayscale as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(force_grayscale)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jpeg_transform_info>())).crop as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(crop)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jpeg_transform_info>())).slow_hflip as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(slow_hflip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jpeg_transform_info>())).crop_width as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(crop_width)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).crop_width_set as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(crop_width_set)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<jpeg_transform_info>())).crop_height as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(crop_height)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).crop_height_set as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(crop_height_set)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).crop_xoffset as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(crop_xoffset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).crop_xoffset_set as *const _ as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(crop_xoffset_set)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).crop_yoffset as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(crop_yoffset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).crop_yoffset_set as *const _ as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(crop_yoffset_set)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).num_components as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(num_components)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).workspace_coef_arrays as *const _
                as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(workspace_coef_arrays)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).output_width as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(output_width)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).output_height as *const _ as usize
        },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(output_height)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).x_crop_offset as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(x_crop_offset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).y_crop_offset as *const _ as usize
        },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(y_crop_offset)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).iMCU_sample_width as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(iMCU_sample_width)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<jpeg_transform_info>())).iMCU_sample_height as *const _ as usize
        },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(jpeg_transform_info),
            "::",
            stringify!(iMCU_sample_height)
        )
    );
}
