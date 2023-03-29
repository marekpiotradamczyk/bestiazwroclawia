CC = g++
CXXFLAGS = -O3 -Wall -W -Wshadow -Wuninitialized -Wconversion -Wformat -std=c++11
TARGETS = main test

all: $(TARGETS)

sweeper: main

eval: eval.h position.o types.h

main: eval.o parser.o position.o search.o types.h

parser: parser.h

position: types.h

search: eval.o search.h

test: position.o

clean:
	rm -f *.o

distclean: clean
	rm -f $(TARGETS)
