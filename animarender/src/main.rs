extern crate anima;

#[macro_use]
extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("AnimaRender")
        .version(crate_version!())
        .author("Mattias Cibien <mattias@mattiascibien.net>")
        .about("3D Rendering Engine")
        .arg(Arg::with_name("vsync")
                .help("Sets vsync enabled"))
        .arg(Arg::with_name("width")
                .help("The width of the rendering window")
                .value_name("WIDTH")
                .takes_value(true))
        .arg(Arg::with_name("height")
                .help("The height of the rendering window")
                .value_name("HEIGHT")
                .takes_value(true))
        .arg(Arg::with_name("scene")
                .help("The scene file to render")
                .value_name("FILE")
                .takes_value(true))
        .get_matches();

    let width = matches.value_of("width").unwrap_or("800").parse::<u32>().unwrap();
    let height = matches.value_of("height").unwrap_or("600").parse::<u32>().unwrap();

    let vsync =  matches.is_present("vsync");

    anima::render(width, height, vsync, "");
}