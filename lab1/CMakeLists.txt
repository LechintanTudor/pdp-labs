cmake_minimum_required(VERSION 3.10)
project(supermarket-inventory)
set(CMAKE_CXX_STANDARD 17)

set(SHOP_SRC_FILES
    src/main.cpp
)

add_executable(shop ${SHOP_SRC_FILES})
