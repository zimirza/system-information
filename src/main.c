#include <unistd.h>

#include "hostname.h"
#include "kernel.h"
#include "utils.h"

#include "main.h"

void main(void) {
    char msg[] = "System Information\n";
    print(msg);
    end();
}