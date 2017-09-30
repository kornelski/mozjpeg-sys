#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate libc;

pub use self::libc::FILE;
pub use std::os::raw::{c_int, c_uint, c_void, c_long, c_ulong};
use std::mem;
use std::default::Default;

pub use J_COLOR_SPACE::*;
pub use J_BOOLEAN_PARAM::*;
pub use J_FLOAT_PARAM::*;
pub use J_INT_PARAM::*;
pub use JINT_COMPRESS_PROFILE_VALUE::*;
pub use J_DCT_METHOD::JDCT_ISLOW as JDCT_DEFAULT;
pub use J_DCT_METHOD::JDCT_IFAST as JDCT_FASTEST;

pub const JPEG_LIB_VERSION: c_int = 62;
/// The basic DCT block is 8x8 samples
pub const DCTSIZE: usize = 8;
/// DCTSIZE²
pub const DCTSIZE2: usize = DCTSIZE*DCTSIZE;

pub type boolean = c_int;
pub type JSAMPLE = u8;
pub type JCOEF = i16;
pub type JDIMENSION = c_uint;
/// ptr to one image row of pixel samples.
pub type JSAMPROW = *const JSAMPLE;
pub type JSAMPROW_MUT = *mut JSAMPLE;
/// ptr to some rows (a 2-D sample array)
pub type JSAMPARRAY = *const JSAMPROW;
pub type JSAMPARRAY_MUT = *mut JSAMPROW_MUT;
/// a 3-D sample array: top index is color
pub type JSAMPIMAGE = *const JSAMPARRAY;
pub type JSAMPIMAGE_MUT = *mut JSAMPARRAY_MUT;
/// one block of coefficients
pub type JBLOCK = [JCOEF; 64usize];
/// pointer to one row of coefficient blocks
pub type JBLOCKROW = *mut JBLOCK;
pub type JBLOCKARRAY = *mut JBLOCKROW;

// must match dct.h; assumes bits in sample == 8
/// type for individual integer DCT coefficient
#[cfg(feature = "nasm_simd")]
pub type DCTELEM = i16;
#[cfg(not(feature = "nasm_simd"))]
pub type DCTELEM = c_int;


#[repr(C)]
pub struct JQUANT_TBL {
    /// This array gives the coefficient quantizers in natural array order
    /// (not the zigzag order in which they are stored in a JPEG DQT marker).
    /// CAUTION: IJG versions prior to v6a kept this array in zigzag order.
    pub quantval: [u16; 64usize],
    sent_table: boolean,
}
impl Default for JQUANT_TBL {
    fn default() -> JQUANT_TBL { unsafe { mem::zeroed() } }
}

#[repr(C)]
pub struct JHUFF_TBL {
    pub bits: [u8; 17usize],
    pub huffval: [u8; 256usize],
    sent_table: boolean,
}
impl Default for JHUFF_TBL {
    fn default() -> JHUFF_TBL { unsafe { mem::zeroed() } }
}

