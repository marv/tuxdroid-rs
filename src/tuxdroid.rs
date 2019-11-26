
pub trait TuxDroid {
    fn eyes_blink(&self) -> bool;

    fn open_mouth(&self) -> bool;
    fn close_mouth(&self) -> bool;

    fn flippers_raise(&self) -> bool;
    fn flippers_lower(&self) -> bool;
    fn flippers_wave(&self) -> bool;
    fn flippers_stop(&self) -> bool;
}

