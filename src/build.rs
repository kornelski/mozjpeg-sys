use std::env;
use std::fs;
use std::io::Write;
#[allow(unused_imports)]
use std::path::{Path, PathBuf};
use std::str::FromStr;

fn compiler(patched_dir: &Path, vendor: &Path) -> cc::Build {
    let mut c = cc::Build::new();
    c.include(patched_dir);
    c.include(vendor);
    c.pic(true);
    c.warnings(false);

    if let Ok(target_cpu) = env::var("TARGET_CPU") {
        c.flag_if_supported(&format!("-march={target_cpu}"));
    }

    if cfg!(feature = "unwinding") {
        c.flag_if_supported("-fexceptions");
    }

    c.define("NO_PROC_FOPEN", Some("1")); // open /proc/cpuinfo breaks my seccomp

    c
}

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let root = dunce::canonicalize(root).expect("CARGO_MANIFEST_DIR");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR"));
    let patched_dir = out_dir.join("patched");
    let vendor = root.join("vendor");
    println!("cargo:rerun-if-changed={}", vendor.display());

    let _ = fs::create_dir_all(&patched_dir);

    // cc crate needs emscripten target to use correct `ar`
    if env::var("TARGET").map_or(false, |t| t == "wasm32-unknown-unknown") {
        println!("cargo:warning=If the build fails, try using wasm32-unknown-emscripten target instead");
        eprintln!("If the build fails, try using wasm32-unknown-emscripten target instead");
    }

    if cfg!(feature = "unwinding") && env::var_os("CARGO_CFG_PANIC").as_deref() == Some("abort".as_ref()) {
        println!("cargo:warning=libjpeg will not be able to gracefully handle errors when used with panic=abort");
    }

    println!("cargo:include={}", env::join_paths([&patched_dir, &vendor]).expect("inc").to_str().expect("inc"));

    let abi = if cfg!(feature = "jpeg80_abi") {
        "80"
    } else if cfg!(feature = "jpeg70_abi") {
        "70"
    } else {
        "62"
    };
    println!("cargo:lib_version={abi}");

    let target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap_or_default();
    let pkg_version = env::var("CARGO_PKG_VERSION").expect("pkg");

    fs::write(patched_dir.join("jversion.h"), format!("
        #define JVERSION \"{pkg_version}\"
        #define JCOPYRIGHT \"Copyright (C)  The libjpeg-turbo Project, Mozilla, and many others\"
        #define JCOPYRIGHT_SHORT JCOPYRIGHT
    ")).expect("jversion");

    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    let timestamp: u64 = if let Ok(epoch) = env::var("SOURCE_DATE_EPOCH") {
        u64::from_str(epoch.as_str()).expect("Invalid SOURCE_DATE_EPOCH environment variable")
    } else {
        std::time::UNIX_EPOCH.elapsed().unwrap().as_secs()
    };

    let c8 = &mut compiler(&patched_dir, &vendor);
    c8.define("BITS_IN_JSAMPLE", Some("8"));
    let c12 = &mut compiler(&patched_dir, &vendor);
    c12.define("BITS_IN_JSAMPLE", Some("12"));

    let copy_to_patched = |source_path: &str| -> PathBuf {
        let dest = patched_dir.join(source_path);
        if !dest.exists() {
            let dest_dir = dest.parent().unwrap();
            if !dest_dir.exists() {
                std::fs::create_dir_all(dest_dir).unwrap();
            }

            let source = vendor.join(source_path);
            assert!(source.exists(), "missing vendored file '{}'", source.display());

            #[cfg(unix)]
            let res = std::os::unix::fs::symlink(&source, &dest);
            #[cfg(not(unix))]
            let res = std::fs::copy(&source, &dest);
            res.unwrap_or_else(|err| panic!("can't copy {} to {}: {err}", source.display(), dest.display()));
        }
        dest
    };

    // vendor directory is read-only, and jmorecfg.h can't be easily replaced,
    // because it's included using file-relative #include,
    // so to prioritize our version, all files need to be moved to a directory
    // with our include files.
    let build_copy = |c: &mut cc::Build, source_path: &str| {
        c.file(copy_to_patched(source_path));
    };

    for file in [
        "jcext.c", "jchuff.c", "jcinit.c", "jcmarker.c",
        "jcmaster.c", "jcomapi.c", "jcparam.c", "jcphuff.c",
        "jctrans.c", "jdatadst.c", "jdatasrc.c", "jdhuff.c",
        "jdtrans.c", "jerror.c", "jmemmgr.c", "jmemnobs.c",
    ] {
        build_copy(c8, file);
    }

    for file in [
        "jcapimin.c", "jcapistd.c", "jccoefct.c", "jccolor.c", "jcdctmgr.c",
        "jcmainct.c", "jcprepct.c", "jcsample.c", "jdapimin.c", "jdapistd.c",
        "jdcoefct.c", "jdcolor.c", "jddctmgr.c", "jdinput.c", "jdmainct.c",
        "jdmarker.c", "jdmaster.c", "jdmerge.c", "jdphuff.c", "jdpostct.c",
        "jdsample.c", "jfdctflt.c", "jfdctfst.c", "jfdctint.c", "jidctflt.c",
        "jidctfst.c", "jidctint.c", "jidctred.c", "jutils.c",
    ] {
        build_copy(c8, file);
        build_copy(c12, file);
    }

    for header in [
        "cderror.h", "cdjpeg.h", "jchuff.h", "jcmaster.h",
        "jdcoefct.h", "jdct.h", "jdhuff.h", "jdmainct.h",
        "jdmaster.h", "jdmerge.h", "jdsample.h", "jerror.h",
        "jinclude.h", "jlossls.h", "jmemsys.h", "jpeg_nbits.h", "jpegapicomp.h",
        "jpegint.h", "jpeglib.h", "jsamplecomp.h", "jsimd.h", "jsimddct.h",
        "simd/arm/align.h", "simd/arm/jchuff.h", "simd/jsimd.h",
        "simd/arm/jchuff.h", "simd/jsimd.h",
        "simd/mips64/jcsample.h", "simd/nasm/jsimdcfg.inc.h", "simd/powerpc/jcsample.h",
        "transupp.h", "turbojpeg.h"
    ] {
        copy_to_patched(header);
    }

    let mut jmorecfg = std::fs::read_to_string(vendor.join("jmorecfg.h")).expect("jmorecfg.h");
    jmorecfg = jmorecfg.split_inclusive("\n").filter(|l| {
        if l.starts_with("#error") {
            return false;
        }
        if let Some(def) = l.strip_prefix("#define ") {
            // Disable lossless (untested, unsupported, uninteresting)
            // and legacy palette quantization code
            for to_remove in ["D_LOSSLESS_", "C_LOSSLESS_", "QUANT_"] {
                if def.starts_with(to_remove) {
                    return false;
                }
            }
        }
        true
    }).collect::<String>();
    std::fs::write(patched_dir.join("jmorecfg.h"), jmorecfg).expect("jmorecfg.h");

    let mut jconfigint_h = fs::File::create(patched_dir.join("jconfigint.h")).expect("jconfint");
    writeln!(jconfigint_h, r#"
        #define BUILD "{timestamp}-mozjpeg-sys"
        #ifndef INLINE
            #if defined(__clang__) || defined(__GNUC__)
                #define INLINE inline __attribute__((always_inline))
            #elif defined(_MSC_VER)
                #define INLINE __forceinline
            #else
                #define INLINE inline
            #endif
        #endif
        #ifndef HIDDEN
            #if defined(__clang__) || defined(__GNUC__)
                #define HIDDEN  __attribute__((visibility("hidden")))
            #endif
        #endif
        #ifndef THREAD_LOCAL
            #if defined (_MSC_VER)
                #define HAVE_THREAD_LOCAL
                #define THREAD_LOCAL  __declspec(thread)
            #elif defined(__clang__) || defined(__GNUC__)
                #define HAVE_THREAD_LOCAL
                #define THREAD_LOCAL  __thread
            #else
                #define THREAD_LOCAL
            #endif
        #endif
        #define SIZEOF_SIZE_T {SIZEOF_SIZE_T}
        #ifndef HAVE_BUILTIN_CTZL
            #if defined (_MSC_VER)
                #if (SIZEOF_SIZE_T == 8)
                    #define HAVE_BITSCANFORWARD64
                #elif (SIZEOF_SIZE_T == 4)
                    #define HAVE_BITSCANFORWARD
                #endif
            #elif defined(__clang__) || defined(__GNUC__)
                #define HAVE_BUILTIN_CTZL 1
            #endif
        #endif
        #define FALLTHROUGH
        #define PACKAGE_NAME "{PACKAGE_NAME}"
        #define VERSION "{VERSION}"
        "#,
        timestamp = timestamp,
        PACKAGE_NAME = env::var("CARGO_PKG_NAME").expect("pkg"),
        VERSION = pkg_version,
        SIZEOF_SIZE_T = if target_pointer_width == "32" {4} else {8}
    ).expect("write");
    drop(jconfigint_h); // close the file

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "ios" && env::var_os("IPHONEOS_DEPLOYMENT_TARGET").is_none() {
        // thread-local storage is not supported on iOS 9
        unsafe { env::set_var("IPHONEOS_DEPLOYMENT_TARGET", "14.0") };
    }

    let mut jconfig_h = format!(r#"
        #define JPEG_LIB_VERSION {abi}
        #define LIBJPEG_TURBO_VERSION 0
        #define STDC_HEADERS 1
        #define NO_GETENV 1
        #define NO_PUTENV 1
        #define HAVE_STDLIB_H 1
        #define HAVE_UNSIGNED_CHAR 1
        #define HAVE_UNSIGNED_SHORT 1
        #define MEM_SRCDST_SUPPORTED 1
    "#);

    if cfg!(feature = "neon_intrinsics") {
        jconfig_h.push_str("#define NEON_INTRINSICS 1\n");
    }

    if cfg!(feature = "icc_io") {
        build_copy(c8, "jcicc.c");
        build_copy(c8, "jdicc.c");
    }

    if cfg!(feature = "arith_enc") {
        jconfig_h.push_str("#define C_ARITH_CODING_SUPPORTED 1\n");
        build_copy(c8, "jcarith.c");
    }
    if cfg!(feature = "arith_dec") {
        jconfig_h.push_str("#define D_ARITH_CODING_SUPPORTED 1\n");
        build_copy(c8, "jdarith.c");
    }

    std::fs::write(patched_dir.join("jconfig.h"), jconfig_h).expect("jconfig.h");

    if cfg!(feature = "arith_enc") || cfg!(feature = "arith_dec") {
        build_copy(c8, "jaricom.c");
    }

    if cfg!(feature = "jpegtran") {
        build_copy(c8, "transupp.c");
    }

    if cfg!(feature = "turbojpeg_api") {
        build_copy(c8, "turbojpeg.c");
        build_copy(c8, "jdatadst-tj.c");
        build_copy(c8, "jdatasrc-tj.c");
    }


    #[cfg(feature = "with_simd")]
    {
        // cfg!(target_arch) doesn't work for cross-compiling.
        let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("arch");
        let nasm_needed_for_arch = target_arch == "x86_64" || target_arch == "x86";
        let with_simd = target_arch != "wasm32" // no WASM-SIMD support here
            && if nasm_needed_for_arch { nasm_supported() } else { gas_supported(&c8) };

        let asm_used_for_arch = nasm_needed_for_arch || matches!(target_arch.as_str(), "arm" | "aarch64" | "mips");
        if with_simd {
            let simd_dir = vendor.join("simd");
            c8.include(&simd_dir);

            // 12-bit lacks SIMD
            c8.define("WITH_SIMD", Some("1"));

            // this is generated by cmake, mainly to check compat with intrinsics
            // but we use the older asm versions anyway
            std::fs::write(patched_dir.join("neon-compat.h"), r#"
                /* Define compiler-independent count-leading-zeros and byte-swap macros */
                #if defined(_MSC_VER) && !defined(__clang__)
                #define BUILTIN_CLZ(x)  _CountLeadingZeros(x)
                #define BUILTIN_CLZLL(x)  _CountLeadingZeros64(x)
                #define BUILTIN_BSWAP32(x)  _byteswap_ulong(x)
                #define BUILTIN_BSWAP64(x)  _byteswap_uint64(x)
                #elif defined(__clang__) || defined(__GNUC__)
                #define BUILTIN_CLZ(x)  __builtin_clz(x)
                #define BUILTIN_CLZLL(x)  __builtin_clzll(x)
                #define BUILTIN_BSWAP32(x)  __builtin_bswap32(x)
                #define BUILTIN_BSWAP64(x)  __builtin_bswap64(x)
                #else
                #error "Unknown compiler"
                #endif
                "#).unwrap();

            if target_arch == "arm" || target_arch == "aarch64" {
                c8.include(simd_dir.join("arm"));
                c8.flag_if_supported("-mfpu=neon");
                build_copy(c8, "simd/arm/jcgray-neon.c");
                build_copy(c8, "simd/arm/jcphuff-neon.c");
                build_copy(c8, "simd/arm/jcsample-neon.c");
                build_copy(c8, "simd/arm/jdmerge-neon.c");
                build_copy(c8, "simd/arm/jdsample-neon.c");
                build_copy(c8, "simd/arm/jfdctfst-neon.c");
                build_copy(c8, "simd/arm/jidctred-neon.c");
                build_copy(c8, "simd/arm/jquanti-neon.c");
            }

            match target_arch.as_str() {
                "x86_64" => {
                    c8.flag_if_supported("-msse");
                    build_copy(c8, "simd/x86_64/jsimd.c");
                },
                "x86" => {
                    c8.flag_if_supported("-msse");
                    build_copy(c8, "simd/i386/jsimd.c");
                },
                "mips" => {
                    build_copy(c8, "simd/mips/jsimd.c");
                },
                "powerpc" | "powerpc64" => {
                    c8.flag_if_supported("-maltivec");
                    build_copy(c8, "simd/powerpc/jsimd.c");
                },
                "arm" => {
                    build_copy(c8, "simd/arm/aarch32/jchuff-neon.c");
                    build_copy(c8, "simd/arm/aarch32/jsimd.c");
                    build_copy(c8, "simd/arm/jdcolor-neon.c");
                    build_copy(c8, "simd/arm/jfdctint-neon.c");
                },
                "aarch64" => {
                    build_copy(c8, "simd/arm/jidctfst-neon.c");
                    build_copy(c8, "simd/arm/aarch64/jsimd.c");
                },
                _ => {},
            }
            if asm_used_for_arch {
                if nasm_needed_for_arch {
                    #[cfg(feature = "nasm_simd")]
                    {
                        for obj in build_nasm(&root, &vendor, &out_dir, &target_arch, &target_os) {
                            c8.object(obj);
                        }
                    }
                } else {
                    build_gas(compiler(&patched_dir, &vendor), &target_arch, abi);
                };
            }
        }
    }

    c8.compile(&format!("mozjpeg{abi}"));
    c12.compile(&format!("mozjpeg{abi}_12b"));
}

#[cfg(feature = "with_simd")]
fn gas_supported(c: &cc::Build) -> bool {
    let supported = c.try_get_compiler().is_ok_and(|c| !c.is_like_msvc());
    if !supported {
        println!("cargo:warning=SIMD needs GNU Assembler, but MSVC is used");
    }
    supported
}

#[cfg(feature = "with_simd")]
fn nasm_supported() -> bool {
    if cfg!(feature = "nasm_simd") {
        match std::process::Command::new("nasm").arg("-v").output() {
            Err(e) => {
                println!("cargo:warning=NASM not installed. Mozjpeg's SIMD won't be enabled: {e}");
                false
            },
            Ok(out) => {
                let ver = String::from_utf8_lossy(&out.stdout);
                if ver.contains("NASM version 0.") {
                    println!("cargo:warning=Installed NASM is outdated and useless. Mozjpeg's SIMD won't be enabled: {ver}");
                    false
                } else {
                    true
                }
            }
        }
    } else {
        false
    }
}

#[cfg(feature = "with_simd")]
fn build_gas(mut c: cc::Build, target_arch: &str, abi: &str) {
    c.file(match target_arch {
        "arm" => "vendor/simd/arm/aarch32/jsimd_neon.S",
        "aarch64" => "vendor/simd/arm/aarch64/jsimd_neon.S",
        "mips" => "vendor/simd/mips/jsimd_dspr2.S",
        _ => {
            panic!("\"with_simd\" feature flag has been enabled in mozjpeg-sys crate on {target_arch} platform, which is does not have SIMD acceleration in MozJPEG. Disable SIMD or compile for x86/ARM/MIPS.");
        },
    });
    c.flag("-xassembler-with-cpp");

    c.compile(&format!("mozjpegsimd{abi}"));
}

#[cfg(feature = "nasm_simd")]
fn build_nasm(root: &Path, vendor_dir: &Path, out_dir: &Path, target_arch: &str, target_os: &str) -> Vec<PathBuf> {
    let mut n = nasm_rs::Build::new();
    n.out_dir(out_dir);

    if std::env::var("PROFILE").ok().map_or(false, |s| "debug" == s) {
        n.debug(true);
    }

    n.define("PIC", None); // Rust always uses -fPIC

    match (target_os, target_arch.ends_with("64")) {
        ("windows", false) => n.define("WIN32", None),
        ("windows", true) => n.define("WIN64", None),
        ("macos" | "ios", _) => n.define("MACHO", None),
        _ => n.define("ELF", None),
    };

    let arch_name = match target_arch {
        "x86" => "i386",
        "x86_64" => {
            n.define("__x86_64__", None);
            "x86_64"
        },
        _ => panic!("Bug: mozjpeg-sys build script is broken on {target_arch}"),
    };

    // these should have had .inc extension
    let dont_compile = ["jccolext-avx2.asm", "jccolext-mmx.asm", "jccolext-sse2.asm", "jcgryext-avx2.asm",
        "jcgryext-mmx.asm", "jcgryext-sse2.asm", "jdcolext-avx2.asm", "jdcolext-mmx.asm",
        "jdcolext-sse2.asm", "jdmrgext-avx2.asm", "jdmrgext-mmx.asm", "jdmrgext-sse2.asm"];

    let simd_dir = vendor_dir.join("simd");
    let simd_arch_dir = simd_dir.join(arch_name);
    n.include(&simd_arch_dir);
    n.include(simd_dir.join("nasm"));
    n.include(vendor_dir.join("win"));
    for entry in fs::read_dir(simd_arch_dir).expect("simd subdir missing") {
        let entry = entry.unwrap();
        let path = entry.path();
        let included = path.extension().map_or(false, |e| e == "asm");
        let excluded = path.file_name().map_or(true, |f| dont_compile.iter().any(|&e| e == f));
        if included && !excluded {
            n.file(path.strip_prefix(root).unwrap_or(&path));
        }
    }
    n.compile_objects().expect("NASM build failed")
}
