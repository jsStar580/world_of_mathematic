use ark_world_of_mathematics_bench_templates::*;
use ark_ed_on_bls12_377::{fq::Fq, fr::Fr, EdwardsProjective as G};

bench!(
    Name = "EdOnBls12_377",
    Group = G,
    ScalarField = Fr,
    PrimeBaseField = Fq,
);
