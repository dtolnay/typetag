#[typetag::serde]
pub trait Trait<T> {}

pub struct Struct;

#[typetag::serde]
impl<T> Trait<T> for Struct {}

fn main() {}
