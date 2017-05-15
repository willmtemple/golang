use std::sync::mpsc::{Receiver,Sender,channel};

/// Make is a trait that provides a single function to ambiguously create
/// multiple different kinds of data. The slow and ruinous permeation of years
/// worth of technical debt will be worth the saved keystrokes, I'm certain.
pub trait Make {
    fn make() -> Self;
}

impl <T> Make for Vec<T> {
    fn make() -> Vec<T> {
        Vec::new()
    }
}

impl <T> Make for (Sender<T>, Receiver<T>) {
    fn make() -> (Sender<T>, Receiver<T>) {
        channel()
    }
}

