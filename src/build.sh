set -e
cd "$CARGO_MANIFEST_DIR/mozjpeg"
test -f ./configure || autoreconf -i
test -f ./Makefile || ./configure --prefix="$OUT_DIR" --host="$HOST" --build="$TARGET" --enable-static --disable-shared --without-arith-enc --without-arith-dec --without-turbojpeg
make -j"$NUM_JOBS" install
echo "cargo:rustc-flags=-l static=jpeg -L native=$OUT_DIR/lib"
echo "cargo:libdir=$OUT_DIR/lib"
