# Define Vars
PYTHON_FILES = mcospkg.py mcospkg-mirror.py
OUTPUT_DIR = output
INSTALL_DIR = /usr/local/bin

# Default targey
all: $(PYTHON_FILES:.py=)

# compile target
%: %.py
	nuitka --remove-output --onefile --output-dir=$(OUTPUT_DIR) $<
	mv $(OUTPUT_DIR)/$@.bin $(OUTPUT_DIR)/$@

# install parget
install: all
	cp $(OUTPUT_DIR)/* $(INSTALL_DIR)

# clean target
clean:
	rm -rf $(OUTPUT_DIR)/*