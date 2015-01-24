set -e
cd "$CARGO_MANIFEST_DIR/mozjpeg"
test -f "$OUT_DIR/autoreconfdone" || {
    autoreconf -i && touch "$OUT_DIR/autoreconfdone";
}
test -f "$OUT_DIR/confdone" || {
    ./configure --prefix="$OUT_DIR" --host="$HOST" --build="$TARGET" --enable-static --disable-shared --without-arith-enc --without-arith-dec --without-turbojpeg &&
    touch "$OUT_DIR/confdone";
}
make -j"$NUM_JOBS" install
echo "cargo:rustc-flags=-l static=jpeg -L native=$OUT_DIR/lib"
test -f $OUT_DIR/lib/libjpeg.a || {
    echo "make failed to generate lib; pwd=$PWD out=$OUT_DIR";
    ls -laR "$OUT_DIR"
    exit 1;
}
