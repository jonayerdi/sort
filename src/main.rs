extern crate rand;
extern crate sort;

use std::env::args;
use std::thread;
use std::time::{Duration, Instant};
use rand::prelude::*;

mod graphics;
use self::graphics::*;

use sort::*;
use sort::bubblesort::*;
use sort::selectionsort::*;
use sort::quicksort::*;
use sort::quicksort2::*;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn parse_args(args: Vec<String>) -> Result<(Box<Fn(&mut List<u32>)>,Vec<u32>),&'static str> {
    // Select sorting algorithm
    let sort_fn: Box<Fn(&mut List<u32>)> = match args.get(1) {
        Some(arg) => match arg.as_str() {
            "bubblesort" => Box::new(&bubblesort),
            "selectionsort" => Box::new(&selectionsort),
            "quicksort" => Box::new(&quicksort),
            "quicksort2" => Box::new(&quicksort2),
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

fn next_step(current_state: &mut ElementList, steps: &Vec<Step>, step_index: usize) {
    if step_index > 0 {
        let previous_step = &steps[step_index-1];
        current_state.operations[previous_step.indices[0]] = None;
        current_state.operations[previous_step.indices[1]] = None;
    }
    if step_index < steps.len() {
        let current_step = &steps[step_index];
        current_state.operations[current_step.indices[0]] = Some(current_step.operation);
        current_state.operations[current_step.indices[1]] = Some(current_step.operation);
        if current_step.operation == Operation::Swap {
            current_state.slice.swap(current_step.indices[0], current_step.indices[1]);
        }
    }
}

fn main() {
    // Data
    let sort_fn;
    let mut data;
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
    // Clone and sort
    let mut data2 = data.clone();
    let mut list = RecorderList::new(&mut data2);
    sort_fn(&mut list);
    let steps = list.steps;
    // Init visualization
    let margin = 2;
    let visualization_options = ListVisualizationOptions::autogenerate(&data, WIDTH, HEIGHT, margin);
    let mut window = ListVisualizationWindow::new(visualization_options);
    // Loop
    let mut current_state = ElementList::new(&mut data);
    let mut step_index = 0;
    let mut draw_step = 0;
    let mut step_begin = Instant::now();
    let refresh_period = Duration::from_millis(5);
    let step_period = Duration::from_millis(5);
    window.redraw(&current_state, None);
    while window.is_open() {
        let frame_begin = Instant::now();
        if step_begin.elapsed() > step_period && step_index <= steps.len() {
            next_step(&mut current_state, &steps, step_index);
            step_index += 1;
            step_begin = Instant::now();
        }
        if draw_step != step_index {
            let mut update_indices = Vec::with_capacity(4);
            if step_index > 1 {
                update_indices.extend(steps[step_index-2].indices.iter());
            }
            if step_index <= steps.len() {
                update_indices.extend(steps[step_index-1].indices.iter());
            }
            window.redraw(&current_state, Some(update_indices));
            draw_step = step_index;
        }
        window.update().unwrap();
        let frame_elapsed = frame_begin.elapsed();
        if refresh_period > frame_elapsed {
            thread::sleep(refresh_period - frame_elapsed);
        }
        //println!("{}", frame_begin.elapsed().subsec_millis());
    }
}
