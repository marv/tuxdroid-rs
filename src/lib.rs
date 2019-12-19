//! TuxDroid crate
//!
//!
//!

extern crate hidapi;

pub mod hardware;
pub mod fux;
pub mod usb;
pub mod tuxdroid;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
