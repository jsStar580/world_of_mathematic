use ark_world_of_mathematics_bench_templates::*;
use ark_pallas::{fq::Fq, fr::Fr, Projective as G};

bench!(
    Name = "Pallas",
    Group = G,
    ScalarField = Fr,
    PrimeBaseField = Fq,
);
