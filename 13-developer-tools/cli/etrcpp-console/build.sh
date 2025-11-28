#!/bin/bash
# Build script for etrcpp - ËTRID C++ CLI

set -e

echo "=========================================="
echo "ËTRID C++ CLI (etrcpp) Build Script"
echo "=========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check for compiler
if ! command -v g++ &> /dev/null && ! command -v clang++ &> /dev/null; then
    echo -e "${RED}Error: No C++ compiler found (g++ or clang++ required)${NC}"
    exit 1
fi

CXX=${CXX:-g++}
echo -e "${GREEN}Using compiler: $CXX${NC}"

# Check for libcurl
if ! command -v curl-config &> /dev/null; then
    echo -e "${RED}Error: libcurl not found${NC}"
    echo "Please install libcurl:"
    echo "  macOS: brew install curl"
    echo "  Ubuntu: sudo apt-get install libcurl4-openssl-dev"
    exit 1
fi

echo -e "${GREEN}Found libcurl: $(curl-config --version)${NC}"

# Download nlohmann_json if needed
JSON_HEADER="include/nlohmann/json.hpp"
if [ ! -f "$JSON_HEADER" ]; then
    echo -e "${YELLOW}Downloading nlohmann_json header...${NC}"
    mkdir -p include/nlohmann
    curl -L -o "$JSON_HEADER" https://github.com/nlohmann/json/releases/download/v3.11.3/json.hpp
    echo -e "${GREEN}Downloaded nlohmann_json${NC}"
fi

# Build
echo ""
echo "Building etrcpp..."
echo ""

CXXFLAGS="-std=c++17 -Wall -Wextra -O2 -Iinclude"
LDFLAGS="-lcurl"

# Compile source files
$CXX $CXXFLAGS -c src/etrcpp.cpp -o src/etrcpp.o
$CXX $CXXFLAGS -c src/rpc_client.cpp -o src/rpc_client.o
$CXX $CXXFLAGS -c src/commands.cpp -o src/commands.o

# Link
$CXX src/etrcpp.o src/rpc_client.o src/commands.o -o etrcpp $LDFLAGS

echo ""
echo -e "${GREEN}=========================================="
echo "Build successful!"
echo "==========================================${NC}"
echo ""
echo "Binary location: $(pwd)/etrcpp"
echo ""
echo "To test:"
echo "  ./etrcpp -h"
echo ""
echo "To install system-wide:"
echo "  sudo install -m 0755 etrcpp /usr/local/bin/"
echo ""
