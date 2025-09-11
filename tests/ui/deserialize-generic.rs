#[typetag::serde]
pub trait Trait<T> {}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct StructWithLifeTime<'a>(&'a str);

#[typetag::serde]
impl<'a> Trait<i8> for StructWithLifeTime<'a> {}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct StructWithConst<const N: usize>(i8);

#[typetag::serde]
impl<const N: usize> Trait<i8> for StructWithConst<N> {}

fn main() {}
