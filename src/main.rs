use std::process;

use rand::prelude::*;
use clap::{Arg,App};

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
        eprintln!($($arg)*);
        process::exit(1);
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
        .arg(Arg::with_name("rand")
            .short("r")
            .long("rand")
            .value_name("COUNT")
            .help("Count of random elements to sort")
            .required(true))
        .get_matches();
    let sort_fn_str = matches.value_of("sort").unwrap();
    let sort_fn = match get_sort_fn(sort_fn_str) {
        Some(fptr) => fptr,
        None => error!("error: Sorting function \"{}\" not found", sort_fn_str),
    };
    let rand_count_str = matches.value_of("rand").unwrap();
    let rand_count = match rand_count_str.parse::<usize>() {
        Ok(n) => n,
        Err(_) => error!("error: Cannot parse \"{}\" as unsigned number", rand_count_str),
    };
    let mut data = Vec::with_capacity(rand_count);
    for _ in 0..rand_count {
        data.push(random::<u32>());
    }
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
