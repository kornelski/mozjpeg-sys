extern crate dunce;
extern crate cc;
#[cfg(feature = "nasm_simd")]
extern crate nasm_rs;
#[allow(unused_imports)]
use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use std::process;
use std::io::Write;

fn main() {
    let mut c = cc::Build::new();
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let config_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("include");
    let vendor = dunce::canonicalize(root.join("vendor")).unwrap();

    fs::create_dir_all(&config_dir).unwrap();

    println!("cargo:include={}", env::join_paths(&[&config_dir, &vendor]).unwrap().to_str().unwrap());
    c.include(&config_dir);
    c.include(&vendor);
    c.pic(true);

    let target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap();

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

    let abi = if cfg!(feature = "jpeg80_abi") {
        "80"
    } else if cfg!(feature = "jpeg70_abi") {
        "70"
    } else {
        "62"
    };
    println!("cargo:lib_version={}", abi);

    let mut jconfigint_h = fs::File::create(config_dir.join("jconfigint.h")).unwrap();
    write!(jconfigint_h, r#"
        #define BUILD "{timestamp}-mozjpeg-sys"
        #ifndef INLINE
            #if defined(__GNUC__)
                #define INLINE inline __attribute__((always_inline))
            #elif defined(_MSC_VER)
                #define INLINE __forceinline
            #else
                #define INLINE inline
            #endif
        #endif
        #define PACKAGE_NAME "{PACKAGE_NAME}"
        #define VERSION "{VERSION}"
        #define SIZEOF_SIZE_T {SIZEOF_SIZE_T}
        "#,
        timestamp = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs(),
        PACKAGE_NAME = env::var("CARGO_PKG_NAME").unwrap(),
        VERSION = env::var("CARGO_PKG_VERSION").unwrap(),
        SIZEOF_SIZE_T = if target_pointer_width == "32" {4} else {8}
    ).unwrap();
    drop(jconfigint_h); // close the file

    let mut jconfig_h = fs::File::create(config_dir.join("jconfig.h")).unwrap();
    write!(jconfig_h, r#"
        #define JPEG_LIB_VERSION {JPEG_LIB_VERSION}
        #define LIBJPEG_TURBO_VERSION 0
        #define BITS_IN_JSAMPLE 8
        #define STDC_HEADERS 1
        #define HAVE_STDLIB_H 1
        #define HAVE_UNSIGNED_CHAR 1
        #define HAVE_UNSIGNED_SHORT 1
        #define MEM_SRCDST_SUPPORTED 1
        "#,
        JPEG_LIB_VERSION = abi
    ).unwrap();

    if cfg!(feature = "arith_enc") {
        jconfig_h.write_all(b"#define C_ARITH_CODING_SUPPORTED 1\n").unwrap();
        c.file("vendor/jcarith.c");
    }
    if cfg!(feature = "arith_dec") {
        jconfig_h.write_all(b"#define D_ARITH_CODING_SUPPORTED 1\n").unwrap();
        c.file("vendor/jdarith.c");
    }


    if cfg!(feature = "arith_enc") || cfg!(feature = "arith_dec") {
        c.file("vendor/jaricom.c");
    }

    if cfg!(feature = "turbojpeg_api") {
        c.file("vendor/turbojpeg.c");
        c.file("vendor/transupp.c");
        c.file("vendor/jdatadst-tj.c");
        c.file("vendor/jdatasrc-tj.c");
    }

    let with_nasm = cfg!(feature = "nasm_simd") && nasm_supported();

    #[cfg(feature = "nasm_simd")]
    {
        if with_nasm {
            c.include(vendor.join("simd"));
            jconfig_h.write_all(b"#define WITH_SIMD 1\n").unwrap();

            // cfg!(target_arch) doesn't work for cross-compiling.
            let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
            let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

            match target_arch.as_str() {
                "x86_64" => {c.file("vendor/simd/jsimd_x86_64.c");},
                "x86" => {c.file("vendor/simd/jsimd_i386.c");},
                "mips" => {c.file("vendor/simd/jsimd_mips.c");},
                "powerpc" | "powerpc64" => {c.file("vendor/simd/jsimd_powerpc.c");},
                "arm" => {c.file("vendor/simd/jsimd_arm.c");},
                "aarch64" => {c.file("vendor/simd/jsimd_arm64.c");},
                _ => {},
            };

            build_nasm(&vendor, &target_arch, &target_os);
        }
    }
    drop(jconfig_h); // close the file

    if !with_nasm {
        c.file("vendor/jsimd_none.c");
    }

    c.compile(&format!("mozjpeg{}", abi));
}

fn nasm_supported() -> bool {
    match process::Command::new("nasm").arg("-v").output() {
        Err(e) => {
            println!("cargo:warning=NASM not installed. Mozjpeg's SIMD won't be enabled: {}", e);
            false
        },
        Ok(out) => {
            let ver = String::from_utf8_lossy(&out.stdout);
            if ver.contains("NASM version 0.") {
                println!("cargo:warning=Installed NASM is outdated and useless. Mozjpeg's SIMD won't be enabled: {}", ver);
                false
            } else {
                true
            }
        }
    }
}

#[cfg(feature = "nasm_simd")]
fn build_nasm(vendor_dir: &Path, target_arch: &str, target_os: &str) {
    let mut n = nasm_rs::Build::new();

    n.include(vendor_dir.join("simd"));
    n.include(vendor_dir.join("win"));
    if std::env::var("PROFILE").map(|s| "debug" == s).unwrap_or(false) {
        n.debug(true);
    }

    n.define("PIC", None); // Rust always uses -fPIC

    match (target_os, target_arch.ends_with("64")) {
        ("windows", false) => n.define("WIN32", None),
        ("windows", true) => n.define("WIN64", None),
        ("macos", _) | ("ios", _) => n.define("MACHO", None),
        _ => n.define("ELF", None),
    };

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

    let files: &[_] = if target_arch == "x86_64" {
        n.define("__x86_64__", None);
        &x86_64
    } else if target_arch == "x86" {
        &x86
    } else {
        panic!("The mozjpeg-sys SIMD build script is incomplete for this platform");
    };

    for file in files {
        n.file(file);
    }

    let name = if cfg!(target_env = "msvc") {
        "mozjpegsimd.lib"
    } else {
        "libmozjpegsimd.a"
    };

    n.compile(name);
    println!("cargo:rustc-link-lib=static=mozjpegsimd");
}
