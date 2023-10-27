cmake_minimum_required(VERSION 3.0.0)
project(fmod CXX)
add_library(fmod INTERFACE)
target_include_directories(fmod INTERFACE include include/fmod)
if(WIN32)
  target_link_libraries(
    fmod INTERFACE ${CMAKE_CURRENT_LIST_DIR}/external/fmod/windows/fmod_vc.lib)
endif()
if(ANDROID)
  target_link_libraries(
    fmod INTERFACE ${CMAKE_CURRENT_LIST_DIR}/external/fmod/android/libfmod.so)
endif()
