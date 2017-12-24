# - MACRO_ENSURE_OUT_OF_SOURCE_BUILD(<errorMessage>)
# MACRO_ENSURE_OUT_OF_SOURCE_BUILD(<errorMessage>)

# Copyright (c) 2006, Alexander Neundorf, <neundorf@kde.org>
#
# Redistribution and use is allowed according to the terms of the BSD license.
# For details see the accompanying COPYING-CMAKE-SCRIPTS file.

macro (MACRO_ENSURE_OUT_OF_SOURCE_BUILD _errorMessage)

   string(COMPARE EQUAL "${CMAKE_SOURCE_DIR}" "${CMAKE_BINARY_DIR}" _insource)
   if (_insource)
     message(SEND_ERROR "${_errorMessage}")
     message(FATAL_ERROR "Remove the file CMakeCache.txt in ${CMAKE_SOURCE_DIR} first.")
     # # Clean up the CMake generated files, too.  (Doesn't work?)
     # file(REMOVE ${CMAKE_SOURCE_DIR}/CMakeCache.txt)
     # file(REMOVE_RECURSE ${CMAKE_SOURCE_DIR}/CMakeFiles)
   endif (_insource)

endmacro (MACRO_ENSURE_OUT_OF_SOURCE_BUILD)
