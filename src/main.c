#include <unistd.h>

#include "main.h"

char* get_kernel() {
    return NULL;
}

char* get_hostname() {
    return NULL;
}

void print(char* text) {
    long text_len = 0;

    char *p = text;

    while (*p != '\0') {
        text_len++;
        p++;
    }

    text_len--;

    __asm__ volatile (
        "mov $1, %%rax\n\t"
        "mov $1, %%rdi\n\t"
        "mov %[ptr], %%rsi\n\t"
        "mov %[len], %%rdx\n\t"
        "syscall\n\t"
        :
        : [ptr] "r" (text),
          [len] "r" (text_len)
        : "rax", "rdi", "rsi", "rdx"
    );
}

void end() {
    __asm__ volatile (
        "mov $60, %%rax\n\t"
        "mov $0, %%rdi\n\t"
        "syscall\n\t"
        :
        :
        : "rax", "rdi"
    );
}

void main(void) {
    char msg[] = "System Information\n";
    print(msg);
    end();
}