.POSIX:

include config.mk

build:
	$(BUILD_COMMAND)

install:
	mkdir -p $(DEST_DIR)
	cp -f $(RELEASE_PATH) $(DEST_PATH)
	chmod 755 $(DEST_PATH)

uninstall:
	rm -f $(DEST_PATH)

.PHONY: build install uninstall
