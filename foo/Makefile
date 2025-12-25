CC = gcc
CFLAGS = -Wall -Wextra -ggdb
SOURCE = foo.c
TARGET = build/foo
HEADER = include/essen.h

all: $(TARGET)

CLANG ?= n
GDB ?= n
RUN_CMD ?= ./$(TARGET)

ifeq ($(GDB), y)
	RUN_CMD = gdb ./$(TARGET)	
endif

ifeq ($(CLANG),y)
	CC = clang
	CFLAGS += -fsanitize=address
endif

$(TARGET): $(SOURCE) $(HEADER)
	$(CC) $(CFLAGS) $< -o $@

clean:
	rm -f $(TARGET)

.PHONY: run all clean

run: $(TARGET)
	$(RUN_CMD)
