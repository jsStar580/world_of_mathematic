use ark_world_of_mathematics_bench_templates::*;
use ark_curve25519::{EdwardsProjective as G, Fq, Fr};

bench!(
    Name = "Curve25519",
    Group = G,
    ScalarField = Fr,
    PrimeBaseField = Fq,
);
