extern crate rand;
extern crate sort;

use std::env::args;
use rand::prelude::*;

mod player;
use player::play;

mod graphics;
use graphics::{ListVisualization,ListVisualizationWindow};

use sort::*;
use sort::bubblesort::*;
use sort::selectionsort::*;
use sort::quicksort::*;
use sort::quicksort2::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const MARGIN: usize = 2;

fn parse_args(args: Vec<String>) -> Result<(&'static Fn(&mut List<u32>),Vec<u32>),&'static str> {
    // Select sorting algorithm
    let sort_fn: &Fn(&mut List<u32>) = match args.get(1) {
        Some(arg) => match arg.as_str() {
            "bubblesort" => &bubblesort,
            "selectionsort" => &selectionsort,
            "quicksort" => &quicksort,
            "quicksort2" => &quicksort2,
            _ => return Err("Unknown sorting algorithm in first parameter"),
        }
        None => return Err("Missing first parameter: Sorting algorithm"),
    };
    // Generate data
    if let Some(arg) = args.get(2) {
        if let Ok(datalength) = arg.parse::<usize>() {
            let mut data = Vec::with_capacity(datalength);
            for _ in 0..datalength {
                data.push(random::<u32>());
            }
            Ok((sort_fn, data))
        } else {
            Err("Invalid second parameter: Cannot parse data size as number")
        }
    } else {
        Err("Missing second parameter: Data size")
    }
}

fn main() {
    // Data
    let sort_fn;
    let data;
    // Parse args
    match parse_args(args().collect()) {
        Ok((func, dat)) => {
            sort_fn = func;
            data = dat;
        },
        Err(msg) => {
            panic!(format!("Error parsing arguments -> {}", msg));
        }
    }
    // Init
    let visualization = ListVisualization::autogenerate(&data, WIDTH, HEIGHT, MARGIN);
    let window = ListVisualizationWindow::new(visualization);
    // Run
    play(data, window);
}
