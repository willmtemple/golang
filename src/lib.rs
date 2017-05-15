mod make;

pub use std::thread::spawn;

pub use make::Make;

#[macro_export]
macro_rules! go {
    ($e:expr) => { $crate::spawn(move || {
            $e;
        })
    };
}

