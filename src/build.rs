extern crate cc;
extern crate nasm_rs;

fn main() {
    let mut c = cc::Build::new();

    c.include("vendor");
    c.warnings(false);

    let files = &[
        "vendor/jcapimin.c", "vendor/jcapistd.c", "vendor/jccoefct.c", "vendor/jccolor.c",
        "vendor/jcdctmgr.c", "vendor/jcext.c", "vendor/jchuff.c", "vendor/jcinit.c",
        "vendor/jcmainct.c", "vendor/jcmarker.c", "vendor/jcmaster.c", "vendor/jcomapi.c",
        "vendor/jcparam.c", "vendor/jcphuff.c", "vendor/jcprepct.c", "vendor/jcsample.c",
        "vendor/jctrans.c", "vendor/jdapimin.c", "vendor/jdapistd.c", "vendor/jdatadst.c",
        "vendor/jdatasrc.c", "vendor/jdcoefct.c", "vendor/jdcolor.c", "vendor/jddctmgr.c",
        "vendor/jdhuff.c", "vendor/jdinput.c", "vendor/jdmainct.c", "vendor/jdmarker.c",
        "vendor/jdmaster.c", "vendor/jdmerge.c", "vendor/jdphuff.c", "vendor/jdpostct.c",
        "vendor/jdsample.c", "vendor/jdtrans.c", "vendor/jerror.c", "vendor/jfdctflt.c",
        "vendor/jfdctfst.c", "vendor/jfdctint.c", "vendor/jidctflt.c", "vendor/jidctfst.c",
        "vendor/jidctint.c", "vendor/jidctred.c", "vendor/jmemmgr.c", "vendor/jmemnobs.c",
        "vendor/jquant1.c", "vendor/jquant2.c", "vendor/jutils.c",
    ];

    for file in files.iter() {
        c.file(file);
    }

    c.define("PACKAGE_NAME", Some(format!("\"{}\"", env!("CARGO_PKG_NAME")).as_str()));
    c.define("VERSION", Some(format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str()));
    c.define("BUILD", Some(format!("\"{}-mozjpeg-sys\"", std::time::UNIX_EPOCH.elapsed().unwrap().as_secs()).as_str()));

    c.define("STDC_HEADERS", Some("1"));
    c.define("HAVE_UNSIGNED_CHAR", Some("1"));
    c.define("HAVE_UNSIGNED_SHORT", Some("1"));
    c.define("HAVE_STDLIB_H", Some("1"));
    c.define("SIZEOF_SIZE_T", Some(if cfg!(target_pointer_width = 32) {"4"} else {"8"}));
    c.define("INLINE", Some("inline"));

    c.define("MEM_SRCDST_SUPPORTED", Some("1"));
    c.define("JPEG_LIB_VERSION", Some("62"));
    c.define("BITS_IN_JSAMPLE", Some("8"));

    if cfg!(feature = "arith_enc") {
        c.define("C_ARITH_CODING_SUPPORTED", Some("1"));
        c.file("vendor/jcarith.c");
    }
    if cfg!(feature = "arith_dec") {
        c.define("D_ARITH_CODING_SUPPORTED", Some("1"));
        c.file("vendor/jdarith.c");
    }
    if cfg!(feature = "arith_enc") || cfg!(feature = "arith_dec") {
        c.file("vendor/jaricom.c");
    }

    if cfg!(feature = "turbojpeg_api") {
        c.define("WITH_TURBOJPEG", Some("1"));
        c.file("vendor/turbojpeg.c");
        c.file("vendor/transupp.c");
        c.file("vendor/jdatadst-tj.c");
        c.file("vendor/jdatasrc-tj.c");
    }

    if cfg!(feature = "nasm_simd") {
        c.include("vendor/simd");
        c.define("WITH_SIMD", Some("1"));

        if cfg!(target_arch = "x86_64") {
            c.file("vendor/simd/jsimd_x86_64.c");
        } else if cfg!(target_arch = "x86") {
            c.file("vendor/simd/jsimd_i386.c");
        } else if cfg!(target_arch = "mips") {
            c.file("vendor/simd/jsimd_mips.c");
        } else if cfg!(target_arch = "powerpc") || cfg!(target_arch = "powerpc64") {
            c.file("vendor/simd/jsimd_powerpc.c");
        } else if cfg!(target_arch = "arm") {
            c.file("vendor/simd/jsimd_arm.c");
        } else if cfg!(target_arch = "aarch64") {
            c.file("vendor/simd/jsimd_arm64.c");
        }
        build_nasm();
    } else {
        c.file("vendor/jsimd_none.c");
    }

    c.compile("libmozjpeg.a");
}

