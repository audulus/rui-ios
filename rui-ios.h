#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>


typedef struct AppState AppState;

void delete_context(struct AppState *cx);

struct AppState *new_context(void);

void render(struct AppState *state, float width, float height, float scale);

void setup_surface(struct AppState *state, void *ca_layer_ptr);

void update(struct AppState *state, float width, float height);
