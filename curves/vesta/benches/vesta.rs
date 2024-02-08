use ark_world_of_mathematics_bench_templates::*;
use ark_vesta::{fq::Fq, fr::Fr, Projective as G};

bench!(
    Name = "Vesta",
    Group = G,
    ScalarField = Fr,
    PrimeBaseField = Fq,
);
