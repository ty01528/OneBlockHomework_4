pub enum Signal {
    Red,
    Yellow,
    Green,
}
pub trait Duration {
    fn duration(&self) -> u32;
}
impl Duration for Signal {
    fn duration(&self) -> u32 {
        match self {
            Signal::Red => 30,
            Signal::Yellow => 5,
            Signal::Green => 20,
        }
    }
}
