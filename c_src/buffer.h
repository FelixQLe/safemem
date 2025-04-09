#ifndef BUFFER_H
#define BUFFER_H

#include <stddef.h>

typedef struct {
    char* data;
    size_t size;
    size_t capacity;
} Buffer;

Buffer* buffer_new(size_t capacity);
void buffer_append(Buffer* buf, const char* str);
void buffer_free(Buffer* buf);

#endif