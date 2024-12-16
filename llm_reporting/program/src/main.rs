//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use sha2::{Sha256, Digest};
use regex::{Regex};
use regex_lib::{PublicValuesStruct};
use bincode;

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.

    let reg = sp1_zkvm::io::read::<String>();
    let input_text = sp1_zkvm::io::read::<String>();

    // Compute the hash of the input text
    let mut hasher = Sha256::new();
    hasher.update(&input_text);
    let input_hash = hasher.finalize();

    // Check if the input text matches the regex pattern
    let is_match = Regex::new(&reg).unwrap().is_match(&input_text);

    // Encode the public values of the program
    // let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct {
    //     reg,
    //     input_hash: input_hash.into(),
    //     is_match,
    // });

    let public_values = PublicValuesStruct {
        reg,
        input_hash: input_hash.into(),
        is_match,
    };
    let bytes = bincode::serialize(&public_values).expect("Failed to serialize public values");

    // Commit to the public values of the program
    sp1_zkvm::io::commit_slice(&bytes);
}
