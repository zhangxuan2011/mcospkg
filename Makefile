.PHONY: clean install prepare uninstall

# Define Vars
PYTHON_FILES = mcospkg.py mcospkg-mirror.py
OUTPUT_DIR = output
INSTALL_DIR = /data/data/com.termux/files/usr/local/bin
PYTHON_INTERP = python
PIP_NAME = pip

# Default target
all: prepare $(PYTHON_FILES:.py=)

# compile target
%: %.py
	@echo "==========Build=========="
	@echo "Generating Executable files(using nuitka)..."
	nuitka --remove-output --onefile --output-dir=$(OUTPUT_DIR) --output-filename=$(basename $<) $<
	@echo "Complete"

# install target
install:
	@echo "==========Install=========="
	@echo "Installing mcospkg..."
	mkdir -p $(OUTPUT_DIR)
	cp $(OUTPUT_DIR)/* $(INSTALL_DIR)
	@echo "Complete."

# clean target
clean:
	rm -rf $(OUTPUT_DIR)
	@echo "Cleaned"

# prepare essential modules
prepare:
	@echo "==========Prepare=========="
	@echo "Installing essential modules..."
	$(PIP_NAME) install rich requests nuitka
	@echo "Complete."

uninstall:
	cd $(INSTALL_DIR)
	rm -rf $(basename $(PYTHON_FILES))
