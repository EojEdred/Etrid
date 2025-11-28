cmake_minimum_required(VERSION 3.15)
project(etrcpp VERSION 1.0.0 LANGUAGES CXX)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Add compiler flags
if(MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

# Find required packages
find_package(CURL REQUIRED)

# nlohmann_json - header-only library
# Option 1: Use system-installed package (if available)
find_package(nlohmann_json 3.2.0 QUIET)

if(NOT nlohmann_json_FOUND)
    # Option 2: Use FetchContent to download it
    include(FetchContent)
    FetchContent_Declare(
        json
        URL https://github.com/nlohmann/json/releases/download/v3.11.3/json.tar.xz
    )
    FetchContent_MakeAvailable(json)
endif()

# Include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${CURL_INCLUDE_DIRS}
)

# Source files
set(SOURCES
    src/etrcpp.cpp
    src/rpc_client.cpp
    src/commands.cpp
)

# Create executable
add_executable(etrcpp ${SOURCES})

# Link libraries
target_link_libraries(etrcpp
    PRIVATE
    CURL::libcurl
    nlohmann_json::nlohmann_json
)

# Installation
install(TARGETS etrcpp
    RUNTIME DESTINATION bin
)

# Print configuration info
message(STATUS "etrcpp CLI Configuration:")
message(STATUS "  C++ Standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "  Build Type: ${CMAKE_BUILD_TYPE}")
message(STATUS "  Install Prefix: ${CMAKE_INSTALL_PREFIX}")
