#include "buffer.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Test functions for each type of vulnerability
void test_buffer_overflow_1() {
    printf("\n=== Buffer Overflow Test 1 ===\n");
    Buffer* buffer = buffer_init(10);
    printf("Created buffer with capacity 10\n");
    
    // Append data that fits within capacity
    buffer_append(buffer, "Hello", 5);
    buffer_print(buffer);
    
    // Overflow: Append more data than the remaining capacity
    printf("Attempting to append 10 more bytes when only 5 bytes remain...\n");
    buffer_append(buffer, " World!!!", 10);
    
    // This print may show corrupted data or crash
    printf("After overflow: ");
    buffer_print(buffer);
    
    buffer_free(buffer);
    printf("Test completed\n");
}

void test_buffer_overflow_2() {
    printf("\n=== Buffer Overflow Test 2 ===\n");
    Buffer* buffer = buffer_init(5);
    printf("Created buffer with capacity 5\n");
    
    // Massive overflow: Append data many times larger than capacity
    printf("Attempting to append 50 bytes to a 5-byte buffer...\n");
    char large_data[50];
    memset(large_data, 'X', sizeof(large_data));
    buffer_append(buffer, large_data, sizeof(large_data));
    
    // This will likely crash before reaching here
    printf("After massive overflow: ");
    buffer_print(buffer);
    
    buffer_free(buffer);
    printf("Test completed\n");
}

void test_double_free_1() {
    printf("\n=== Double Free Test 1 ===\n");
    Buffer* buffer = buffer_init(10);
    printf("Created buffer\n");
    
    buffer_append(buffer, "Test", 4);
    buffer_print(buffer);
    
    // First free (correct)
    printf("Freeing buffer first time...\n");
    buffer_free(buffer);
    
    // Second free (vulnerability)
    printf("Attempting to free buffer second time...\n");
    buffer_free(buffer);  // This will cause a double-free error
    
    printf("Test completed\n");  // We may not reach here
}

void test_double_free_2() {
    printf("\n=== Double Free Test 2 ===\n");
    Buffer* buffer = buffer_init(10);
    printf("Created buffer\n");
    
    // Free immediately
    printf("Freeing buffer first time...\n");
    buffer_free(buffer);
    
    // Wait a bit to potentially trigger different behavior
    printf("Waiting briefly...\n");
    for (volatile int i = 0; i < 1000000; i++) {}
    
    // Double free after some time
    printf("Attempting to free buffer second time after delay...\n");
    buffer_free(buffer);  // This will cause a double-free error
    
    printf("Test completed\n");  // We may not reach here
}

void test_use_after_free_1() {
    printf("\n=== Use After Free Test 1 ===\n");
    Buffer* buffer = buffer_init(10);
    printf("Created buffer\n");
    
    buffer_append(buffer, "Hello", 5);
    buffer_print(buffer);
    
    // Free the buffer
    printf("Freeing buffer...\n");
    buffer_free(buffer);
    
    // Use after free: access data
    printf("Attempting to access data after free...\n");
    char* data = buffer_data(buffer);  // Accessing freed memory
    printf("Data after free: %s\n", data);  // This might crash or show garbage
    
    printf("Test completed\n");  // We may not reach here
}

void test_use_after_free_2() {
    printf("\n=== Use After Free Test 2 ===\n");
    Buffer* buffer = buffer_init(10);
    printf("Created buffer\n");
    
    buffer_append(buffer, "Hello", 5);
    buffer_print(buffer);
    
    // Free the buffer
    printf("Freeing buffer...\n");
    buffer_free(buffer);
    
    // Use after free: try to append more data
    printf("Attempting to append data after free...\n");
    buffer_append(buffer, " World", 6);  // Writing to freed memory
    
    // Try to print after use-after-free
    printf("After use-after-free append: ");
    buffer_print(buffer);  // This will likely crash
    
    printf("Test completed\n");  // We may not reach here
}

int main() {
    printf("Starting vulnerability tests for buffer manager\n");
    printf("WARNING: These tests will crash due to intentional memory vulnerabilities\n");
    printf("Each test should be run separately by uncommenting one at a time\n\n");
    
    // Uncomment one test at a time to observe behavior
    
    // test_buffer_overflow_1();
    test_buffer_overflow_2();
    // test_double_free_1();
    // test_double_free_2();
    // test_use_after_free_1();
    // test_use_after_free_2();
    
    printf("\nAll tests completed (if you see this, some tests didn't crash as expected)\n");
    return 0;
}