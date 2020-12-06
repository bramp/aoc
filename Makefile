.SUFFIXES:
MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --no-builtin-variables

CC=/usr/bin/clang++
CFLAGS=-std=c++17 -g -Wall # Debug
LDFLAGS=--stdlib=libc++

OUTDIR=bin

.PHONY: clean all
.SECONDARY: bin/common.o

clean:
	-rm -r $(OUTDIR)/*

all: $(shell find . -name "[0-9]*.cpp" | sed s_\./_bin/_ | sed s_.cpp__)

$(OUTDIR)/%.o: %.cpp Makefile
	$(CC) -c $(CFLAGS) $< -o $@

$(OUTDIR)/%: %.cpp Makefile
	$(CC) $(LDFLAGS) $(CFLAGS) $< -o $@

