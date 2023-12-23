#!/bin/bash

# Clean up
rm -Rv ./node_modules/ ../static/popper/ ../static/css/ ../static/js/ ../static/scss/ ../static/icons
# Install dependencies
npm install
# Make directories
mkdir -p ../static/css/ && mkdir ../static/popper/ && mkdir ../static/js/ && mkdir ../static/scss/ && mkdir ../static/icons/
# Copy CSS
cp node_modules/bootstrap/dist/css/* ../static/css/
# Copy PopperJS
cp node_modules/@popperjs/core/dist/umd/popper.*js ../static/popper && cp node_modules/@popperjs/core/dist/umd/popper.*map ../static/popper
# Copy Bootstrap JS
cp node_modules/bootstrap/dist/js/* ../static/js
# Copy Bootstrap SCSS
cp -r node_modules/bootstrap/scss/* ../static/scss
# Copy Bootstrap icons
cp node_modules/bootstrap-icons/icons/* ../static/icons