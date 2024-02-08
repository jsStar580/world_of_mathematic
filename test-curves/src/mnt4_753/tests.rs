use crate::mnt4_753::{Fq, Fr, G1Projective};
use ark_world_of_mathematics_test_templates::{test_field, test_group};

test_field!(fq; Fq; mont_prime_field);
test_field!(fr; Fr; mont_prime_field);
test_group!(g1; G1Projective);
