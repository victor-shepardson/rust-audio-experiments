rust-audio-experiments
======================

real time audio in rust (0.9-pre)

sine.rs: test tone on default output device

sine_tasks.rs: simple 2-channel sine wave synth demonstrating rust-portaudio with JACK and multiple tasks

ringmod.rs: like sine_tasks, but ring modulates a stereo input signal

#Build

git clone

build the latest rust: https://github.com/mozilla/rust
(last compiled against 9fc48061d7ffa6a0839daf6577091b2455f785ec)

build the latest PortAudio (the "stable" release won't work): http://portaudio.com/download.html

build rust-portaudio: https://github.com/JeremyLetang/rust-portaudio
(last compiled against https://github.com/victor-shepardson/rust-portaudio)

Makefile is set up for linux x86_64 and assumes rust-portaudio is in the home directory. will need changes to link portaudio on other platforms

make

#Run

install JACK: http://jackaudio.org/download

configure and start JACK server

./ringmod

#Notes

tested on Ubuntu 13.10 x86_64

watch out for this bug in PortAudio: https://www.assembla.com/spaces/portaudio/tickets/81#/activity/ticket: JACK streams must have exactly 0 or 2 input channels and exactly 0 or 2 output channels.

to get interactive latency, you may need to configure the "Frames/Period" parameter for the JACK server. Minimum output latency for PortAudio w/ JACK is three times JACK buffer length