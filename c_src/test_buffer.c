#include "buffer.h"
#include <stdio.h>
#include <string.h>

void test_c_vulnerabilities() {
    printf("Testing C vulnerabilities:\n");

    // Buffer Overflow Tests
    printf("\nBuffer Overflow Tests:\n");

    // Test 1: Small Overflow
    Buffer* c_buf1 = buffer_new(4);
    const char* hello = "Hello";  // 5 chars + null > 4
    buffer_append(c_buf1, hello);
    printf("Small overflow: Appended 'Hello' to capacity=4 (may corrupt memory)\n");
    buffer_free(c_buf1);

    // Test 2: Large Overflow
    Buffer* c_buf2 = buffer_new(4);
    char big[1001];  // 1000 'A's + null
    memset(big, 'A', 1000);
    big[1000] = '\0';
    buffer_append(c_buf2, big);
    printf("Large overflow: Appended 1000 'A's to capacity=4 (wild overwrite)\n");
    buffer_free(c_buf2);
}

int main() {
    test_c_vulnerabilities();
    return 0;
}