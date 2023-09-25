cmake_minimum_required(VERSION 3.27.6)

project(hello)

add_executable(${PROJECT_NAME} ./src/main.cpp)

# DOWNLOAD ALL THE SUBMODULES
find_package(Git QUIET)
if(GIT_FOUND AND EXISTS "${PROJECT_SOURCE_DIR}/.git")
# Update submodules as needed
    option(GIT_SUBMODULE "Check submodules during build" ON)
    if(GIT_SUBMODULE)
        message(STATUS "Submodule update")
        execute_process(COMMAND ${GIT_EXECUTABLE} submodule update --init --recursive
                        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
                        RESULT_VARIABLE GIT_SUBMOD_RESULT)
        if(NOT GIT_SUBMOD_RESULT EQUAL "0")
            message(FATAL_ERROR "git submodule update --init failed with ${GIT_SUBMOD_RESULT}, please checkout submodules")
        endif()
    endif()
endif()

if(NOT EXISTS "${PROJECT_SOURCE_DIR}/extern/ftxui/CMakeLists.txt")
    message(FATAL_ERROR "The submodules were not downloaded! GIT_SUBMODULE was turned off or failed. Please update submodules and try again.")
endif()

add_subdirectory(./src/Adder)
add_subdirectory(./extern/ftxui)

target_include_directories(${PROJECT_NAME} 
    PUBLIC ./src/Adder
    PUBLIC ./extern/ftxui/include
)

target_link_directories(${PROJECT_NAME}
    PRIVATE ./src/Adder/
    PRIVATE ./extern/ftxui/
)

target_link_libraries(${PROJECT_NAME} 
    PRIVATE adder
    PRIVATE ftxui::screen
    PRIVATE ftxui::dom
    PRIVATE ftxui::component
)