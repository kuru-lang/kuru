#include <stdio.h>
#include <stdint.h>
#include <string.h>

/// A 32 kb stream (up to 1/4 second of audio at Uint16 48000, or Uint8 Grayscale 128x128).
///
/// A Stream is meant to be shared between two threads.  The master thread spawns the slave thead,
/// and gives it this struct.  The master thread cannot read any of buffer until the slave thread
/// increments the `slave` index.  The indices go back around in a circle.
typedef struct Stream {
    volatile uint32_t master; // 4 bytes (multiple threads may access this data)
    volatile uint32_t slave; // 4 bytes (multiple threads may access this data)
    uint8_t buffer[32768 - 8]; // 32760 bytes (protected by master and slave indices)
} Stream;

Stream stream_new(void) {
    Stream stream;
    stream.start = 0;
    stream.end = 0;
    memset(stream.buffer, 0, 32760); 
    return stream;
}

Stream stream_write(void) {
}

void info(void) {
    flockfile(stdout);
    putc_unlocked('c', stdout);
    putc_unlocked('\n', stdout);
    funlockfile(stdout);
}

int main(int argc, char* argv[]) {
    Stream stream = stream_new;
}
