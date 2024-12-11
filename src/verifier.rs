use plonky2::{
    alloc::vec::Vec,
    plonk::config::{GenericConfig, PoseidonGoldilocksConfig},
};
use starky::{
    config::StarkConfig, fibonacci_stark::FibonacciStark, proof::StarkProofWithPublicInputs,
    verifier::verify_stark_proof,
};

use crate::deserializer::{deserialize_proof, deserialize_public_inputs, DeserializeError};

pub fn verify_starky_proof(proof: &Vec<u8>, pubs: &Vec<u8>) -> Result<(), DeserializeError> {
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;
    // TODO -> check S
    type S = FibonacciStark<F, D>;
    let config = StarkConfig::standard_fast_config();
    let num_rows = 1 << 5;
    let stark = S::new(num_rows);

    let proof = deserialize_proof::<F, C, D>(proof)?;
    let pubs = deserialize_public_inputs(pubs)?;

    let proof_with_pubs = StarkProofWithPublicInputs {
        proof,
        public_inputs: pubs,
    };

    verify_stark_proof(stark, proof_with_pubs, &config, None).unwrap();
    Ok(())
}
