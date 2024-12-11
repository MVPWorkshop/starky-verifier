use plonky2::alloc::vec::Vec;

use plonky2::{
    field::{extension::Extendable, goldilocks_field::GoldilocksField},
    hash::hash_types::RichField,
    plonk::config::GenericConfig,
};
use starky::proof::StarkProof;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("Deserialization of proof failed")]
    InvalidProof,
    #[error("Deserialization of public inputs failed")]
    InvalidPubs,
}

pub fn serialize_pubs(pubs: &Vec<GoldilocksField>) -> Vec<u8> {
    postcard::to_allocvec(&pubs).unwrap()
}

pub fn serialize_proof<F, C, const D: usize>(proof: &StarkProof<F, C, D>) -> Vec<u8>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
{
    postcard::to_allocvec(proof).unwrap()
}

pub(crate) fn deserialize_proof<F, C, const D: usize>(
    proof_bytes: &Vec<u8>,
) -> Result<StarkProof<F, C, D>, DeserializeError>
where
    F: RichField + Extendable<D>,
    C: GenericConfig<D, F = F>,
{
    postcard::from_bytes(proof_bytes).map_err(|_| DeserializeError::InvalidProof)
}

pub(crate) fn deserialize_public_inputs(
    pubs: &Vec<u8>,
) -> Result<Vec<GoldilocksField>, DeserializeError> {
    postcard::from_bytes(pubs).map_err(|_| DeserializeError::InvalidPubs)
}
