#[cfg(test)]
mod tests {
    extern crate std;

    use std::fs;

    use plonky2::{
        alloc::vec::Vec,
        field::{extension::Extendable, goldilocks_field::GoldilocksField},
        hash::hash_types::RichField,
        plonk::config::{GenericConfig, PoseidonGoldilocksConfig},
    };
    use starky::proof::StarkProof;

    use crate::{deserializer, verifier::verify_starky_proof};

    #[test]
    fn test_verify() {
        const D: usize = 2;
        type C = PoseidonGoldilocksConfig;
        type F = <C as GenericConfig<D>>::F;

        let proof_bytes = handle_proof::<F, C, D>();
        let pubs_bytes = handle_pubs();

        verify_starky_proof(proof_bytes, pubs_bytes).unwrap();
    }

    // ! Helper functions
    fn handle_proof<F, C, const D: usize>() -> Vec<u8>
    where
        F: RichField + Extendable<D>,
        C: GenericConfig<D, F = F>,
    {
        let json_path_proof = "./src/resources/json/proof.json";
        // ! Read from JSON to String
        let json_string_proof =
            fs::read_to_string(json_path_proof).expect("Failed to read JSON file");

        // ! Deserialize JSON into Vec<u32>
        let json_data_proof: StarkProof<F, C, D> =
            serde_json::from_str(&json_string_proof).expect("Failed to parse JSON");
        // println!("{:?}", json_data_proof);

        // ! Serialize into Bytes
        let bytes_proof = deserializer::serialize_proof(&json_data_proof);
        // println!("{:?}", bytes_proof);

        // ! Write bytes to file
        let output_path_proof = "./src/resources/bin/proof.bin";
        fs::write(output_path_proof, bytes_proof).expect("Failed to write binary file");
        // ! Read bytes from file
        let bytes_from_file_proof = fs::read(output_path_proof).unwrap();

        // ! Just a check that it is in right format and it can be deserialized
        // let deserialized_value_proof =
        deserializer::deserialize_proof::<F, C, D>(bytes_from_file_proof.clone()).unwrap();

        bytes_from_file_proof
    }

    fn handle_pubs() -> Vec<u8> {
        let json_path_pubs = "./src/resources/json/pubs_array.json";
        // ! Read from JSON to String
        let json_string_pubs =
            fs::read_to_string(json_path_pubs).expect("Failed to read JSON file");

        // ! Deserialize JSON into Vec<GoldilocksField>
        let json_data_pubs: Vec<GoldilocksField> =
            serde_json::from_str(&json_string_pubs).expect("Failed to parse JSON");
        // println!("{:?}", json_data_pubs);

        // ! Serialize into Bytes
        let bytes_pubs = deserializer::serialize_pubs(&json_data_pubs);
        // println!("{:?}", bytes_pubs);

        // ! Write bytes to file
        let output_path_pubs = "./src/resources/bin/pubs_array.bin";
        fs::write(output_path_pubs, bytes_pubs).expect("Failed to write binary file");
        // ! Read bytes from file
        let bytes_from_file_pubs = fs::read(output_path_pubs).unwrap();

        // ! Just a check that it is in right format and it can be deserialized
        // let deserialized_value_pubs =
        deserializer::deserialize_public_inputs(bytes_from_file_pubs.clone()).unwrap();
        // println!("{:?}", deserialized_value_pubs);
        bytes_from_file_pubs
    }
}
