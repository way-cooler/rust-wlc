#!/usr/bin/sh
echo "Installing dependencies."
# Stop on errors, be verbose
set -ex

# First install libxkbcommon
echo "Installing libxkbcommon..."
echo "> apt-get update"
sudo apt-get -qq update
echo "> installing libxkbcommon"
sudo apt-get install -y libxkbcommon-dev

echo "Installing wlc..."
echo "> Cloning into wlc"
# Clone wlc
git clone "https://github.com/Cloudef/wlc.git"

# From wlc's README
cd wlc

echo "> Updating submodules"
git submodule update --init --recursive # - initialize and fetch submodules
mkdir target && cd target               # - create build target directory
echo "> Running cmake"
cmake -DCMAKE_BUILD_TYPE=Upstream ..    # - run CMake
echo "> Make"
make                                    # - compile

cd ..
echo "Dependencies installed."


