mod masks;
mod sketch;

extern crate image;


use std::time::SystemTime;
use clap::{App, Arg};
use crate::sketch::{Sketch, UserParams};


fn get_params() -> anyhow::Result<UserParams> {
    let argument_matches = App::new("shapalizer")
        .arg(Arg::with_name("resolution")
            .short("r")
            .long("resolution")
            .help("Output image resolution.")
            .takes_value(true))
        .get_matches();

    let resolution = argument_matches.value_of("resolution").unwrap_or("1920x1080");
    let resolution: Vec<u32> = resolution.split('x').into_iter().map(|s| s.parse().unwrap()).collect();

    let destination_width = resolution[0];
    let destination_height = resolution[1];

    let params = UserParams {
        destination_height,
        destination_width,
        ..UserParams::default()
    };
    Ok(params)
}


fn main() -> anyhow::Result<()> {
    let params = get_params()?;

    let image = image::open("input.jpg")?;
    println!("Successfully loaded image.");

    let mut sketch = Sketch::new(image.into_rgba8(), params);
    let number_of_cycles = 5000;
    sketch.run_cycles(number_of_cycles);

    let output = sketch.output();
    println!("Finished processing image.");
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    output.save(format!("output{:?}.jpg", timestamp))?;
    Ok(())
}
