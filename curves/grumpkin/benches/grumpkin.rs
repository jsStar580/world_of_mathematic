use ark_world_of_mathematics_bench_templates::*;
use ark_grumpkin::{fq::Fq, fr::Fr, Projective as G};

bench!(
    Name = "Grumpkin",
    Group = G,
    ScalarField = Fr,
    PrimeBaseField = Fq,
);
