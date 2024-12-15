use sails_client_gen::ClientGenerator;
use std::{env, path::PathBuf, fs};
use app::MyProgram;

fn main() {
    // Build contract to get .opt.wasm
    sails_rs::build_wasm();

    // Path where the file "Cargo.toml" is located (points to the root of the project)
    // 'CARGO_MANIFEST_DIR' specifies this directory in en::var
    let cargo_toml_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Path where the client will be generated 
    // 'OUT_DIR' points to a temporary directory used by the compiler 
    // to store files generated at compile time. 
    let outdir_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Path where the file "app.idl" will be created
    let idl_path = cargo_toml_path.clone().join("app.idl");
    let client_path = outdir_path.clone().join("app_client.rs");

    // This generate the contract IDL
    sails_idl_gen::generate_idl_to_file::<MyProgram>(idl_path.clone())
        .unwrap();

    // Generator of the clients of the contract
    ClientGenerator::from_idl_path(&idl_path)
        // .with_mocks("with_mocks")
        .generate_to(client_path.clone())
        .unwrap();

    // Then, copies the client that is in the OUT_DIR path in the current
    // directory (wasm), where the 
    // "Cargo.toml" file is located 
    fs::copy(client_path, cargo_toml_path.join("app_client.rs"))
        .unwrap();
}