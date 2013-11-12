all: sine sine_tasks
sine:
	rustc sine.rs -L ~/rust-portaudio/build/x86_64-unknown-linux-gnu/portaudio/ --link-args -lportaudio
sine_tasks:
	rustc sine_tasks.rs -L ~/rust-portaudio/build/x86_64-unknown-linux-gnu/portaudio/ --link-args -lportaudio
clean:
	rm sine sine_tasks

