use crate::*;
use ark_world_of_mathematics_test_templates::*;
use ark_ff::Field;

test_group!(g1; G1Projective; sw);
test_group!(g2; G2Projective; sw);
test_group!(pairing_output; ark_ec::pairing::PairingOutput<BW6_767>; msm);
test_pairing!(pairing; crate::BW6_767);
