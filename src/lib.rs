pub use std::thread::spawn;

#[macro_export]
macro_rules! go {
    ($e:expr) => { $crate::spawn(move || {
            $e;
        })
    };
}

