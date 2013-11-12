#[feature(globs)];
extern mod portaudio;

use portaudio::*;
use std::num::*;
use std::f32;
use std::comm::*;
use std::rt::io::timer;

fn main() -> () {
    let bufsize = 1024;
    let (port, chan) = stream::<~[f32]>();

    println!("Portaudio init error : {:s}", pa::get_error_text(pa::initialize()));
    let def_output = pa::get_default_output_device();
    let info_output = pa::get_device_info(def_output).unwrap();
    println!("Default output device info :");
    println!("version : {:d}", info_output.struct_version);
    println!("name : {:s}", info_output.name);
    println!("max output channels : {:d}", info_output.max_output_channels);
    println!("max output channels : {:d}", info_output.max_output_channels);
    println!("default sample rate : {:f}", info_output.default_sample_rate);

    let isr = 1.0 / info_output.default_sample_rate as f32;

    do spawn {
        let mut phase = 0.0;
        loop{
            let mut buf:~[f32] = ~[];
            buf.reserve(bufsize);
            buf.grow_fn(bufsize, |i|{
                    phase += f32::consts::PI * 440.0 * isr;
                    sin(phase)
                });
            chan.send(buf);
        }
    }

    do spawn {
        let stream_params_out = types::PaStreamParameters {
            device : def_output,
            channel_count : 1,
            sample_format : types::PaFloat32,
            suggested_latency : pa::get_device_info(def_output).unwrap().default_low_output_latency
        };
        
        let mut stream : pa::PaStream<f32> = pa::PaStream::new(types::PaFloat32);

        let mut err= stream.open(None, Some(&stream_params_out), 44100., 1024, types::PaClipOff);
        println!("Portaudio Open error : {:s}", pa::get_error_text(err));
        
        err = stream.start();
        println!("Portaudio Start error : {:s}", pa::get_error_text(err));
       
        let mut phase = 0.0;   
        loop{
            /*let mut test = stream.get_stream_write_available();
            while test == 0 {
            test = stream.get_stream_write_available();
        }*/
            stream.write(port.recv(), bufsize as u32);
            //stress test
            timer::sleep(20);
        }       
    }
   
}