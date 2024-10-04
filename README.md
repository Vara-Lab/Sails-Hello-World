[![Open in Gitpod](https://img.shields.io/badge/Open_in-Gitpod-white?logo=gitpod)](https://github.com/Vara-Lab/Sails-Hello-World.git)

# Tutorial: Deploying Your First "Sails Hello World" on Vara Network

## Tabla of contents

- [Introduction](#introduction)
- [Requisites](#requisites-ubuntu)
- [Step 1: Clone the Smart Contract Template](#step-1-clone-the-smart-contract-template)
- [Step 2: Set Up Development Environment](#step-2-set-up-development-environment)
- [Step 3: Implement the Smart Contract](#step-3-implement-the-smart-contract)
- [Step 4: Implement wasm file to compile your contract](#step-4-implement-wasm-file-to-compile-your-contract)
- [Step 5: Compile the Smart Contract](#step-5-compile-the-smart-contract)
- [Step 6: Interact with Your Contract on Vara Network](#step-6-interact-with-your-contract-on-vara-network)
- [Step 7: Upload Your Contract on Vara Network](#step-7-upload-your-contract-on-vara-network)
- [Step 8: Send a Message to a Program](#step-8-send-a-message-to-a-program)

## Introduction

Welcome to the tutorial on deploying your first "Sails Hello World" program on Vara Network. Vara Network is a decentralized platform for deploying smart contracts and decentralized applications (dApps). This tutorial will guide you through the process of setting up your development environment and deploying a simple smart contract that prints "Hello" on the Vara Network.

## Requisites - Important (ubuntu)

1. You need to install GCC and Clang.

    ```bass
    sudo apt install -y build-essential clang cmake curl
    ```

2. Rust: You need to have rust 1.80 or newer to be able to compile your contract
    - In case that you dont have rust, you need to run the next commands one by one in your terminal:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup target add wasm32-unknown-unknown
    ```
    
    - If you have an outdated version of rust, use the following commands in your terminal:

    ```bash
    rustup install 1.81
    rustup default 1.81
    ```

## Step 1: Clone the Smart Contract Template

1. Create a GitHub account if you don't have one already.
2. Sign in to Gitpod using your GitHub account.[![Open in Gitpod]](https://gitpod.io/new/#https://github.com/Vara-Lab/Sails-Hello-World.git)
3. Create a new workspace on Gitpod using the following repository URL: 

   ```bash
   https://github.com/Vara-Lab/Sails-Hello-World.git
   ```
> Note: If you want to test the contract directly, enter the directory with the command:

    cd hello-world-contract
    
> Then, compile it with the command:

    cargo build --release

> Note: If you have an error like the following in your terminal "the `wasm32-unknown-unknown` target may not be installed" you need to install the wasm32 target to compile your contract, run the following command in your terminal and recompile the contract:

    rustup target add wasm32-unknown-unknown
    
Now, you can upload the contract in the [Gear IDEA](https://idea.gear-tech.io/programs?node=wss%3A%2F%2Ftestnet.vara.network)

## Step 2: Set Up Development Environment

4. Open your terminal and navigate to the directory where you want to store your project files and directories.
5. Create the necessary files for your project by running the following command:

   ```bash
   touch Cargo.toml
   ```

6. Add the following code to the `Cargo.toml` file:

    ```rust
    [workspace]
    resolver = "2"
    members = []

    [workspace.package]
    version = "0.1.0"
    edition = "2021"

    [workspace.dependencies]
    sails-client-gen = "0.5.0"
    sails-idl-gen = "0.5.0"
    sails-rs = "0.5.0"
    ```

7. Now, in the directory that is your `Cargo.toml` file, put the next commands in your terminal (this will create your app and wasm directories to create your contract):
    ```bash
    cargo new app --lib
    cargo new wasm --lib
    ```

8. Now, in the `app/Cargo.toml` file, you need to edit it as follows:

    ```rust
    [package]
    name = "app"
    version.workspace = true
    edition.workspace = true

    [dependencies]
    sails-rs.workspace = true
    ```

9. Then, in your `wasm/Cargo.toml` file, you will need to edit it as follows:

    ```rust
    [package]
    name = "wasm"
    version.workspace = true
    edition.workspace = true

    [dependencies]
    app = { path = "../app" }

    [build-dependencies]
    sails-rs = { workspace = true, features = ["wasm-builder"] }
    sails-client-gen.workspace = true
    sails-idl-gen.workspace = true
    app = { path = "../app" }
    ```

## Step 3: Implement the Smart Contract

10. Now, you will create your smart contract, first you need to enter in the `app/src` directory:

    ```bash
    cd app/src
    ```

11. You need to create a "services" directory (for better maintenance of services in the future), put the next commands in your terminal:

    ```bash
    mkdir services
    ```

12. Create the "mod.rs" and "my_service.rs" files in the services directory:

    ```bash
    touch services/{mod.rs,my_service.rs}
    ```

13. Add the following code to the `service/my_service.rs` file:

    ```rust
    use sails_rs::prelude::*;

    #[derive(Default)]
    pub struct MyService;

    #[service]
    impl MyService {
        pub fn new() -> Self {
            Self
        }

        pub fn hello(&mut self) -> String {
            "Hello world!".to_string()
        }
    }
    ```

14. Next, add the following code to the `service/mod.rs` file: 

    ```rust
    pub mod my_service;
    ```

15. With your new service already created, you will create your "program" for your contract, add the following code to the `app/src/lib.rs` file:

    ```rust
    #![no_std]
    use sails_rs::prelude::*;

    pub mod services;
    use services::my_service::MyService;

    #[derive(Default)]
    pub struct MyProgram;

    #[program]
    impl MyProgram {
        pub fn new() -> Self {
            Self
        }

        #[route("MyService")]
        pub fn my_service_svc(&self) -> MyService {
            MyService::new()
        }
    }
    ```

## Step 4: Implement wasm file to compile your contract

16. Now, access the `wasm` directory:

    ```bash
    cd ../../wasm/
    ```

17. Replace the code inside the `src/lib.rs` withe following code:

    ```rust
    #![no_std]

    #[cfg(target_arch = "wasm32")]
    pub use app::wasm::*;
    ```

18. Now, create the `build.rs` file:

    ```bash
    touch build.rs
    ```

19. And, add the next code in 'wasm/build.rs' that will "compile" and create the idl file and client for your contract:

    ```rust
    use sails_idl_gen::program;
    use std::{env, path::PathBuf};
    use sails_client_gen::ClientGenerator;
    use app::MyProgram;

    fn main() {
        // Build contract to get .opt.wasm
        // gear_wasm_builder::build();
        sails_rs::build_wasm();

        // Path where the file "Cargo.toml" is located (points to the root of the project)
        // 'CARGO_MANIFEST_DIR' specifies this directory in en::var
        let cargo_toml_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

        // Path where the file "app.idl" will be created
        let idl_path = cargo_toml_path.clone().join("app.idl");

        // This generate the contract IDL
        program::generate_idl_to_file::<MyProgram>(idl_path.clone())
            .unwrap();

        // Generator of the clients of the contract
        ClientGenerator::from_idl_path(&idl_path)
            .with_mocks("with_mocks")
            .generate_to(cargo_toml_path.join("app_client.rs"))
            .unwrap();
    }
    ```

## Step 5: Compile the Smart Contract

20. Return to your terminal and navigate to the root directory of your workspace.

    ```bash
    cd ..
    ```

21. Compile the smart contract by running the following command:

    ```bash
    cargo build --release
    ```

> Note: If you have an error like the following in your terminal "the `wasm32-unknown-unknown` target may not be installed" you need to install the wasm32 target to compile your contract, run the following command in your terminal and recompile the contract:

    rustup target add wasm32-unknown-unknown

Once the compilation is complete, locate the `app.idl` file in `wasm` directory, and the `wasm.opt.wasm` fie in the `target/wasm32-unknown-unknown/release` directory.


## Step 6: Interact with Your Contract on Vara Network

22. To interact with the Gear IDEA and deploy your contract, you will need to download a wallet extension such as [Polkadot-JS](https://polkadot.js.org/extension/), [Talisman](https://talisman.xyz/), or [Subwallet](https://subwallet.app/) to interact with Substrate-based chains.

<div align="center">
  <img src="https://polkadot.js.org/extension/extension-overview.png" alt="Polkadot-JS Extension">
</div>

23. Access [Gear IDEA](https://idea.gear-tech.io/programs?node=wss%3A%2F%2Frpc.vara.network) using your web browser.

<div align="center">
  <img src="https://hackernoon.imgix.net/images/77WjQmBCAIQ7dyhZ22Bkui5QTrb2-6n92fqm.jpeg" alt="Gear Protocol">
</div>


24. Connect your Substrate wallet to Gear IDEA by clicking the connect button.

25. Navigate to [Gear IDEA](https://idea.gear-tech.io). You will be prompted to grant access to your account for the Gear Tech application. Click "Yes, allow this application access".

<div align="center">
  <img src="https://wiki.gear-tech.io/assets/images/polkadot-access-c98e0c0e2df8de4cb5673f80e81743ac.png" alt="Gear Protocol">
</div>

## Step 7: Upload Your Contract on Vara Network

26. Upload the `wasm.opt.wasm` file by clicking the "Upload Program" button.

27. After uploading the .opt.wasm file, you need to uploat the IDL file. Click the "Select files" button and search for the .idl file.

<div align="center">
  <img src="https://wiki.vara.network/assets/images/add_idl-8886abd21d6ce6039f762cc5f2496660.png" alt="Gear Protocol">
</div>

28. Specify the contract name, click the `calculate gas` button to set the gas limit automatically, and then press the `Sumbit` button.

<div align="center">
  <img src="https://wiki.vara.network/assets/images/interface-d26e3a6259956cf5538b36b18b2793f1.png" alt="Gear Protocol">
</div>

29. Sign the program uploading the transaction to the Gear network. After your message has been successfully processed, you are to see correspondent log messages.

<div align="center">
  <img src="https://wiki.gear-tech.io/assets/images/sign-transaction-f9ae773fdad49788a0e9894238ba5558.png" alt="Find Your Program">
</div>

30. Once your program is uploaded, head to the Programs section and find your program.

<div align="center">
  <img src="https://wiki.gear-tech.io/assets/images/message-log-158efeb8c52fca9fcc080c40561c36df.png" alt="Signing Transaction">
</div>

## Step 8: Send a Message to a Program

31. Now, try sending your newly uploaded program a message to see how it responds! Click the "Send message" button.

32. Select the service and and the function to be called in the contract, click the `Calculate` button to set the gas limit automatically, then click the `Send Message` button to interact with your first "Sails Hello World" program on Vara Network.

<div align="center">
  <img src="https://wiki.vara.network/assets/images/send-request-141208770a80171baabd5c8243143447.png" alt="Sending Message Interface">
</div>

33. Sign the message sending transaction.

Congratulations! You have successfully deployed your first smart contract on Vara Network. Explore further and experiment with more complex smart contracts and decentralized applications to harness the full potential of Vara Network.
