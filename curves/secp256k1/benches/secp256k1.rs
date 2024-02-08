use ark_world_of_mathematics_bench_templates::*;
use ark_secp256k1::{fq::Fq, fr::Fr, Projective as G};

bench!(
    Name = "Secp256k1",
    Group = G,
    ScalarField = Fr,
    PrimeBaseField = Fq,
);
