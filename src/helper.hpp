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

inline void debugl() {
  std::cout << std::endl;
}

template<typename T, typename... Types>
inline void debugl(T curr, Types... next) {
#ifdef DEBUG_MESSAGES
  std::cout << curr;

  debugl(next...);
#endif
}

template<typename T>
inline void tdebug(T message) {
#if defined DEBUG_TOKENS and defined DEBUG_MESSAGES
  std::cout << message << std::endl;
#endif
}

inline void tdebugf(const char* format...) {
#if defined DEBUG_TOKENS and defined DEBUG_MESSAGES
  va_list argptr;
  va_start(argptr, format);
  
  vprintf(format, argptr);

  va_end(argptr);
#endif
}

template<typename T>
inline void adebug(T message) {
#if defined DEBUG_AST and defined DEBUG_MESSAGES
  std::cout << message << std::endl;
#endif
}

inline void adebugf(const char* format...) {
#if defined DEBUG_AST and defined DEBUG_MESSAGES
  va_list argptr;
  va_start(argptr, format);
  
  vprintf(format, argptr);

  va_end(argptr);
#endif
}