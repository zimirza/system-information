#include <stdio.h>
#include <stdlib.h>
#include <sys/utsname.h>
#include <unistd.h>

char* get_kernel() {
    char* kernel = malloc(sizeof(char));
    struct utsname buffer;

    if (uname(&buffer) != 0) {
        return "(no kernel)";
    }

    snprintf(kernel, sizeof(kernel) * 4, "%s %s", buffer.sysname, buffer.release);

    return kernel;
}

char* get_hostname() {
    static char hostname[128];
    if (gethostname(hostname, 128) == 0) {
        hostname[128 - 1] = '\0';
        return hostname;
    }
    return "(no hostname)";
}

int main(void) {
    char* hostname = get_hostname();
    char* kernel = get_kernel();

    printf("%s@%s\n", getenv("USER"), hostname);
    printf("%s\n", kernel);

    free(kernel);

    return 0;
}