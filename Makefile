OUT_DIR?=/tmp
MOZDIR=$(OUT_DIR)/mozjpeg
CONFIGOPTIONS=--host="$(HOST)" --build="$(TARGET)" --enable-static --disable-shared --without-arith-enc --without-arith-dec --without-java --without-turbojpeg

all: $(OUT_DIR)/lib/libjpeg.a
	@echo "cargo:rustc-flags=-l static=jpeg -L native=$(OUT_DIR)/lib"

$(OUT_DIR)/lib/libjpeg.a: $(MOZDIR)/Makefile
	$(MAKE) -C $(MOZDIR) install

$(MOZDIR)/Makefile: $(MOZDIR)/configure
	( cd $(MOZDIR) && ./configure --prefix="$(OUT_DIR)" $(CONFIGOPTIONS) )

$(MOZDIR)/configure: $(MOZDIR)/configure.ac
	( cd $(MOZDIR) && autoreconf -i ) && touch "$@"

$(MOZDIR)/configure.ac:
	git clone --depth=1 https://github.com/mozilla/mozjpeg.git "$(MOZDIR)"

clean:
	-rm -rf $(MOZDIR)

.PHONY: all clean
