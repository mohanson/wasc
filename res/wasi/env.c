#include <stdlib.h>
#include <string.h>

int main(void) {
    char *env = getenv("FOO");
    if (env == NULL) {
        return 1;
    }
    if (strcmp(env, "BAR") != 0) {
        return 2;
    }
    return 0;
}

