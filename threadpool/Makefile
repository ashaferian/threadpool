CC = gcc
CFLAGS = -g -Wall -pthread -std=c11 -latomic



all: queue threadpool queue_test
	${CC} ${CFLAGS} lthread.c -o lthread

queue:
	${CC} ${CFLAGS} -fPIC -c queue_nl.c -o queue_nl.o

threadpool:
	${CC} ${CFLAGS} -fPIC -shared threadpool.c queue_nl.o -o libthreadpool.so

queue_test: queue
	${CC} ${CFLAGS} queue_test.c queue_nl.o -o queue_test

test: all queue_test
	${CC} ${CFLAGS} thread_test.c -L${PWD} -lthreadpool -o thread_test
	./thread_test

clean:
	rm -rf *~
	rm -rf *.o
	rm -rf *.so
	@rm -rf *.dSYM
	@rm -f queue_test
	@rm -f lthread
	@rm -f thread_test
