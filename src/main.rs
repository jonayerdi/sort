use rand::prelude::*;
use clap::{Arg,App,ArgGroup};

mod player;
use player::play;

mod graphics;
use graphics::{ListVisualization, ListVisualizationWindow};

use sort::bubblesort::*;
use sort::quicksort::*;
use sort::quicksort2::*;
use sort::selectionsort::*;
use sort::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const MARGIN: usize = 2;

macro_rules! error {
    ($($arg:tt)*) => ({
        eprint!("error: ");
        eprintln!($($arg)*);
        std::process::exit(1);
    })
}

fn get_sort_fn(name: &str) -> Option<fn(&mut List<u32>)> {
    match name {
        "bubblesort" => Some(bubblesort),
        "selectionsort" => Some(selectionsort),
        "quicksort" => Some(quicksort),
        "quicksort2" => Some(quicksort2),
        _ => None,
    }
}

fn get_data_from_file(filename: &str) -> Vec<u32> {
    use std::fs::File;
    use std::io::stdin;
    use std::io::prelude::*;
    let mut data = Vec::with_capacity(16); // Arbitrary initial capacity
    let mut file: Box<Read> = if filename == "." {
        Box::new(stdin())
    } else {
        Box::new(File::open(filename)
            .unwrap_or_else(|_| error!("Cannot open data file \"{}\"", filename)))
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|_| error!("Cannot read data file \"{}\" as UTF-8 text", filename));
    contents.lines().enumerate().for_each(|(line, text)| {
        let text = text.trim();
        match text.parse::<u32>() {
            Ok(n) => data.push(n),
            Err(_) => {
                // Display error only if the line contains non-whitespace characters
                if text.chars().fold(false, |acc, c| acc || !c.is_whitespace()) {
                    error!("Cannot parse \"{}:{}\" as unsigned integer", filename, line + 1)
                }
            },
        };
    });
    data.shrink_to_fit();
    data
}

fn get_random_data(count: usize) -> Vec<u32> {
    let mut data = Vec::with_capacity(count);
    for _ in 0..count {
        data.push(random::<u32>());
    }
    data.shrink_to_fit();
    data
}

fn parse_args() -> (fn(&mut List<u32>), Vec<u32>) {
    let matches = App::new("Sort GUI")
        .version("0.1.0")
        .author("Jon Ayerdi")
        .about("Visualize sorting algorithms")
        .arg(Arg::with_name("sort")
            .short("s")
            .long("sort")
            .value_name("ALGORITHM")
            .help("Sorting algorithm to use")
            .required(true))
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .value_name("FILE")
            .help("File containing the data to be sorted. Use \".\" as a filename to read from stdin")
            .required(false))
        .arg(Arg::with_name("rand")
            .short("r")
            .long("rand")
            .value_name("COUNT")
            .help("Count of random elements to sort")
            .required(false))
        .group(ArgGroup::with_name("data")
            .arg("file")
            .arg("rand")
            .required(true))
        .get_matches();
    // Get sort_fn
    let sort_fn_str = matches.value_of("sort").unwrap();
    let sort_fn = match get_sort_fn(sort_fn_str) {
        Some(fptr) => fptr,
        None => error!("Sorting function \"{}\" not found", sort_fn_str),
    };
    // Get data
    let data = 
        if let Some(filename) = matches.value_of("file") {
            get_data_from_file(filename)
        } else {
            // unwrap() should be safe because <file|rand> is mandatory
            let rand_count_str = matches.value_of("rand").unwrap();
            let rand_count = match rand_count_str.parse::<usize>() {
                Ok(n) => n,
                Err(_) => error!("Cannot parse \"{}\" as unsigned integer", rand_count_str),
            };
            get_random_data(rand_count)
        };
    (sort_fn, data)
}

fn main() {
    // Parse args
    let (sort_fn, data) = parse_args();
    // Init
    let visualization = ListVisualization::autogenerate(&data, WIDTH, HEIGHT, MARGIN);
    let window = ListVisualizationWindow::new(visualization);
    // Run
    play(sort_fn, data, window);
}