#[repr(C)]
pub struct jpeg_component_info {
    /// identifier for this component (0..255)
    pub component_id: c_int,
    /// its index in SOF or cinfo->comp_info[]
    pub component_index: c_int,
    /// horizontal sampling factor (1..4)
    pub h_samp_factor: c_int,
    /// vertical sampling factor (1..4)
    pub v_samp_factor: c_int,
    /// quantization table selector (0..3)
    pub quant_tbl_no: c_int,
    /// DC entropy table selector (0..3)
    ///
    /// These values may vary between scans.
    /// For compression, they must be supplied by parameter setup;
    /// for decompression, they are read from the SOS marker.
    /// The decompressor output side may not use these variables.
    pub dc_tbl_no: c_int,
    /// AC entropy table selector (0..3)
    pub ac_tbl_no: c_int,
    pub width_in_blocks: JDIMENSION,
    pub height_in_blocks: JDIMENSION,
    /// Remaining fields should be treated as private by applications.
    DCT_scaled_size: c_int,
    downsampled_width: JDIMENSION,
    downsampled_height: JDIMENSION,
    component_needed: boolean,
    MCU_width: c_int,
    MCU_height: c_int,
    MCU_blocks: c_int,
    MCU_sample_width: c_int,
    last_col_width: c_int,
    last_row_height: c_int,
    quant_table: *mut JQUANT_TBL,
    dct_table: *mut c_void,
}
impl Default for jpeg_component_info {
    fn default() -> jpeg_component_info { unsafe { mem::zeroed() } }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_scan_info {
    pub comps_in_scan: c_int,
    pub component_index: [c_int; 4usize],
    pub Ss: c_int,
    pub Se: c_int,
    pub Ah: c_int,
    pub Al: c_int,
}
impl Default for jpeg_scan_info {
    fn default() -> jpeg_scan_info { unsafe { mem::zeroed() } }
}

#[repr(C)]
pub struct jpeg_marker_struct {
    pub next: *mut jpeg_marker_struct,
    pub marker: u8,
    pub original_length: c_uint,
    pub data_length: c_uint,
    pub data: *mut u8,
}

pub enum jpeg_marker {
    APP0  = 0xE0,    /* APP0 marker code */
    COM  = 0xFE,    /* COM marker code */
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum J_COLOR_SPACE {
    /// error/unspecified
    JCS_UNKNOWN,
    /// monochrome
    JCS_GRAYSCALE,
    /// red/green/blue as specified by the RGB_RED, RGB_GREEN, RGB_BLUE, and RGB_PIXELSIZE macros
    JCS_RGB,
    /// Y/Cb/Cr (also known as YUV)
    JCS_YCbCr,
    /// C/M/Y/K
    JCS_CMYK,
    /// Y/Cb/Cr/K
    JCS_YCCK,
    /// red/green/blue
    JCS_EXT_RGB,
    /// red/green/blue/x
    /// When out_color_space it set to JCS_EXT_RGBX, JCS_EXT_BGRX, JCS_EXT_XBGR,
    /// or JCS_EXT_XRGB during decompression, the X byte is undefined, and in
    /// order to ensure the best performance, libjpeg-turbo can set that byte to
    /// whatever value it wishes.
    JCS_EXT_RGBX,
    /// blue/green/red
    JCS_EXT_BGR,
    /// blue/green/red/x
    JCS_EXT_BGRX,
    /// x/blue/green/red
    JCS_EXT_XBGR,
    /// x/red/green/blue
    JCS_EXT_XRGB,
    /// Use the following colorspace constants to
    /// ensure that the X byte is set to 0xFF, so that it can be interpreted as an
    /// opaque alpha channel.
    ///
    /// red/green/blue/alpha
    JCS_EXT_RGBA,
    /// blue/green/red/alpha
    JCS_EXT_BGRA,
    /// alpha/blue/green/red
    JCS_EXT_ABGR,
    /// alpha/red/green/blue
    JCS_EXT_ARGB,
    /// 5-bit red/6-bit green/5-bit blue
    JCS_RGB565,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum J_DCT_METHOD {
    JDCT_ISLOW = 0,
    JDCT_IFAST = 1,
    JDCT_FLOAT = 2,
}

#[repr(C)]
// #[deprecated]
pub enum J_DITHER_MODE {
    JDITHER_NONE = 0,
    JDITHER_ORDERED = 1,
    JDITHER_FS = 2,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
/// These 32-bit GUIDs and the corresponding `jpeg_*_get_*_param()`
/// `jpeg_*_set_*_param()` functions allow for extending the libjpeg API without
/// breaking backward ABI compatibility.  The actual parameters are stored in
/// the opaque `jpeg_comp_master` and `jpeg_decomp_master` structs.
pub enum J_BOOLEAN_PARAM {
    /// TRUE=optimize progressive coding scans
    JBOOLEAN_OPTIMIZE_SCANS = 0x680C061E,
    /// TRUE=use trellis quantization
    JBOOLEAN_TRELLIS_QUANT = 0xC5122033,
    /// TRUE=use trellis quant for DC coefficient
    JBOOLEAN_TRELLIS_QUANT_DC = 0x339D4C0C,
    /// TRUE=optimize for sequences of EOB
    JBOOLEAN_TRELLIS_EOB_OPT = 0xD7F73780,
    /// TRUE=use lambda weighting table
    JBOOLEAN_USE_LAMBDA_WEIGHT_TBL = 0x339DB65F,
    /// TRUE=use scans in trellis optimization
    JBOOLEAN_USE_SCANS_IN_TRELLIS = 0xFD841435,
    /// TRUE=optimize quant table in trellis loop
    JBOOLEAN_TRELLIS_Q_OPT = 0xE12AE269,
    /// TRUE=preprocess input to reduce ringing of edges on white background
    JBOOLEAN_OVERSHOOT_DERINGING = 0x3F4BBBF9,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum J_FLOAT_PARAM {
    JFLOAT_LAMBDA_LOG_SCALE1 = 0x5B61A599,
    JFLOAT_LAMBDA_LOG_SCALE2 = 0xB9BBAE03,
    JFLOAT_TRELLIS_DELTA_DC_WEIGHT = 0x13775453
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum J_INT_PARAM {
  /// compression profile
  JINT_COMPRESS_PROFILE = 0xE9918625,
  /// splitting point for frequency in trellis quantization
  JINT_TRELLIS_FREQ_SPLIT = 0x6FAFF127,
  /// number of trellis loops
  JINT_TRELLIS_NUM_LOOPS = 0xB63EBF39,
  /// base quantization table index
  JINT_BASE_QUANT_TBL_IDX = 0x44492AB1,
  /// DC scan optimization mode
  JINT_DC_SCAN_OPT_MODE = 0x0BE7AD3C
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum JINT_COMPRESS_PROFILE_VALUE {
  JCP_MAX_COMPRESSION = 0x5D083AAD, /* best compression ratio (progressive, all mozjpeg extensions) */
  JCP_FASTEST = 0x2AEA5CB4 /* libjpeg[-turbo] defaults (baseline, no mozjpeg extensions) */
}

#[repr(C)]
/// Routines that are to be used by both halves of the library are declared
/// to receive a pointer to this structure.  There are no actual instances of
/// jpeg_common_struct, only of jpeg_compress_struct and jpeg_decompress_struct.
pub struct jpeg_common_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    /// Progress monitor, or NULL if none
    pub progress: *mut jpeg_progress_mgr,
    /// Available for use by application
    pub client_data: *mut c_void,
    /// So common code can tell which is which
    pub is_decompressor: boolean,
    /// For checking call sequence validity
    pub global_state: c_int,
}

enum jpeg_comp_master {}
enum jpeg_c_main_controller {}
enum jpeg_c_prep_controller {}
enum jpeg_c_coef_controller {}
enum jpeg_marker_writer {}
enum jpeg_color_converter {}
enum jpeg_downsampler {}
enum jpeg_forward_dct {}
enum jpeg_entropy_encoder {}

#[repr(C)]
pub struct jpeg_compress_struct {
    pub common : jpeg_common_struct,
    pub dest: *mut jpeg_destination_mgr,
    /// Description of source image --- these fields must be filled in by
    /// outer application before starting compression.
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub input_components: c_int,
    /// `in_color_space` must be correct before you can even call `jpeg_set_defaults()`.
    pub in_color_space: J_COLOR_SPACE,
    /// image gamma of input image
    pub input_gamma: f64,
    /// bits of precision in image data
    pub data_precision: c_int,
    pub num_components: c_int,
    pub jpeg_color_space: J_COLOR_SPACE,
    /// comp_info[i] describes component that appears i'th in SOF
    pub comp_info: *mut jpeg_component_info,
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; 4usize],
    /// ptrs to coefficient quantization tables, or NULL if not defined,
    /// and corresponding scale factors (percentage, initialized 100).
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4usize],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4usize],
    pub arith_dc_L: [u8; 16usize],
    pub arith_dc_U: [u8; 16usize],
    pub arith_ac_K: [u8; 16usize],
    pub num_scans: c_int,
    pub scan_info: *const jpeg_scan_info,
    /// TRUE=caller supplies downsampled data
    pub raw_data_in: boolean,
    pub arith_code: boolean,
    /// TRUE=optimize entropy encoding parms
    pub optimize_coding: boolean,
    pub CCIR601_sampling: boolean,
    pub smoothing_factor: c_int,
    pub dct_method: J_DCT_METHOD,
    /// MCUs per restart, or 0 for no restart
    pub restart_interval: c_uint,
    pub restart_in_rows: c_int,
    pub write_JFIF_header: boolean,
    pub JFIF_major_version: u8,
    pub JFIF_minor_version: u8,
    pub density_unit: u8,
    pub X_density: u16,
    pub Y_density: u16,
    pub write_Adobe_marker: boolean,
    pub next_scanline: JDIMENSION,
    /// Remaining fields are known throughout compressor, but generally
    /// should not be touched by a surrounding application.
    /// These fields are computed during compression startup
    progressive_mode: boolean,
    pub max_h_samp_factor: c_int,
    pub max_v_samp_factor: c_int,
    total_iMCU_rows: JDIMENSION,
    comps_in_scan: c_int,
    cur_comp_info: [*mut jpeg_component_info; 4usize],
    MCUs_per_row: JDIMENSION,
    MCU_rows_in_scan: JDIMENSION,
    blocks_in_MCU: c_int,
    MCU_membership: [c_int; 10usize],
    Ss: c_int,
    Se: c_int,
    Ah: c_int,
    Al: c_int,
    master: *mut jpeg_comp_master,
    main: *mut jpeg_c_main_controller,
    prep: *mut jpeg_c_prep_controller,
    coef: *mut jpeg_c_coef_controller,
    marker: *mut jpeg_marker_writer,
    cconvert: *mut jpeg_color_converter,
    downsample: *mut jpeg_downsampler,
    fdct: *mut jpeg_forward_dct,
    entropy: *mut jpeg_entropy_encoder,
    script_space: *mut jpeg_scan_info,
    script_space_size: c_int,
}

enum jpeg_decomp_master {}
enum jpeg_d_main_controller {}
enum jpeg_d_coef_controller {}
enum jpeg_d_post_controller {}
enum jpeg_input_controller {}
enum jpeg_marker_reader {}
enum jpeg_entropy_decoder {}
enum jpeg_inverse_dct {}
enum jpeg_upsampler {}
enum jpeg_color_deconverter {}
enum jpeg_color_quantizer {}

#[repr(C)]
pub struct jpeg_decompress_struct {
    pub common: jpeg_common_struct,

    pub src: *mut jpeg_source_mgr,
    /// Basic description of image --- filled in by jpeg_read_header()
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub num_components: c_int,
    pub jpeg_color_space: J_COLOR_SPACE,
    /// Decompression processing parameters --- these fields must be set before
    /// calling jpeg_start_decompress().  Note that jpeg_read_header() initializes
    /// them to default values.
    pub out_color_space: J_COLOR_SPACE,
    pub scale_num: c_uint,
    pub scale_denom: c_uint,
    /// image gamma wanted in output
    pub output_gamma: f64,
    pub buffered_image: boolean,
    /// TRUE=downsampled data wanted
    pub raw_data_out: boolean,
    pub dct_method: J_DCT_METHOD,
    pub do_fancy_upsampling: boolean,
    pub do_block_smoothing: boolean,
    // #[deprecated]
    pub quantize_colors: boolean,
    // #[deprecated]
    pub dither_mode: J_DITHER_MODE,
    // #[deprecated]
    pub two_pass_quantize: boolean,
    // #[deprecated]
    pub desired_number_of_colors: c_int,
    // #[deprecated]
    pub enable_1pass_quant: boolean,
    // #[deprecated]
    pub enable_external_quant: boolean,
    // #[deprecated]
    pub enable_2pass_quant: boolean,
    /// Description of actual output image that will be returned to application.
    /// These fields are computed by jpeg_start_decompress().
    /// You can also use jpeg_calc_output_dimensions() to determine these values
    /// in advance of calling jpeg_start_decompress().
    pub output_width: JDIMENSION,
    pub output_height: JDIMENSION,
    pub out_color_components: c_int,
    pub output_components: c_int,
    /// min recommended height of scanline buffer
    /// If the buffer passed to jpeg_read_scanlines() is less than this many rows
    /// high, space and time will be wasted due to unnecessary data copying.
    /// Usually rec_outbuf_height will be 1 or 2, at most 4.
    pub rec_outbuf_height: c_int,
    // #[deprecated]
    pub actual_number_of_colors: c_int,
    // #[deprecated]
    pub colormap: JSAMPARRAY_MUT,
    /// Row index of next scanline to be read from jpeg_read_scanlines().
    /// Application may use this to control its processing loop, e.g.,
    /// "while (output_scanline < output_height)".
    pub output_scanline: JDIMENSION,
    /// Current input scan number and number of iMCU rows completed in scan.
    /// These indicate the progress of the decompressor input side.
    pub input_scan_number: c_int,
    pub input_iMCU_row: JDIMENSION,
    pub output_scan_number: c_int,
    pub output_iMCU_row: JDIMENSION,
    /// Current progression status.  coef_bits[c][i] indicates the precision
    /// with which component c's DCT coefficient i (in zigzag order) is known.
    /// It is -1 when no data has yet been received, otherwise it is the point
    /// transform (shift) value for the most recent scan of the coefficient
    /// (thus, 0 at completion of the progression).
    /// This pointer is NULL when reading a non-progressive file.
    pub coef_bits: *mut c_void,
    /// Internal JPEG parameters --- the application usually need not look at
    /// these fields.  Note that the decompressor output side may not use
    /// any parameters that can change between scans.
    quant_tbl_ptrs: [*mut JQUANT_TBL; 4usize],
    dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4usize],
    ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4usize],
    data_precision: c_int,
    pub comp_info: *mut jpeg_component_info,
    progressive_mode: boolean,
    arith_code: boolean,
    arith_dc_L: [u8; 16usize],
    arith_dc_U: [u8; 16usize],
    arith_ac_K: [u8; 16usize],
    restart_interval: c_uint,
    saw_JFIF_marker: boolean,
    JFIF_major_version: u8,
    JFIF_minor_version: u8,
    density_unit: u8,
    X_density: u16,
    Y_density: u16,
    saw_Adobe_marker: boolean,
    Adobe_transform: u8,
    CCIR601_sampling: boolean,
    pub marker_list: *mut jpeg_marker_struct,
    /// These fields are computed during decompression startup
    pub max_h_samp_factor: c_int,
    pub max_v_samp_factor: c_int,
    min_DCT_scaled_size: c_int,
    total_iMCU_rows: JDIMENSION,
    sample_range_limit: *mut JSAMPLE,
    comps_in_scan: c_int,
    cur_comp_info: [*mut jpeg_component_info; 4usize],
    MCUs_per_row: JDIMENSION,
    MCU_rows_in_scan: JDIMENSION,
    blocks_in_MCU: c_int,
    MCU_membership: [c_int; 10usize],
    Ss: c_int,
    Se: c_int,
    Ah: c_int,
    Al: c_int,
    unread_marker: c_int,
    master: *mut jpeg_decomp_master,
    main: *mut jpeg_d_main_controller,
    coef: *mut jpeg_d_coef_controller,
    post: *mut jpeg_d_post_controller,
    inputctl: *mut jpeg_input_controller,
    marker: *mut jpeg_marker_reader,
    entropy: *mut jpeg_entropy_decoder,
    idct: *mut jpeg_inverse_dct,
    upsample: *mut jpeg_upsampler,
    cconvert: *mut jpeg_color_deconverter,
    cquantize: *mut jpeg_color_quantizer,
}

#[repr(C)]
/// Error handler object
pub struct jpeg_error_mgr {
    /// Error exit handler: does not return to caller
    pub error_exit: Option<extern "C" fn(cinfo: &mut jpeg_common_struct)>,
    pub emit_message: Option<extern "C" fn(cinfo: &mut jpeg_common_struct, msg_level: c_int)>,
    pub output_message: Option<extern "C" fn(cinfo: &mut jpeg_common_struct)>,
    pub format_message: Option<extern "C" fn(cinfo: &mut jpeg_common_struct, buffer: &[u8; 80usize])>,
    pub reset_error_mgr: Option<extern "C" fn(cinfo: &mut jpeg_common_struct)>,
    pub msg_code: c_int,
    pub msg_parm: msg_parm_union,
    pub trace_level: c_int,
    pub num_warnings: c_long,
    pub jpeg_message_table: *const *const i8,
    pub last_jpeg_message: c_int,
    pub addon_message_table: *const *const i8,
    pub first_addon_message: c_int,
    pub last_addon_message: c_int,
}

#[repr(C)]
pub struct msg_parm_union {
    pub _bindgen_data_: [u32; 20usize],
}
impl msg_parm_union {
    pub unsafe fn i(&mut self) -> *mut [c_int; 8usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
    pub unsafe fn s(&mut self) -> *mut [i8; 80usize] {
        ::std::mem::transmute(&self._bindgen_data_)
    }
}
impl Default for msg_parm_union {
    fn default() -> msg_parm_union { unsafe { mem::zeroed() } }
}

#[repr(C)]
pub struct jpeg_progress_mgr {
    pub progress_monitor: Option<extern "C" fn(cinfo: &mut jpeg_common_struct)>,
    pub pass_counter: c_long,
    pub pass_limit: c_long,
    pub completed_passes: c_int,
    pub total_passes: c_int,
}

#[repr(C)]
pub struct jpeg_destination_mgr {
    pub next_output_byte: *mut u8,
    pub free_in_buffer: usize,
    pub init_destination: Option<extern "C" fn(cinfo: &mut jpeg_compress_struct)>,
    pub empty_output_buffer: Option<extern "C" fn(cinfo: &mut jpeg_compress_struct)
                                                       -> boolean>,
    pub term_destination: Option<extern "C" fn(cinfo: &mut jpeg_compress_struct)>,
}

#[repr(C)]
pub struct jpeg_source_mgr {
    pub next_input_byte: *const u8,
    pub bytes_in_buffer: usize,
    pub init_source: Option<extern "C" fn(cinfo: &mut jpeg_decompress_struct)>,
    pub fill_input_buffer: Option<extern "C" fn(cinfo: &mut jpeg_decompress_struct)
                                                     -> boolean>,
    pub skip_input_data: Option<extern "C" fn(cinfo: &mut jpeg_decompress_struct,
                                                    num_bytes:
                                                        c_long)>,
    pub resync_to_restart: Option<extern "C" fn(cinfo: &mut jpeg_decompress_struct,
                                                      desired: c_int)
                                                     -> boolean>,
    pub term_source: Option<extern "C" fn(cinfo: &mut jpeg_decompress_struct)>,
}

pub enum jvirt_sarray_control {}
pub enum jvirt_barray_control {}

#[repr(C)]
pub struct jpeg_memory_mgr {
    pub alloc_small: Option<extern "C" fn(cinfo: &mut jpeg_common_struct,
                                                pool_id: c_int,
                                                sizeofobject: usize) -> *mut c_void>,
    pub alloc_large: Option<extern "C" fn(cinfo: &mut jpeg_common_struct,
                                                pool_id: c_int,
                                                sizeofobject: usize) -> *mut c_void>,
    pub alloc_sarray: Option<extern "C" fn(cinfo: &mut jpeg_common_struct,
                                                 pool_id: c_int,
                                                 samplesperrow: JDIMENSION,
                                                 numrows: JDIMENSION) -> JSAMPARRAY_MUT>,
    pub alloc_barray: Option<extern "C" fn(cinfo: &mut jpeg_common_struct,
                                                 pool_id: c_int,
                                                 blocksperrow: JDIMENSION,
                                                 numrows: JDIMENSION) -> JBLOCKARRAY>,
    pub request_virt_sarray: Option<extern "C" fn(cinfo: &mut jpeg_common_struct,
                                                        pool_id: c_int,
                                                        pre_zero: boolean,
                                                        samplesperrow: JDIMENSION,
                                                        numrows: JDIMENSION,
                                                        maxaccess: JDIMENSION) -> *mut jvirt_sarray_control>,
    pub request_virt_barray: Option<extern "C" fn(cinfo: &mut jpeg_common_struct,
                                                        pool_id:
                                                            c_int,
                                                        pre_zero: boolean,
                                                        blocksperrow:
                                                            JDIMENSION,
                                                        numrows: JDIMENSION,
                                                        maxaccess: JDIMENSION) -> *mut jvirt_barray_control>,
    pub realize_virt_arrays: Option<extern "C" fn(cinfo: &mut jpeg_common_struct)>,
    pub access_virt_sarray: Option<extern "C" fn(cinfo: &mut jpeg_common_struct,
                                                       ptr: *mut jvirt_sarray_control,
                                                       start_row: JDIMENSION,
                                                       num_rows: JDIMENSION,
                                                       writable: boolean) -> JSAMPARRAY_MUT>,
    pub access_virt_barray: Option<extern "C" fn(cinfo: &mut jpeg_common_struct,
                                                       ptr: *mut jvirt_barray_control,
                                                       start_row: JDIMENSION,
                                                       num_rows: JDIMENSION,
                                                       writable: boolean) -> JBLOCKARRAY>,
    pub free_pool: Option<extern "C" fn(cinfo: &mut jpeg_common_struct, pool_id: c_int)>,
    pub self_destruct: Option<extern "C" fn(cinfo: &mut jpeg_common_struct)>,
    pub max_memory_to_use: c_long,
    pub max_alloc_chunk: c_long,
}

pub type jpeg_marker_parser_method = Option<extern "C" fn(cinfo: &mut jpeg_decompress_struct) -> boolean>;

extern "C" {
    pub fn jpeg_std_error<'a>(err: &'a mut jpeg_error_mgr) -> &'a mut jpeg_error_mgr;
    pub fn jpeg_CreateCompress(cinfo: &mut jpeg_compress_struct, version: c_int, structsize: usize);
    pub fn jpeg_CreateDecompress(cinfo: &mut jpeg_decompress_struct, version: c_int, structsize: usize);
    pub fn jpeg_destroy_compress(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_destroy_decompress(cinfo: &mut jpeg_decompress_struct);
    pub fn jpeg_stdio_dest(cinfo: &mut jpeg_compress_struct, outfile: *mut FILE);
    pub fn jpeg_stdio_src(cinfo: &mut jpeg_decompress_struct, infile: *mut FILE);
    pub fn jpeg_mem_dest(cinfo: &mut jpeg_compress_struct,
                     outbuffer: *mut *mut u8,
                     outsize: *mut c_ulong);
    pub fn jpeg_mem_src(cinfo: &mut jpeg_decompress_struct,
                    inbuffer: *const u8,
                    insize: c_ulong);
    pub fn jpeg_set_defaults(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_set_colorspace(cinfo: &mut jpeg_compress_struct, colorspace: J_COLOR_SPACE);
    pub fn jpeg_default_colorspace(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_set_quality(cinfo: &mut jpeg_compress_struct, quality: c_int, force_baseline: boolean);
    pub fn jpeg_set_linear_quality(cinfo: &mut jpeg_compress_struct,
                               scale_factor: c_int,
                               force_baseline: boolean);
    pub fn jpeg_add_quant_table(cinfo: &mut jpeg_compress_struct,
                            which_tbl: c_int,
                            basic_table: *const c_uint,
                            scale_factor: c_int,
                            force_baseline: boolean);
    pub fn jpeg_float_add_quant_table(cinfo: &mut jpeg_compress_struct,
                                  which_tbl: c_int,
                                  basic_table: *const c_uint,
                                  scale_factor: f32,
                                  force_baseline: boolean);
    pub fn jpeg_quality_scaling(quality: c_int) -> c_int;
    pub fn jpeg_float_quality_scaling(quality: f32) -> f32;
    pub fn jpeg_simple_progression(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_suppress_tables(cinfo: &mut jpeg_compress_struct, suppress: boolean);
    pub fn jpeg_alloc_quant_table(cinfo: &mut jpeg_common_struct) -> *mut JQUANT_TBL;
    pub fn jpeg_alloc_huff_table(cinfo: &mut jpeg_common_struct) -> *mut JHUFF_TBL;
    pub fn jpeg_start_compress(cinfo: &mut jpeg_compress_struct, write_all_tables: boolean);
    pub fn jpeg_write_scanlines(cinfo: &mut jpeg_compress_struct, scanlines: JSAMPARRAY,
                            num_lines: JDIMENSION) -> JDIMENSION;
    pub fn jpeg_finish_compress(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_write_raw_data(cinfo: &mut jpeg_compress_struct, data: JSAMPIMAGE,
                           num_lines: JDIMENSION) -> JDIMENSION;
    pub fn jpeg_write_marker(cinfo: &mut jpeg_compress_struct, marker: c_int,
                         dataptr: *const u8, datalen: c_uint);
    pub fn jpeg_write_m_header(cinfo: &mut jpeg_compress_struct, marker: c_int, datalen: c_uint);
    pub fn jpeg_write_m_byte(cinfo: &mut jpeg_compress_struct, val: c_int);
    pub fn jpeg_write_tables(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_read_header(cinfo: &mut jpeg_decompress_struct, require_image: boolean) -> c_int;
    pub fn jpeg_start_decompress(cinfo: &mut jpeg_decompress_struct) -> boolean;
    pub fn jpeg_read_scanlines(cinfo: &mut jpeg_decompress_struct, scanlines: JSAMPARRAY_MUT,
                           max_lines: JDIMENSION) -> JDIMENSION;
    pub fn jpeg_finish_decompress(cinfo: &mut jpeg_decompress_struct) -> boolean;
    pub fn jpeg_read_raw_data(cinfo: &mut jpeg_decompress_struct, data: JSAMPIMAGE_MUT,
                          max_lines: JDIMENSION) -> JDIMENSION;
    pub fn jpeg_has_multiple_scans(cinfo: &jpeg_decompress_struct) -> boolean;
    pub fn jpeg_start_output(cinfo: &mut jpeg_decompress_struct, scan_number: c_int) -> boolean;
    pub fn jpeg_finish_output(cinfo: &mut jpeg_decompress_struct) -> boolean;
    pub fn jpeg_input_complete(cinfo: &jpeg_decompress_struct) -> boolean;
    // #[deprecated]
    pub fn jpeg_new_colormap(cinfo: &mut jpeg_decompress_struct);
    pub fn jpeg_consume_input(cinfo: &mut jpeg_decompress_struct) -> c_int;
    pub fn jpeg_calc_output_dimensions(cinfo: &mut jpeg_decompress_struct);
    pub fn jpeg_save_markers(cinfo: &mut jpeg_decompress_struct,
                         marker_code: c_int,
                         length_limit: c_uint);
    pub fn jpeg_set_marker_processor(cinfo: &mut jpeg_decompress_struct,
                                 marker_code: c_int,
                                 routine: jpeg_marker_parser_method);
    pub fn jpeg_read_coefficients(cinfo: &mut jpeg_decompress_struct) -> *mut *mut jvirt_barray_control;
    pub fn jpeg_write_coefficients(cinfo: &mut jpeg_compress_struct,
                               coef_arrays: *mut *mut jvirt_barray_control);
    pub fn jpeg_copy_critical_parameters(srcinfo: &jpeg_decompress_struct,
                                     dstinfo: &mut jpeg_compress_struct);
    pub fn jpeg_abort_compress(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_abort_decompress(cinfo: &mut jpeg_decompress_struct);
    pub fn jpeg_resync_to_restart(cinfo: &mut jpeg_decompress_struct, desired: c_int) -> boolean;
    pub fn jpeg_c_bool_param_supported(cinfo: &jpeg_compress_struct,
                                   param: J_BOOLEAN_PARAM) -> boolean;
    pub fn jpeg_c_set_bool_param(cinfo: &mut jpeg_compress_struct,
                             param: J_BOOLEAN_PARAM, value: boolean);
    pub fn jpeg_c_get_bool_param(cinfo: &jpeg_compress_struct,
                             param: J_BOOLEAN_PARAM) -> boolean;
    pub fn jpeg_c_float_param_supported(cinfo: &jpeg_compress_struct, param: J_FLOAT_PARAM) -> boolean;
    pub fn jpeg_c_set_float_param(cinfo: &mut jpeg_compress_struct, param: J_FLOAT_PARAM, value: f32);
    pub fn jpeg_c_get_float_param(cinfo: &jpeg_compress_struct, param: J_FLOAT_PARAM) -> f32;
    pub fn jpeg_c_int_param_supported(cinfo: &jpeg_compress_struct, param: J_INT_PARAM) -> boolean;
    pub fn jpeg_c_set_int_param(cinfo: &mut jpeg_compress_struct, param: J_INT_PARAM, value: c_int);
    pub fn jpeg_c_get_int_param(cinfo: &jpeg_compress_struct, param: J_INT_PARAM) -> c_int;
    #[cfg(test)] fn jsimd_can_rgb_ycc() -> c_int;
    #[cfg(test)] fn jsimd_can_fdct_ifast() -> c_int;
    #[cfg(test)] fn jsimd_fdct_ifast_sse2(block: *mut DCTELEM);
}

#[test]
pub fn simd_is_detectable() {
    unsafe {
        jsimd_can_rgb_ycc();
    }
}

#[test]
#[cfg(all(target_arch="x86_64", feature="nasm_simd"))]
pub fn simd_works_sse2() {
    unsafe {
        assert!(jsimd_can_fdct_ifast() != 0);
        jsimd_fdct_ifast_sse2([0 as DCTELEM; 64].as_mut_ptr());
    }
}

#[test]
pub fn try_decompress() {
    unsafe {
        let mut err = mem::zeroed();
        jpeg_std_error(&mut err);
        let mut cinfo: jpeg_decompress_struct = mem::zeroed();
        let size = mem::size_of_val(&cinfo) as usize;
        cinfo.common.err = &mut err;
        jpeg_CreateDecompress(&mut cinfo, JPEG_LIB_VERSION, size);
        jpeg_destroy_decompress(&mut cinfo);
    }
}

#[test]
pub fn try_compress() {
    unsafe {
        let mut err = mem::zeroed();
        jpeg_std_error(&mut err);
        let mut cinfo: jpeg_compress_struct = mem::zeroed();
        let size = mem::size_of_val(&cinfo) as usize;
        cinfo.common.err = &mut err;
        if 0 == jpeg_c_bool_param_supported(&cinfo, JBOOLEAN_TRELLIS_QUANT) {
            panic!("Not linked to mozjpeg?");
        }
        jpeg_CreateCompress(&mut cinfo, JPEG_LIB_VERSION, size);
        jpeg_destroy_compress(&mut cinfo);
    }
}
