OUT_DIR?=/tmp/mozjpeg-build
CARGO_MANIFEST_DIR?=$(PWD)
MOZDIR=$(CARGO_MANIFEST_DIR)/vendor
CFLAGS?=-O3 -fPIC
CONFIGOPTIONS=--host="$(HOST)" --build="$(TARGET)" --enable-static --disable-shared --without-arith-enc --without-arith-dec --without-java --without-turbojpeg CFLAGS="$(CFLAGS)"

all: $(OUT_DIR)/lib/libjpeg.a
	@echo "cargo:rustc-flags=-l static=jpeg -L native=$(OUT_DIR)/lib"

$(OUT_DIR)/lib/libjpeg.a: $(OUT_DIR)/Makefile
	$(MAKE) -C $(OUT_DIR) install

$(OUT_DIR):
	mkdir -p $@

$(OUT_DIR)/Makefile: $(OUT_DIR) $(MOZDIR)/configure
	( cd $(OUT_DIR) && $(MOZDIR)/configure --prefix="$(OUT_DIR)" $(CONFIGOPTIONS) )

$(MOZDIR)/configure: $(MOZDIR)/configure.ac $(MOZDIR)/ltmain.sh
	( cd $(MOZDIR) && autoreconf -i ) && touch "$@"

$(MOZDIR)/ltmain.sh: $(MOZDIR)/configure.ac
	( cd $(MOZDIR) && glibtoolize ) && touch "$@"

$(MOZDIR)/configure.ac:
	git submodule update

clean:
	-rm -rf $(OUT_DIR)

.PHONY: all clean
