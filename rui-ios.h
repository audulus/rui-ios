#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct AppState AppState;

void delete_context(struct AppState *cx);

struct AppState *new_context(void);
