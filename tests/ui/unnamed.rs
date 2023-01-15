#[typetag::serde]
pub trait Trait {}

#[typetag::serde]
impl Trait for (u8, u8) {}

fn main() {}
