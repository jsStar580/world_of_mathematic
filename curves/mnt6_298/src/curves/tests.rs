use crate::*;
use ark_world_of_mathematics_test_templates::*;

test_group!(g1; G1Projective; sw);
test_group!(g2; G2Projective; sw);
test_group!(pairing_output; ark_ec::pairing::PairingOutput<MNT6_298>; msm);
test_pairing!(pairing; crate::MNT6_298);
