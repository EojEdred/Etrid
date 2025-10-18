# Makefile for etrcpp - ËTRID C++ CLI
# Alternative to CMake for systems without CMake installed

# Compiler and flags
CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -O2 -I./include
LDFLAGS = -lcurl

# Source files
SOURCES = src/etrcpp.cpp src/rpc_client.cpp src/commands.cpp
OBJECTS = $(SOURCES:.cpp=.o)
TARGET = etrcpp

# nlohmann_json - header-only library
# Download if not present
JSON_HEADER = include/nlohmann/json.hpp
JSON_URL = https://github.com/nlohmann/json/releases/download/v3.11.3/json.hpp

# Default target
all: check_json $(TARGET)

# Check and download nlohmann_json if needed
check_json:
	@if [ ! -f "$(JSON_HEADER)" ]; then \
		echo "Downloading nlohmann_json header..."; \
		mkdir -p include/nlohmann; \
		curl -L -o $(JSON_HEADER) $(JSON_URL); \
	fi

# Build target
$(TARGET): $(OBJECTS)
	$(CXX) $(OBJECTS) -o $(TARGET) $(LDFLAGS)
	@echo "Build complete: $(TARGET)"

# Compile source files
%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Install target
install: $(TARGET)
	install -m 0755 $(TARGET) /usr/local/bin/

# Uninstall target
uninstall:
	rm -f /usr/local/bin/$(TARGET)

# Clean build artifacts
clean:
	rm -f $(OBJECTS) $(TARGET)
	@echo "Clean complete"

# Deep clean (including downloaded headers)
distclean: clean
	rm -rf include/nlohmann
	@echo "Deep clean complete"

# Help target
help:
	@echo "ËTRID C++ CLI (etrcpp) Makefile"
	@echo ""
	@echo "Usage:"
	@echo "  make          - Build etrcpp"
	@echo "  make install  - Install to /usr/local/bin (requires sudo)"
	@echo "  make clean    - Remove build artifacts"
	@echo "  make distclean- Remove build artifacts and downloaded headers"
	@echo "  make help     - Show this help message"
	@echo ""
	@echo "Requirements:"
	@echo "  - C++17 compatible compiler (g++, clang++)"
	@echo "  - libcurl development libraries"
	@echo "  - Internet connection (for first build, to download nlohmann_json)"

# Phony targets
.PHONY: all check_json install uninstall clean distclean help
