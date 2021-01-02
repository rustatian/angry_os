//! This file handles the `print!()` macro, which allows displaying information to the UEFI stdout

use core::fmt::{Result, Write};

pub struct ScreenWriter;

impl Write for ScreenWriter {
    fn write_str(&mut self, string: &str) -> Result {
       crate::efi::output_string(string);
       Ok(())
    }
}

/// The std Rust `print!()` macro!
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        let _ = <$crate::print::ScreenWriter as core::fmt::Write>::write_fmt(
            &mut $crate::print::ScreenWriter,
            format_args!($($arg)*)
        );
    }
}