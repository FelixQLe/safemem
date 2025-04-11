#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "buffer.h"

// Initialize a new buffer with given capacity
Buffer* buffer_init(size_t initial_capacity) {
    Buffer* buffer = (Buffer*)malloc(sizeof(Buffer));
    if (!buffer) {
        fprintf(stderr, "Failed to allocate Buffer struture\n");
        return NULL;
    }

    buffer->data = (char*)malloc(initial_capacity);
    if (!buffer->data) {
        fprintf(stderr, "Failed to allocate buffer data\n");
        free(buffer);
        return NULL;
    }

    buffer->size = 0;
    buffer->capacity = initial_capacity;

    return buffer;

}

// VULNERABLE: Append data to buffer WITHOUT bounds checking
// This can cause buffer overflow
void buffer_append(Buffer* buffer, const char* data, size_t length) {
    // Vulnerability: No capacity check before copying data
    memcpy(buffer->data + buffer->size, data, length);
    buffer->size += length;
    // This will cause buffer overflow when wrting beyond allocated memory

}

// Get current buffer size
size_t buffer_size(Buffer* buffer) {
    return buffer->size;
}

// Get buffer data
char* buffer_data(Buffer* buffer) {
    return buffer->data;
}

// VULNERABLE: Free buffer with no safeguards against double-free
void buffer_free(Buffer* buffer) {
    // Vulnerability: No NULL check after free
    // This allows double-free vulerabilities
    free(buffer->data);
    free(buffer);
    // not setting pointers to NULL (buffer->data = NULL) after free allows use-after-free
}

// Print buffer contents (helper for debugging)
void buffer_print(Buffer* buffer) {
    printf("Buffer (size=%zu, capacity=%zu):", buffer->size, buffer->capacity);
    for (size_t i = 0; i < buffer->size; i++) {
        printf("%c", buffer->data[i]);
    }
    printf("\n");
}