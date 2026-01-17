# MIT License
#
# Copyright (c) 2026 - WBTek: Greg Slocum
# Division of WhiteBear Family, Inc.
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

# Project Settings
APP_NAME := fungus-test
SERVER_USER := root
SERVER_HOST := feet
SERVER_PATH := /www/slocum.net/wbtek/fungus-test

# Trunk Commands
TRUNK := trunk

all: build

# 1. Build for local testing (uses Trunk.toml settings)
build:
	$(TRUNK) build --release

# 2. Run local dev server on Red
serve:
	$(TRUNK) serve

# 3. Deploy to Feet
# This builds, then scp's everything in dist/ to the server.
# Then it ensures permissions are correct (755 for dir, 644 for files).
deploy: build
	@echo "--- Uploading to $(SERVER_HOST) ---"
	scp -r dist/* $(SERVER_USER)@$(SERVER_HOST):$(SERVER_PATH)/
	@echo "--- Done! Check https://wbtek.net/fungus-test/ ---"

perms:
	@echo "--- Fixing Permissions ---"
	ssh $(SERVER_USER)@$(SERVER_HOST) "chmod 755 $(SERVER_PATH) && chmod 644 $(SERVER_PATH)/*"
	@echo "--- Done! ---"

# 4. Clean up local build artifacts
clean:
	rm -rf dist
	cargo clean

.PHONY: all build serve deploy clean

