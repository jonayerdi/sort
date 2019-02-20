extern crate rand;
extern crate minifb;
extern crate sort;

use std::thread;
use std::time::{Duration, Instant};
use rand::prelude::*;
use minifb::{WindowOptions, Window};

use sort::*;
use sort::bubblesort::*;
use sort::selectionsort::*;

struct ListVisualizationOptions {
    margin: usize,
    element_positions: Vec<(usize,usize)>,
    unit_height: f64,
}
impl ListVisualizationOptions {
    fn autogenerate(data: &[u32], width: usize, height: usize, margin: usize) -> ListVisualizationOptions {
        let mut largest = 0;
        for &e in data {
            if e > largest {
                largest = e;
            }
        }
        let unit_height = (height-2*margin) as f64 / (largest as f64);
        let element_width = (width-(data.len()+1)*margin) as f64 / data.len() as f64;
        let mut element_positions = Vec::with_capacity(data.len());
        let mut position = margin as f64;
        for _ in 0..data.len() {
            element_positions.push((position as usize,(position+element_width) as usize));
            position += element_width + margin as f64;
        }
        ListVisualizationOptions {
            margin, element_positions, unit_height
        }
    }
}

struct ElementList<'a> {
    slice: &'a mut [u32],
    operations: Vec<Option<Operation>>,
}
impl<'a> ElementList<'a> {
    fn new(slice: &mut [u32]) -> ElementList {
        let mut operations = Vec::with_capacity(slice.len());
        for _ in 0..slice.len() {
            operations.push(None);
        }
        ElementList { slice, operations }
    }
}

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const COLOR_BACKGROUND: u32 = 0xFF111111;
const COLOR_FILL: u32 = 0xFF00AA22;
const COLOR_COMPARE: u32 = 0xFF00AAAA;
const COLOR_SWAP: u32 = 0xFFAA0055;

fn draw_screen(fb: &mut [u32], options: &ListVisualizationOptions, list: &ElementList, redraw_indices: Option<Vec<usize>>) {
    let update_indices = if let Some(indices) = redraw_indices {
        // Draw requested elements only
        indices
    } else {
        // Draw background
        for e in &mut fb.iter_mut() {
            *e = COLOR_BACKGROUND;
        }
        // Draw all elements
        (0..list.slice.len()).collect::<Vec<_>>()
    };
    for i in update_indices {
        // Select element color based on current operation
        let color = if let Some(operation) = &list.operations[i] {
            match operation {
                Operation::Compare => COLOR_COMPARE,
                Operation::Swap => COLOR_SWAP,
            }
        } else { COLOR_FILL };
        let x_bounds = options.element_positions.get(i).unwrap();
        let y_bound = HEIGHT - (((list.slice[i] as f64)*options.unit_height) as usize + options.margin);
        // Draw background above element
        for y in 0..y_bound {
            for e in fb[y*WIDTH + x_bounds.0..y*WIDTH + x_bounds.1].iter_mut() { 
                *e = COLOR_BACKGROUND;
            }
        }
        // Draw background below element (margin)
        for y in HEIGHT-options.margin..HEIGHT {
            for e in fb[y*WIDTH + x_bounds.0..y*WIDTH + x_bounds.1].iter_mut() { 
                *e = COLOR_BACKGROUND;
            }
        }
        // Draw element bar
        for y in y_bound..HEIGHT-options.margin {
            for e in fb[y*WIDTH + x_bounds.0..y*WIDTH + x_bounds.1].iter_mut() { 
                *e = color;
            }
        }
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
    // Generate data
    let mut data = [0; 100];
    for i in 0..data.len() {
        data[i] = rand::thread_rng().next_u32() % HEIGHT as u32;
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
        // println!("{}", frame_begin.elapsed().subsec_millis());
    }
}
