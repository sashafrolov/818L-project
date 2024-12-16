//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use alloy_sol_types::SolType;
use clap::Parser;
use regex_lib::PublicValuesStruct;
use sp1_sdk::{ProverClient, SP1Stdin};
use std::fs::read_to_string;
use std::env;
use regex::{Regex};
use sha2::{Sha256, Digest};
use std::time::Instant;


/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const REGEX_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    // #[clap(long, default_value = "20")]
    // n: u32,

    #[clap(long, default_value = "../res/regex.txt")]
    regex_file_path: String,

    #[clap(long, default_value = "../res/input.csv")]
    input_file_path: String,
}


fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();

    println!("Using regex_file_path: {}", &args.regex_file_path);
    println!("Using input_file_path: {}", &args.input_file_path);

    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);

    let regex = read_to_string(&args.regex_file_path).expect("Failed to read regex file");
    let input_text = read_to_string(&args.input_file_path).expect("Failed to read the input_text file");

    println!("Regex: {}", regex);
    println!("Input Text: {}", input_text);

    stdin.write(&regex);
    stdin.write(&input_text);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(REGEX_ELF, stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let decoded = bincode::deserialize(output.as_slice()).unwrap();
        let PublicValuesStruct { reg, input_hash, is_match } = decoded;

        // println!("Decoded: {:?}", decoded);
        println!("reg: {}", reg);
        println!("input_hash: {:?}", input_hash);
        println!("is_match: {}", is_match);


        let mut hasher = Sha256::new();
        hasher.update(&input_text);
        let expected_input_hash: [u8; 32] = hasher.finalize().as_slice().try_into().expect("Wrong length");
        let expected_is_match = Regex::new(&reg).unwrap().is_match(&input_text);

        // Check if the input text matches the regex pattern
        assert_eq!(expected_input_hash, input_hash);
        assert_eq!(expected_is_match, is_match);
        println!("Values are correct!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(REGEX_ELF);

        let start = Instant::now();

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        let duration = start.elapsed();

        println!("Successfully generated proof!");
        println!("Proof generation took: {:?}", duration);

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
