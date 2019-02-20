extern crate rand;
extern crate sort;

mod graphics;
mod audio;

use std::thread;
use std::time::{Duration, Instant};
use rand::prelude::*;

use self::graphics::*;
use self::audio::*;

use sort::*;
use sort::bubblesort::*;
use sort::selectionsort::*;

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
    // Generate data
    let mut data = [0; 100];
    for i in 0..data.len() {
        data[i] = random::<u32>();
    }
    // Clone and sort
    let mut data2 = data.clone();
    let mut list = RecorderList::new(&mut data2);
    bubblesort(&mut list);
    let steps = list.steps;
    // Init visualization
    let margin = 2;
    let options = ListVisualizationOptions::autogenerate(&data, WIDTH, HEIGHT, margin);
    let mut fb: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window = Window::new("Sort GUI", WIDTH, HEIGHT,
        WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        }
    );
    // Init audio
    let mut have_audio = audio::try_init_audio();
    // Loop
    let mut current_state = ElementList::new(&mut data);
    let mut step_index = 0;
    let mut draw_step = 0;
    let mut step_begin = Instant::now();
    let refresh_period = Duration::from_millis(0);
    let step_period = Duration::from_millis(0);
    draw_screen(&mut fb, &options, &current_state, None);
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
            draw_screen(&mut fb, &options, &current_state, Some(update_indices));
            draw_step = step_index;
        }
        window.update_with_buffer(&fb).unwrap();
        let frame_elapsed = frame_begin.elapsed();
        if refresh_period > frame_elapsed {
            thread::sleep(refresh_period - frame_elapsed);
        }
        println!("{}", frame_begin.elapsed().subsec_millis());
    }
}
