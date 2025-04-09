#include "buffer.h"
#include <stdlib.h>
#include <string.h>

Buffer* buffer_new(size_t capacity) {
    Buffer* buf = malloc(sizeof(Buffer));
    buf->data = malloc(capacity);
    buf->size = 0;
    buf->capacity = capacity;
    return buf;
}

void buffer_append(Buffer* buf, const char* str) {
    size_t len = strlen(str);
    if (buf->size + len >= buf->capacity) {
        buf->capacity *= 2;
        buf->data = realloc(buf->data, buf->capacity);
    }
    memcpy(buf->data + buf->size, str, len);
    buf->size += len;
}

void buffer_free(Buffer* buf) {
    free(buf->data);
    free(buf);
}