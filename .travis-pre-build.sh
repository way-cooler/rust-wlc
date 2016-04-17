#!/bin/sh
# Stop on errors, be verbose
set -ex

# Clone wlc
git clone "https://github.com/Cloudef/wlc.git"

# From wlc's README
cd wlc

git submodule update --init --recursive # - initialize and fetch submodules
mkdir target && cd target               # - create build target directory
cmake -DCMAKE_BUILD_TYPE=Upstream ..    # - run CMake
make                                    # - compile

cd ..