fn build_nasm() {
    let mut flags = vec!["-Ivendor/simd/", "-Ivendor/win/"];
    if std::env::var("PROFILE").map(|s| "debug" == s).unwrap_or(false) {
        flags.push("-g");
    }

    if cfg!(target_os = "linux") {
        flags.push("-DELF");
    }

    let x86_64 = [
        "vendor/simd/jfdctflt-sse-64.asm", "vendor/simd/jccolor-sse2-64.asm", "vendor/simd/jcgray-sse2-64.asm",
        "vendor/simd/jchuff-sse2-64.asm", "vendor/simd/jcsample-sse2-64.asm", "vendor/simd/jdcolor-sse2-64.asm",
        "vendor/simd/jdmerge-sse2-64.asm", "vendor/simd/jdsample-sse2-64.asm", "vendor/simd/jfdctfst-sse2-64.asm",
        "vendor/simd/jfdctint-sse2-64.asm", "vendor/simd/jidctflt-sse2-64.asm", "vendor/simd/jidctfst-sse2-64.asm",
        "vendor/simd/jidctint-sse2-64.asm", "vendor/simd/jidctred-sse2-64.asm", "vendor/simd/jquantf-sse2-64.asm",
        "vendor/simd/jquanti-sse2-64.asm",
    ];
    let x86 = [
        "vendor/simd/jsimdcpu.asm", "vendor/simd/jfdctflt-3dn.asm", "vendor/simd/jidctflt-3dn.asm",
        "vendor/simd/jquant-3dn.asm", "vendor/simd/jccolor-mmx.asm", "vendor/simd/jcgray-mmx.asm",
        "vendor/simd/jcsample-mmx.asm", "vendor/simd/jdcolor-mmx.asm", "vendor/simd/jdmerge-mmx.asm",
        "vendor/simd/jdsample-mmx.asm", "vendor/simd/jfdctfst-mmx.asm", "vendor/simd/jfdctint-mmx.asm",
        "vendor/simd/jidctfst-mmx.asm", "vendor/simd/jidctint-mmx.asm", "vendor/simd/jidctred-mmx.asm",
        "vendor/simd/jquant-mmx.asm", "vendor/simd/jfdctflt-sse.asm", "vendor/simd/jidctflt-sse.asm",
        "vendor/simd/jquant-sse.asm", "vendor/simd/jccolor-sse2.asm", "vendor/simd/jcgray-sse2.asm",
        "vendor/simd/jchuff-sse2.asm", "vendor/simd/jcsample-sse2.asm", "vendor/simd/jdcolor-sse2.asm",
        "vendor/simd/jdmerge-sse2.asm", "vendor/simd/jdsample-sse2.asm", "vendor/simd/jfdctfst-sse2.asm",
        "vendor/simd/jfdctint-sse2.asm", "vendor/simd/jidctflt-sse2.asm", "vendor/simd/jidctfst-sse2.asm",
        "vendor/simd/jidctint-sse2.asm", "vendor/simd/jidctred-sse2.asm", "vendor/simd/jquantf-sse2.asm",
        "vendor/simd/jquanti-sse2.asm",
    ];

    let files: &[_] = if cfg!(target_arch = "x86_64") {
        flags.push("-D__x86_64__");
        &x86_64
    } else if cfg!(target_arch = "x86") {
        &x86
    } else {
        panic!("The mozjpeg-sys SIMD build script is incomplete for this platform");
    };

    nasm_rs::compile_library_args("libmozjpegsimd.a", files, &flags);
    println!("cargo:rustc-link-lib=static=mozjpegsimd");
}
