#!/bin/bash

git clone --filter=blob:none --sparse https://github.com/TomHarte/ProcessorTests tests
cd tests
git sparse-checkout add 6502/v1
mv 6502/v1/*.json .
rm -rf 6502
rm README.md
rm .gitignore
