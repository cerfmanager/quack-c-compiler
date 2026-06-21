CC = gcc
CFLAGS = -std=c11 -Wall -Wextra -O2

all: quack

quack: quack.c
	$(CC) $(CFLAGS) -o quack quack.c

UNAME_S := $(shell uname -s)

ifeq ($(UNAME_S),Darwin)
  ENTRY := _test_mem
  LDFLAGS_ENTRY := -Wl,-e,$(ENTRY)
else
  ENTRY := test_mem
  LDFLAGS_ENTRY := -Wl,-e,$(ENTRY)
endif

memtest: lab1_test.c
	rm -rf test
	$(CC) -include quack.c $(LDFLAGS_ENTRY) -o test lab1_test.c
	./test

clean:
	rm -f quack

.PHONY: all clean
