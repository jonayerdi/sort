extern crate minifb;
extern crate sort;

use std::marker::PhantomData;
use std::sync::mpsc::{sync_channel,SyncSender,Receiver};
use std::time::{Duration,Instant};
use std::thread::sleep;

use minifb::{WindowOptions, Window};

pub const COLOR_BACKGROUND: u32 = 0xFF111111;
pub const COLOR_FILL: u32 = 0xFF00AA22;
pub const COLOR_READ: u32 = 0xFF00AAAA;
pub const COLOR_WRITE: u32 = 0xFFAA0055;

pub struct ListVisualization<T> 
where T: Copy + Ord + Into<f64> + std::fmt::Display
{
    pub height: usize,
    pub width: usize,
    pub margin: usize,
    pub element_positions: Vec<(usize,usize)>,
    pub unit_height: f64,
    _marker: PhantomData<T>,
}
impl<T> ListVisualization<T>
where T: Copy + Ord + Into<f64> + std::fmt::Display
{
    pub fn autogenerate(data: &[T], width: usize, height: usize, margin: usize) -> ListVisualization<T> {
        let largest = data.iter().fold(0.0 as f64, |acc, &x| x.into().max(acc));
        let unit_height = (height-2*margin) as f64 / largest;
        let element_width = (width-(data.len()+1)*margin) as f64 / data.len() as f64;
        let mut element_positions = Vec::with_capacity(data.len());
        let mut position = margin as f64;
        for _ in 0..data.len() {
            element_positions.push((position as usize,(position+element_width) as usize));
            position += element_width + margin as f64;
        }
        ListVisualization {
            height, width, margin, element_positions, unit_height, _marker: PhantomData
        }
    }
    pub fn draw<I>(&self, changes: I, framebuffer: &mut [u32]) 
    where I: Iterator<Item=ListUpdate<T>> 
    {
        for element in changes {
            let x_bounds = self.element_positions.get(element.index).unwrap();
            let y_bound = self.height - (((element.value.into())*self.unit_height) as usize + self.margin);
            // Draw background above element
            for y in 0..y_bound {
                let y_offset = y*self.width;
                for e in framebuffer[y_offset + x_bounds.0..y_offset + x_bounds.1].iter_mut() { 
                    *e = COLOR_BACKGROUND;
                }
            }
            // Draw background below element (margin)
            for y in self.height-self.margin..self.height {
                let y_offset = y*self.width;
                for e in framebuffer[y_offset + x_bounds.0..y_offset + x_bounds.1].iter_mut() { 
                    *e = COLOR_BACKGROUND;
                }
            }
            // Draw element bar
            for y in y_bound..self.height-self.margin {
                let y_offset = y*self.width;
                for e in framebuffer[y_offset + x_bounds.0..y_offset + x_bounds.1].iter_mut() { 
                    *e = element.color;
                }
            }
        }
    }
}

#[derive(Clone,Copy)]
pub struct ListUpdate<T>
where T: Copy + Ord + Into<f64> + std::fmt::Display
{
    pub index: usize,
    pub value: T,
    pub color: u32,
}

pub struct ListVisualizationWindow<T> 
where T: Copy + Ord + Into<f64> + std::fmt::Display
{
    pub window: Window,
    pub channel: (SyncSender<Vec<ListUpdate<T>>>, Receiver<Vec<ListUpdate<T>>>),
    pub visualization: ListVisualization<T>,
    pub framebuffer: Vec<u32>,
}
impl<T> ListVisualizationWindow<T>  
where T: Copy + Ord + Into<f64> + std::fmt::Display
{
    pub fn new(visualization: ListVisualization<T>) -> ListVisualizationWindow<T> {
        let window = Window::new("Sort GUI", visualization.width, visualization.height,
            WindowOptions::default()).unwrap_or_else(|e| {
                panic!("{}", e);
            }
        );
        let channel = sync_channel(8); // Arbitrary buffer size
        ListVisualizationWindow { 
            window,
            channel,
            framebuffer: vec![0; visualization.width * visualization.height],
            visualization,
        }
    }
    pub fn update(&mut self, changes: Vec<ListUpdate<T>>) -> minifb::Result<()> {
        self.visualization.draw(changes.iter().map(|x| *x), &mut self.framebuffer);
        self.window.update_with_buffer(&self.framebuffer)
    }
    pub fn update_loop(&mut self, refresh_period: Duration) {
        while self.is_open() {
            let before = Instant::now();
            if let Ok(next_update) = self.channel.1.try_recv() {
                self.update(next_update).unwrap();
            } else {
                // Need to call this periodically for the Window to remain responsive
                self.window.update_with_buffer(&self.framebuffer).unwrap();
            }
            let elapsed = Instant::now() - before;
            if elapsed < refresh_period {
                sleep(refresh_period - elapsed);
            }
            println!("{}", elapsed.subsec_millis());
        }
    }
    pub fn make_update_channel(&self) -> SyncSender<Vec<ListUpdate<T>>> {
        self.channel.0.clone()
    }
    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
}
