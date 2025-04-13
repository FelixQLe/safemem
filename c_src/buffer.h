#ifndef BUFFER_H
#define BUFFER_H

#include <stddef.h>

// Buffer structure to manage memory
typedef struct {
    char* data;
    size_t size;
    size_t capacity;
} Buffer;

Buffer* buffer_init(size_t initial_capacity);

// VULNERABLE: append data to buffer WITHOUT bounds checking
// This can cause buffer overflow
void buffer_append(Buffer* buffer, const char* data, size_t length);

// Get current buffer size
size_t buffer_size(Buffer* buffer);

// Get buffer capacity
size_t buffer_capacity(Buffer* buffer);

// Get buffer data
char* buffer_data(Buffer* buffer);

// Double free buffer
void buffer_double_free(Buffer* buffer);

// VULNERABLE: Free buffer with no safeguards against double-free
void buffer_free(Buffer* buffer);

// print buffer contents (helper for debugging)
void buffer_print(Buffer* buffer);

#endif // BUFFER_H
