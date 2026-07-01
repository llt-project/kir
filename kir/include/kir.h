#ifndef KIR
#define KIR

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Project Project;

typedef struct HProject {
  struct Project *ptr;
} HProject;

char *add(int32_t a, int32_t b);

void print(const char *s);

struct HProject project_new(const char *const *s, uintptr_t len);

#endif  /* KIR */
