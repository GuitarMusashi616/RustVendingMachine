pub trait Stockable {
    fn get_state() -> &'static str;
}

pub struct Unstocked {}

impl Stockable for Unstocked {
    fn get_state() -> &'static str {
        return "Unstocked";
    }
}

impl Stockable for Stocked {
    fn get_state() -> &'static str {
        return "Stocked";
    }
}

pub struct Stocked {
    pub password: String,
}
