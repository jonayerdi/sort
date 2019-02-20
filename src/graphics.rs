extern crate minifb;
extern crate sort;

use minifb::{WindowOptions, Window};
use sort::Operation;

const COLOR_BACKGROUND: u32 = 0xFF111111;
const COLOR_FILL: u32 = 0xFF00AA22;
const COLOR_COMPARE: u32 = 0xFF00AAAA;
const COLOR_SWAP: u32 = 0xFFAA0055;

pub struct ListVisualizationOptions {
    pub height: usize,
    pub width: usize,
    pub margin: usize,
    pub element_positions: Vec<(usize,usize)>,
    pub unit_height: f64,
}
impl ListVisualizationOptions {
    pub fn autogenerate(data: &[u32], width: usize, height: usize, margin: usize) -> ListVisualizationOptions {
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
            height, width, margin, element_positions, unit_height
        }
    }
}

pub struct ElementList<'a> {
    pub slice: &'a mut [u32],
    pub operations: Vec<Option<Operation>>,
}
impl<'a> ElementList<'a> {
    pub fn new(slice: &mut [u32]) -> ElementList {
        let mut operations = Vec::with_capacity(slice.len());
        for _ in 0..slice.len() {
            operations.push(None);
        }
        ElementList { slice, operations }
    }
}

pub struct ListVisualizationWindow {
    pub window: Window,
    pub framebuffer: Vec<u32>,
    pub options: ListVisualizationOptions,
}
impl ListVisualizationWindow {
    pub fn new(options: ListVisualizationOptions) -> ListVisualizationWindow {
        let mut framebuffer: Vec<u32> = vec![0; options.width * options.height];
        let mut window = Window::new("Sort GUI", options.width, options.height,
            WindowOptions::default()).unwrap_or_else(|e| {
                panic!("{}", e);
            }
        );
        ListVisualizationWindow { window, framebuffer, options }
    }
    pub fn redraw(&mut self, list: &ElementList, redraw_indices: Option<Vec<usize>>) {
        let update_indices = if let Some(indices) = redraw_indices {
            // Draw requested elements only
            indices
        } else {
            // Draw background
            for e in &mut self.framebuffer.iter_mut() {
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
            let x_bounds = self.options.element_positions.get(i).unwrap();
            let y_bound = self.options.height - (((list.slice[i] as f64)*self.options.unit_height) as usize + self.options.margin);
            // Draw background above element
            for y in 0..y_bound {
                for e in self.framebuffer[y*self.options.width + x_bounds.0..y*self.options.width + x_bounds.1].iter_mut() { 
                    *e = COLOR_BACKGROUND;
                }
            }
            // Draw background below element (margin)
            for y in self.options.height-self.options.margin..self.options.height {
                for e in self.framebuffer[y*self.options.width + x_bounds.0..y*self.options.width + x_bounds.1].iter_mut() { 
                    *e = COLOR_BACKGROUND;
                }
            }
            // Draw element bar
            for y in y_bound..self.options.height-self.options.margin {
                for e in self.framebuffer[y*self.options.width + x_bounds.0..y*self.options.width + x_bounds.1].iter_mut() { 
                    *e = color;
                }
            }
        }
    }
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
    pub fn update(&mut self) -> minifb::Result<()> {
        self.window.update_with_buffer(&self.framebuffer)
    }
}
