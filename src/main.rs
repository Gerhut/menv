#[macro_use]
extern crate clap;
extern crate dotenv;
extern crate mustache;

use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{stdin, stdout};

use clap::{Arg, ArgMatches};
use dotenv::dotenv;
use mustache::{Data, MapBuilder, Template};

fn get_template(filename: &OsStr) -> Template {
    if filename == "-" {
        use std::io::Read;
        let mut string = String::new();
        stdin().lock().read_to_string(&mut string)
            .expect("Failed to read from stdio");
        mustache::compile_str(&string)
            .expect("Failed to compile template")
    } else {
        mustache::compile_path(filename)
            .expect("Failed to compile template")
    }
}

fn get_data() -> Data {
    let mut builder = MapBuilder::new();
    for (key, value) in env::vars() {
        builder = builder.insert_str(key, value);
    }
    builder.build()
}

fn output(filename: &OsStr, template: Template, data: Data) {
    if filename == "-" {
        template.render_data(&mut stdout(), &data)
            .expect("Failed to render output")
    } else {
        let mut file = File::create(filename)
            .expect("Failed to create output file");
        template.render_data(&mut file, &data)
            .expect("Failed to render output")
    }
}

fn main() {
    let matches: ArgMatches = app_from_crate!()
        .arg(Arg::with_name("dotenv")
            .help("Apply environment variables from \".env\" file")
            .long("dotenv")
            .short("d"))
        .arg(Arg::with_name("template")
            .required(true)
            .help("Template file, or \"-\" for stdin"))
        .arg(Arg::with_name("output")
            .required(true)
            .help("Rendered file, or \"-\" for stdout"))
        .get_matches();

    if matches.is_present("dotenv") {
        dotenv().expect("Failed to import dotenv");
    }

    let template = get_template(matches.value_of_os("template").unwrap());
    let data = get_data();
    output(matches.value_of_os("output").unwrap(), template, data)
}
