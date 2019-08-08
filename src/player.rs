use sort::{CallbackList, List, Operation};

use crate::graphics::*;

use std::marker::Send;
use std::sync::mpsc::SyncSender;
use std::thread;
use std::time::Duration;

pub fn play<T, F>(sort_fn: F, data: Vec<T>, window: ListVisualizationWindow<T>)
where
    T: 'static + Copy + Ord + Into<f64> + Send + std::fmt::Display,
    F: 'static + FnOnce(&mut List<T>) + Send,
{
    // Make update channel for Window
    let channel = window.make_update_channel();
    // Display initial slice
    channel
        .send(
            data.iter()
                .enumerate()
                .map(move |(i, &e)| ListUpdate {
                    index: i,
                    value: e,
                    color: COLOR_FILL,
                })
                .collect(),
        )
        .unwrap();
    // Call the sorting function
    thread::spawn(move || {
        // Capture variables + create CallbackList
        let channel = &channel;
        let mut data = data;
        let mut list = CallbackList::new(&mut data, make_callback(channel));
        // Call sort function
        sort_fn(&mut list);
        // Display ending animation
        ending_animation(&data, &channel);
    });
    // Execute window loop
    window.update_loop(Duration::from_millis(10));
}

fn make_callback<'a, T>(
    channel: &'a SyncSender<Vec<ListUpdate<T>>>,
) -> Box<'a + Fn(Operation, &[T]) + Send>
where
    T: Copy + Ord + Into<f64> + Send + std::fmt::Display,
{
    Box::new(move |operation, slice| {
        channel
            .send(match operation {
                Operation::Get(i) => vec![ListUpdate {
                    index: i,
                    value: slice[i],
                    color: COLOR_READ,
                }],
                Operation::Set(i) => vec![ListUpdate {
                    index: i,
                    value: slice[i],
                    color: COLOR_WRITE,
                }],
                Operation::Compare(i1, i2) => vec![
                    ListUpdate {
                        index: i1,
                        value: slice[i1],
                        color: COLOR_READ,
                    },
                    ListUpdate {
                        index: i2,
                        value: slice[i2],
                        color: COLOR_READ,
                    },
                ],
                Operation::Swap(i1, i2) => vec![
                    ListUpdate {
                        index: i1,
                        value: slice[i1],
                        color: COLOR_WRITE,
                    },
                    ListUpdate {
                        index: i2,
                        value: slice[i2],
                        color: COLOR_WRITE,
                    },
                ],
            })
            .unwrap();
    })
}

fn ending_animation<T>(
    data: &Vec<T>,
    channel: &SyncSender<Vec<ListUpdate<T>>>,
) 
where
    T: Copy + Ord + Into<f64> + Send + std::fmt::Display,
{
    let mut previous = None;
    data.iter().enumerate().for_each(move |(index, &value)| {
        // Select color: Different color if previous_value > value
        let color = if let Some(previous_value) = previous {
            if previous_value > value {
                COLOR_DONE_ERROR
            } else {
                COLOR_DONE_OK
            }
        } else {
            COLOR_DONE_OK
        };
        // Send update
        channel
            .send(vec![ListUpdate {
                index,
                value,
                color,
            }])
            .unwrap();
        // Store previous value
        previous = Some(value);
    });
}
