#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

const char* get_hostname() {
    static char hostname[128];
    if (gethostname(hostname, 128) == 0) {
        hostname[128 - 1] = '\0';
        return hostname;
    }
    return "(no hostname)";
}

int main() {
    printf("%s@%s\n", getenv("USER"), get_hostname());

    return 0;
}