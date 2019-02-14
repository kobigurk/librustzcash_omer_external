extern crate sapling_crypto;
extern crate pairing;

use sapling_crypto::jubjub::{edwards, fs::Fs, PrimeOrder, Unknown, JubjubBls12, JubjubEngine };
use pairing::bls12_381::Bls12;
pub type PK = edwards::Point<Bls12,Unknown>;
#[derive(Clone)]
pub struct JubjubPoint {
    purpose: &'static str,
    ge: PK,
}

fn main() {
    let x = PK::zero();
    println!("point x: {}", x.into_xy().0);
}
