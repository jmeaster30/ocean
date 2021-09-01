#pragma once

//so we don't go including these everywhere when we don't want to print messages
#ifdef DEBUG_MESSAGES
#include <iostream>
#include <stdarg.h>
#include <stdio.h>
#endif

template<typename T>
inline void debug(T message) {
#ifdef DEBUG_MESSAGES
  std::cout << message << std::endl;
#endif
}

inline void debugf(const char* format...) {
#ifdef DEBUG_MESSAGES
  va_list argptr;
  va_start(argptr, format);
  
  vprintf(format, argptr);

  va_end(argptr);
#endif
}

template<typename T>
inline void tdebug(T message) {
#ifdef DEBUG_TOKENS
  std::cout << message << std::endl;
#endif
}

inline void tdebugf(const char* format...) {
#ifdef DEBUG_TOKENS
  va_list argptr;
  va_start(argptr, format);
  
  vprintf(format, argptr);

  va_end(argptr);
#endif
}

template<typename T>
inline void adebug(T message) {
#ifdef DEBUG_AST
  std::cout << message << std::endl;
#endif
}

inline void adebugf(const char* format...) {
#ifdef DEBUG_AST
  va_list argptr;
  va_start(argptr, format);
  
  vprintf(format, argptr);

  va_end(argptr);
#endif
}