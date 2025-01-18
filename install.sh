#!/bin/bash
for file in target/release/my_program*; do
	if [ -x "$file" ]; then
		sudo cp "$file" /usr/local/bin/
	fi
done
