use std::env;
use std::path::Path;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[macro_use]
extern crate serde_derive;
use hound;

mod sig;

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<sig::Wave, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() == 0 {
        // TODO: handle error
        return;
    }

    let filename = &args[1];
    println!("In file {}", filename);

    if filename == "" {
        // TODO: handle error
        return;
    }

    let path = Path::new(filename);
    println!("Path {:?}", path);

    println!("file_name {:?}", path.file_name().unwrap());
    // println!("stem {:?}", path.file_stem().unwrap());
    // println!("extension {:?}", path.extension().unwrap());

    let extension = path.extension().unwrap();
    if extension != "json" {
        // TODO: handle error
        println!("we need json!");
        return;
    }

    let signal_data = read_user_from_file(path);

    match signal_data {
        Ok(ref signal) => {
            println!("{:#?}", signal);

            let samples = sig::parse(&signal);
            let file_name = path.file_stem().unwrap().to_str().unwrap().to_owned() + ".wav";

            let spec = hound::WavSpec {
                channels: 2,
                sample_rate: signal.get_sample_rate(),
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };
            let mut writer = hound::WavWriter::create(file_name, spec).unwrap();
            for value in samples {
                writer.write_sample(value as f32).unwrap();
                writer.write_sample(value as f32).unwrap();
            }
            writer.finalize().unwrap();
        },
        Err(ref e) => println!("{:?}", &e)
    }
}
