extern "C" {
    pub fn jpeg_std_error<'a>(err: &'a mut jpeg_error_mgr) -> *mut jpeg_error_mgr;
    pub fn jpeg_CreateCompress(cinfo: *mut jpeg_compress_struct, version: c_int, structsize: usize);
    pub fn jpeg_CreateDecompress(
        cinfo: *mut jpeg_decompress_struct,
        version: c_int,
        structsize: usize,
    );
    pub fn jpeg_destroy_compress(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_destroy_decompress(cinfo: &mut jpeg_decompress_struct);
    pub fn jpeg_stdio_dest(cinfo: &mut jpeg_compress_struct, outfile: *mut FILE);
    pub fn jpeg_stdio_src(cinfo: &mut jpeg_decompress_struct, infile: *mut FILE);
    pub fn jpeg_mem_dest(
        cinfo: &mut jpeg_compress_struct,
        outbuffer: *mut *mut u8,
        outsize: *mut c_ulong,
    );
    pub fn jpeg_mem_src(cinfo: &mut jpeg_decompress_struct, inbuffer: *const u8, insize: c_ulong);
    pub fn jpeg_set_defaults(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_set_colorspace(cinfo: &mut jpeg_compress_struct, colorspace: J_COLOR_SPACE);
    pub fn jpeg_default_colorspace(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_set_quality(
        cinfo: &mut jpeg_compress_struct,
        quality: c_int,
        force_baseline: boolean,
    );
    pub fn jpeg_set_linear_quality(
        cinfo: &mut jpeg_compress_struct,
        scale_factor: c_int,
        force_baseline: boolean,
    );
    pub fn jpeg_add_quant_table(
        cinfo: &mut jpeg_compress_struct,
        which_tbl: c_int,
        basic_table: *const c_uint,
        scale_factor: c_int,
        force_baseline: boolean,
    );
    pub fn jpeg_quality_scaling(quality: c_int) -> c_int;
    pub fn jpeg_float_quality_scaling(quality: f32) -> f32;
    pub fn jpeg_simple_progression(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_suppress_tables(cinfo: &mut jpeg_compress_struct, suppress: boolean);
    pub fn jpeg_alloc_quant_table(cinfo: &mut jpeg_common_struct) -> *mut JQUANT_TBL;
    pub fn jpeg_alloc_huff_table(cinfo: &mut jpeg_common_struct) -> *mut JHUFF_TBL;
    pub fn jpeg_start_compress(cinfo: &mut jpeg_compress_struct, write_all_tables: boolean);
    pub fn jpeg_write_scanlines(
        cinfo: &mut jpeg_compress_struct,
        scanlines: JSAMPARRAY,
        num_lines: JDIMENSION,
    ) -> JDIMENSION;
    pub fn jpeg_finish_compress(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_write_raw_data(
        cinfo: &mut jpeg_compress_struct,
        data: JSAMPIMAGE,
        num_lines: JDIMENSION,
    ) -> JDIMENSION;
    pub fn jpeg_write_marker(
        cinfo: &mut jpeg_compress_struct,
        marker: c_int,
        dataptr: *const u8,
        datalen: c_uint,
    );
    pub fn jpeg_write_m_header(cinfo: &mut jpeg_compress_struct, marker: c_int, datalen: c_uint);
    pub fn jpeg_write_m_byte(cinfo: &mut jpeg_compress_struct, val: c_int);
    pub fn jpeg_write_tables(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_read_header(cinfo: &mut jpeg_decompress_struct, require_image: boolean) -> c_int;
    pub fn jpeg_start_decompress(cinfo: &mut jpeg_decompress_struct) -> boolean;
    pub fn jpeg_read_scanlines(
        cinfo: &mut jpeg_decompress_struct,
        scanlines: JSAMPARRAY_MUT,
        max_lines: JDIMENSION,
    ) -> JDIMENSION;
    pub fn jpeg_finish_decompress(cinfo: &mut jpeg_decompress_struct) -> boolean;
    pub fn jpeg_read_raw_data(
        cinfo: &mut jpeg_decompress_struct,
        data: JSAMPIMAGE_MUT,
        max_lines: JDIMENSION,
    ) -> JDIMENSION;
    pub fn jpeg_has_multiple_scans(cinfo: &mut jpeg_decompress_struct) -> boolean;
    pub fn jpeg_start_output(cinfo: &mut jpeg_decompress_struct, scan_number: c_int) -> boolean;
    pub fn jpeg_finish_output(cinfo: &mut jpeg_decompress_struct) -> boolean;
    pub fn jpeg_input_complete(cinfo: &mut jpeg_decompress_struct) -> boolean;
    #[deprecated]
    pub fn jpeg_new_colormap(cinfo: &mut jpeg_decompress_struct);
    pub fn jpeg_consume_input(cinfo: &mut jpeg_decompress_struct) -> c_int;
    pub fn jpeg_float_add_quant_table(
        cinfo: &mut jpeg_compress_struct,
        which_tbl: c_int,
        basic_table: *const c_uint,
        scale_factor: f32,
        force_baseline: boolean,
    );

    /// Precalculate JPEG dimensions for current compression parameters
    pub fn jpeg_save_markers(
        cinfo: &mut jpeg_decompress_struct,
        marker_code: c_int,
        length_limit: c_uint,
    );
    pub fn jpeg_set_marker_processor(
        cinfo: &mut jpeg_decompress_struct,
        marker_code: c_int,
        routine: jpeg_marker_parser_method,
    );
    pub fn jpeg_read_coefficients(
        cinfo: &mut jpeg_decompress_struct,
    ) -> *mut *mut jvirt_barray_control;
    pub fn jpeg_write_coefficients(
        cinfo: &mut jpeg_compress_struct,
        coef_arrays: *mut *mut jvirt_barray_control,
    );
    pub fn jpeg_copy_critical_parameters(
        srcinfo: &mut jpeg_decompress_struct,
        dstinfo: &mut jpeg_compress_struct,
    );
    pub fn jpeg_abort_compress(cinfo: &mut jpeg_compress_struct);
    pub fn jpeg_abort_decompress(cinfo: &mut jpeg_decompress_struct);
    pub fn jpeg_resync_to_restart(cinfo: &mut jpeg_decompress_struct, desired: c_int) -> boolean;
    pub fn jpeg_c_bool_param_supported(
        cinfo: &mut jpeg_compress_struct,
        param: J_BOOLEAN_PARAM,
    ) -> boolean;
    pub fn jpeg_c_set_bool_param(
        cinfo: &mut jpeg_compress_struct,
        param: J_BOOLEAN_PARAM,
        value: boolean,
    );
    pub fn jpeg_c_get_bool_param(cinfo: &mut jpeg_compress_struct, param: J_BOOLEAN_PARAM) -> boolean;
    pub fn jpeg_c_float_param_supported(
        cinfo: &mut jpeg_compress_struct,
        param: J_FLOAT_PARAM,
    ) -> boolean;
    pub fn jpeg_c_set_float_param(
        cinfo: &mut jpeg_compress_struct,
        param: J_FLOAT_PARAM,
        value: f32,
    );
    pub fn jpeg_c_get_float_param(cinfo: &mut jpeg_compress_struct, param: J_FLOAT_PARAM) -> f32;
    pub fn jpeg_c_int_param_supported(cinfo: &mut jpeg_compress_struct, param: J_INT_PARAM) -> boolean;
    pub fn jpeg_c_set_int_param(cinfo: &mut jpeg_compress_struct, param: J_INT_PARAM, value: c_int);
    pub fn jpeg_c_get_int_param(cinfo: &mut jpeg_compress_struct, param: J_INT_PARAM) -> c_int;
}
