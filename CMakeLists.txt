cmake_minimum_required (VERSION 3.22)
project (ibus-afrim)

add_compile_options(-Wall -Wextra)

include(GNUInstallDirs)

set(LIBEXECDIR ${CMAKE_INSTALL_FULL_LIBEXECDIR})
set(PKGDATADIR ${CMAKE_INSTALL_FULL_DATADIR})
set(LOCALEDIR ${CMAKE_INSTALL_FULL_LOCALEDIR})

add_definitions(
    -DPACKAGE_NAME="${PROJECT_NAME}"
    -DGETTEXT_PACKAGE="${PROJECT_NAME}"
    -DPACKAGE_VERSION="0.0.1"
    -DPACKAGE_BUGREPORT="https://github.com/fodydev/ibus-afrim/issues"
    -DLOCALEDIR="${LOCALEDIR}"
    -DPKGDATADIR="${PKGDATADIR}"
    -DLIBEXECDIR="${LIBEXECDIR}"
)

### Require out-of-source builds
file(TO_CMAKE_PATH "${PROJECT_BINARY_DIR}/CMakeLists.txt" LOC_PATH)
if(EXISTS "${LOC_PATH}")
    message(FATAL_ERROR "You cannot build in a source directory (or any directory with a CMakeLists.txt file). Please make a build subdirectory. Feel free to remove CMakeCache.txt and CMakeFiles.")
endif()

find_package(PkgConfig)
pkg_check_modules(IBUS REQUIRED ibus-1.0)

include(FetchContent)

# Corrosion tools
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG v0.5 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)
# Adds Corrosion

# The compiled library code is here
add_subdirectory(src)

# The executable code is here
add_subdirectory(apps)


