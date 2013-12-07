CC = rustc
ARCH = x86_64
PAPATH = ~/rust-portaudio
CFLAGS = -L $(PAPATH)/build/$(ARCH)-unknown-linux-gnu/portaudio/ --link-args -lportaudio

%: %.rs
	$(CC) $< $(CFLAGS)

all: sine_tasks ringmod

clean:
	rm sine_tasks ringmod

