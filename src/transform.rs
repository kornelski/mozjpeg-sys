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

extern "C-unwind" {
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

