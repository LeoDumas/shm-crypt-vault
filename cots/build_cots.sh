#!/bin/bash
set -e

echo "Building and installing Protobuf"

# Clone with submodules if it doesn't exist
if [ -d "protobuf" ]; then
    echo "protobuf directory already exists"
else
    git clone -b main --recursive https://github.com/protocolbuffers/protobuf.git
fi

cd protobuf

# Check for already existing build folder
# Should never occur but in case
if [ -d "build" ]; then
    echo "Cleaning old build directory..."
    rm -rf build
fi

mkdir build && cd build

cmake -Dprotobuf_BUILD_TESTS=OFF ../

cmake --build . -j$(nproc)

sudo cmake --install .
sudo ldconfig

echo "Protobuf installation complete!"