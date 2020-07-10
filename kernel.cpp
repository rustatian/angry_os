#include <stddef.h>
#include <stdint-gcc.h>


#if defined(__linux__)
#error "You are not using a cross-compiler, you will most certainly run into trouble"
#endif

#if !defined(__i386__)
#error "This tutorial needs to be compiled with a ix86-elf compiler"
#endif

/* Hardware text mode color constants. */
enum vgaColor {
    VgaColorBlack [[maybe_unused]] = 0,
    VgaColorBlue [[maybe_unused]] = 1,
    VgaColorGreen [[maybe_unused]] = 2,
    VgaColorCyan [[maybe_unused]] = 3,
    VgaColorRed [[maybe_unused]] = 4,
    VgaColorMagenta [[maybe_unused]] = 5,
    VgaColorBrown [[maybe_unused]] = 6,
    VgaColorLightGrey [[maybe_unused]] = 7,
    VgaColorDarkGrey [[maybe_unused]] = 8,
    VgaColorLightBlue [[maybe_unused]] = 9,
    VgaColorLightGreen [[maybe_unused]] = 10,
    VgaColorLightCyan [[maybe_unused]] = 11,
    VgaColorLightRed [[maybe_unused]] = 12,
    VgaColorLightMagenta [[maybe_unused]] = 13,
    VgaColorLightBrown [[maybe_unused]] = 14,
    VgaColorWhite [[maybe_unused]] = 15,
};

static inline uint8_t vgaEntryColor(enum vgaColor fg, enum vgaColor bg) {
    return fg | bg << 4;
}

static inline uint16_t vgaEntry(unsigned char uc, uint8_t color) {
    return (uint16_t) uc | (uint16_t) color << 8;
}

size_t strlen(const char *str) {
    size_t len = 0;
    while (str[len])
        len++;
    return len;
}

static const size_t vgaWidth = 80;
static const size_t vgaHeight = 25;

size_t terminalRow;
size_t terminalColumn;
uint8_t terminalColor;
uint16_t *terminalBuffer;

void terminalInitialize() {
    terminalRow = 0;
    terminalColumn = 0;
    terminalColor = vgaEntryColor(VgaColorLightGrey, VgaColorBlack);
    terminalBuffer = (uint16_t *) 0xB8000;
    for (size_t y = 0; y < vgaHeight; y++) {
        for (size_t x = 0; x < vgaWidth; x++) {
            const size_t index = y * vgaWidth + x;
            terminalBuffer[index] = vgaEntry(' ', terminalColor);
        }
    }
}

[[maybe_unused]] void terminalSetColor(uint8_t color) {
    terminalColor = color;
}

void terminalPutEntryAt(char c, uint8_t color, size_t x, size_t y) {
    const size_t index = y * vgaWidth + x;
    terminalBuffer[index] = vgaEntry(c, color);
}

void terminalPutNewLine() {
    ++terminalRow;
    terminalColumn = 0;
}


void terminalPutChar(char c) {
    if (c == '\n') {
        terminalPutNewLine();
        return;
    }
    terminalPutEntryAt(c, terminalColor, terminalColumn, terminalRow);
    if (++terminalColumn == vgaWidth) {
        terminalColumn = 0;
        if (++terminalRow == vgaHeight)
            terminalRow = 0;
    }
}


void terminalWrite(const char *data, size_t size) {
    for (size_t i = 0; i < size; i++) {
        terminalPutChar(data[i]);
    }
}

void terminalWriteString(const char *data) {
    terminalWrite(data, strlen(data));
}


extern "C" void kmain(void) {
    /* Initialize terminal interface */
    terminalInitialize();


    /* Newline support is left as an exercise. */
    terminalWriteString("Hello, Angry OS!!!\nSee you later!");
}