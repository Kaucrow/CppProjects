cmake_minimum_required(VERSION 3.27.6)

project(hello)

add_executable(${PROJECT_NAME} ./src/main.cpp)

add_subdirectory(./src/Adder)

add_subdirectory(./external/ftxui)

target_include_directories(${PROJECT_NAME} 
    PUBLIC ./src/Adder
    PUBLIC ./external/ftxui/include
)

target_link_directories(${PROJECT_NAME}
    PRIVATE ./src/Adder/
    PRIVATE ./external/ftxui/
)

target_link_libraries(${PROJECT_NAME} 
    PRIVATE adder
    PRIVATE ftxui::screen
    PRIVATE ftxui::dom
    PRIVATE ftxui::component
)