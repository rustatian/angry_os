#include <stddef.h>
#include <stdint.h>

#if defined(__linux__)
#error \
    "You are not using a cross-compiler, you will most certainly run into trouble"
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
  return fg | bg << 4u;
}

static inline uint16_t vgaEntry(unsigned char uc, uint8_t color) {
  return (uint16_t)uc | (uint16_t)color << 8u;
}

size_t strlen(const char *str) {
  size_t len = 0;
  while (str[len]) len++;
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
  terminalColor = vgaEntryColor(VgaColorGreen, VgaColorDarkGrey);
  terminalBuffer = (uint16_t *)0xB8000; // buffer for the VGA
  for (size_t y = 0; y < vgaHeight; y++) {
    for (size_t x = 0; x < vgaWidth; x++) {
      const size_t index = y * vgaWidth + x;
      terminalBuffer[index] = vgaEntry(' ', terminalColor);
    }
  }
}

[[maybe_unused]] void terminalSetColor(uint8_t color) { terminalColor = color; }

void terminalPutEntryAt(char c, uint8_t color, size_t termCol, size_t termRow) {
  /*
   * the calc is following
   * 0xB8000 is address, 0 byte is row 0 col 0
   * 0xB8000[0] will be termRow = 0, vgaWidth = 80, termCol = 0
   */
  const size_t index = termRow * vgaWidth + termCol;
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
    if (++terminalRow == vgaHeight) terminalRow = 0;
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

// convert integer to a char
char *itoa(int value, char *result, int base) {
  // check that the base if valid
  if (base < 2 || base > 36) {
    *result = '\0';
    return result;
  }

  char *ptr = result, *ptr1 = result, tmp_char;
  int tmp_value;

  do {
    tmp_value = value;
    value /= base;
    *ptr++ =
        "zyxwvutsrqponmlkjihgfedcba9876543210123456789abcdefghijklmnopqrstuvwxy"
        "z"[35 + (tmp_value - value * base)];
  } while (value);

  // Apply negative sign
  if (tmp_value < 0) *ptr++ = '-';
  *ptr-- = '\0';
  while (ptr1 < ptr) {
    tmp_char = *ptr;
    *ptr-- = *ptr1;
    *ptr1++ = tmp_char;
  }
  return result;
}

extern "C" void kmain(void) {
  /* Initialize terminal interface */
  terminalInitialize();

  terminalWriteString("Hello, Angry OS!!!\nLet's print some numbers.\n");
  for (int i = 0; i < 1000; ++i) {
    char *result;
    itoa(i, result, 10);
    terminalWriteString("Iteration: ");
    terminalWriteString(result);
    terminalWriteString("\n");
  }
}