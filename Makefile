OUT_DIR?=/tmp
MOZDIR=vendor
CFLAGS?=-O3 -fPIC -mtune=native -march=native
CONFIGOPTIONS=--host="$(HOST)" --build="$(TARGET)" --enable-static --disable-shared --without-arith-enc --without-arith-dec --without-java --without-turbojpeg CFLAGS="$(CFLAGS)"

all: $(OUT_DIR)/lib/libjpeg.a
	@echo "cargo:rustc-flags=-l static=jpeg -L native=$(OUT_DIR)/lib"

$(OUT_DIR)/lib/libjpeg.a: $(MOZDIR)/Makefile
	$(MAKE) -C $(MOZDIR) install

$(MOZDIR)/Makefile: $(MOZDIR)/configure
	( cd $(MOZDIR) && ./configure --prefix="$(OUT_DIR)" $(CONFIGOPTIONS) )

$(MOZDIR)/configure: $(MOZDIR)/configure.ac $(MOZDIR)/ltmain.sh
	( cd $(MOZDIR) && autoreconf -i ) && touch "$@"

$(MOZDIR)/ltmain.sh: $(MOZDIR)/configure.ac
	( cd $(MOZDIR) && glibtoolize ) && touch "$@"

$(MOZDIR)/configure.ac:
	git submodule update

clean:
	-rm -rf $(MOZDIR)

.PHONY: all clean
