#[feature(globs)];
extern mod portaudio;
extern mod extra;

use portaudio::*;
use std::num::*;
use std::f32;
use std::comm::*;
use std::io::timer;
use extra::arc;

fn main() -> () {
    //set sample format
    type Sample = f32;
    let sampleFormat = types::PaFloat32;
    let pi = f32::consts::PI;

    let channels = 2u;
    let bufframes = 1024u;
    let bufsamples = bufframes*channels;
    let max_pending_buffers: uint = 2;

    let (port, chan) = stream::<~[Sample]>();

    let pending_buffers = 0u;
    let pending_buffers = arc::RWArc::new(pending_buffers);

    println!("Portaudio init error : {:s}", pa::get_error_text(pa::initialize()));
    println!("--------------------");
/*    let def_output = pa::get_default_output_device();
    let info_output = pa::get_device_info(def_output).unwrap();
    println!("Default output device info :");
    println!("version : {:d}", info_output.struct_version);
    println!("name : {:s}", info_output.name);
    println!("max input channels : {:d}", info_output.max_input_channels);
    println!("max output channels : {:d}", info_output.max_output_channels);
    println!("default sample rate : {:f}", info_output.default_sample_rate);
    println!("--------------------");*/
    println!("Host count: {:d}", pa::get_host_api_count());
    let jack_idx = pa::host_api_type_id_to_host_api_index(types::PaJACK);
    let host_info = pa::get_host_api_info(jack_idx);
    match host_info {
        Some(info) => {
            println!("JACK info :");
            println!("version : {:d}", info.struct_version);
            println!("host_type: {:d}", info.host_type as i32);
            println!("name : {:s}", info.name);
            println!("device count : {:d}", info.device_count);
            let mut i = info.device_count;
            while i>0 {
                i-=1;
                let dev_idx = pa::host_api_device_index_to_device_index(jack_idx, i);
                let dev_info = pa::get_device_info(dev_idx).unwrap();
                println!("JACK device {:d} info :", i);
                println!("version : {:d}", dev_info.struct_version);
                println!("name : {:s}", dev_info.name);
                println!("max input channels : {:d}", dev_info.max_input_channels);
                println!("max output channels : {:d}", dev_info.max_output_channels);
                println!("default sample rate : {:f}", dev_info.default_sample_rate);
            }
        }
        _ => { println!("JACK host not found"); }
    }
    println!("--------------------");

    let jack_dev = 0;
    println!("using jack device {}", jack_dev);
    let dev_idx = pa::host_api_device_index_to_device_index(jack_idx, jack_dev);
    let dev_info = pa::get_device_info(dev_idx).unwrap();
    
    let latency_in = dev_info.default_low_input_latency;
    let latency_out = dev_info.default_low_output_latency;
    let sample_rate = dev_info.default_sample_rate;
    println!("latency in: {}, out: {}; sample rate: {}", latency_in, latency_out, sample_rate);

    let isr = 1.0 / sample_rate as Sample;
    let buf_ms = (isr * bufframes as Sample * 1000 as Sample) as u64;
    println!("buf_ms = {}", buf_ms);


    //SYNTH TASK
    let synth_pending_buffers = pending_buffers.clone();
    do spawn {
        let mut phase = 0.0;
        loop{
            let mut buf:~[Sample] = ~[];
            buf.reserve(bufsamples);
            buf.grow_fn(bufsamples, |i|{
                    if phase > 1.0 {
                        phase -= 1.0;
                    }
                    if phase < 0.0 {
                        phase += 1.0;
                    }
                    match i%2 {
                        0 => { 
                            phase += 440.0 * isr;
                            sin(2.0*pi*phase)
                        }
                        1 => {
                            sin(4.0*pi*phase)
                        }
                        _ => {0.0}
                    }
                });
            while(max_pending_buffers <= synth_pending_buffers.read(|count| {*count})) { 
                timer::sleep(buf_ms);
            }
            chan.send(buf);
            synth_pending_buffers.write(|count| {
                *count+=1;
            });
        }
    }

    //OUTPUT TASK
    let output_pending_buffers =pending_buffers.clone();
    do spawn {
        let stream_params_in = types::PaStreamParameters {
            device : dev_idx,
            channel_count : channels as i32,
            sample_format : sampleFormat,
            suggested_latency : latency_in
        };

        let stream_params_out = types::PaStreamParameters {
            device : dev_idx,
            channel_count : channels as i32,
            sample_format : sampleFormat,
            suggested_latency : latency_out
        };
        
        let mut stream : pa::PaStream<Sample> = pa::PaStream::new(sampleFormat);

        let mut err= stream.open(None, Some(&stream_params_out), sample_rate, bufframes as u32, types::PaClipOff);
        println!("Portaudio Open error : {:s}", pa::get_error_text(err));
        
        err = stream.start();
        println!("start executed");
        println!("Portaudio Start error : {:s}", pa::get_error_text(err));
       
        loop{
            stream.write(port.recv(), bufframes as u32);
            output_pending_buffers.write( |count| {
                *count-=1;
            });
            //stress test
            //timer::sleep(20);
        }       
    }
   
}