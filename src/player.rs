extern crate sort;

use sort::{Operation,List,CallbackList};

use crate::graphics::*;

use std::sync::mpsc::SyncSender;
use std::time::Duration;
use std::thread;
use std::marker::Send;

pub fn play<'a,T>(data: Vec<T>, window: ListVisualizationWindow<T>) 
where T: 'static + Copy + Ord  + Into<f64> + Send + std::fmt::Display
{
    // Make update channel for Window
    let channel = window.make_update_channel();
    // Display initial slice
    channel.send(data.iter().enumerate()
        .map(move |(i,&e)| ListUpdate {
            index: i,
            value: e,
            color: COLOR_FILL,
        }).collect()
    ).unwrap();
    // Call the sorting function
    thread::spawn(move || {
        let mut data = data;
        let mut list = CallbackList::new(&mut data, make_callback(channel));
        sort::quicksort2::quicksort2(&mut list);
    });
    // Execute window loop
    window.update_loop(Duration::from_millis(10));
}

fn make_callback<T>(updater: SyncSender<Vec<ListUpdate<T>>>) -> Box<Fn(Operation,&[T]) + Send>
where T: 'static + Copy + Ord  + Into<f64> + Send + std::fmt::Display
{
    Box::new(move |operation, slice| {
        updater.send(
            match operation {
                Operation::Get(i) => vec![ListUpdate { index: i, value: slice[i], color: COLOR_READ }],
                Operation::Set(i) => vec![ListUpdate { index: i, value: slice[i], color: COLOR_WRITE }],
                Operation::Compare(i1,i2) => vec![
                    ListUpdate { index: i1, value: slice[i1], color: COLOR_READ }, 
                    ListUpdate { index: i2, value: slice[i2], color: COLOR_READ },
                ],
                Operation::Swap(i1,i2) => vec![
                    ListUpdate { index: i1, value: slice[i1], color: COLOR_WRITE }, 
                    ListUpdate { index: i2, value: slice[i2], color: COLOR_WRITE },
                ],
            }
        ).unwrap();
    })
}
