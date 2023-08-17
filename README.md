# Logic Module Program

This project computes the logical and arithmetic operators for a string to individual modules:

- We take a string input like **(a==1)||(b1>3)&&(c_1!=5)||(d<=4)** and gets the user parameters for the variables **a,b1,c_1 and d.**

The result is broken down into modules and then computed for them.

- Ex: **a==1: ,Enter value of a=5.**

Result for the modules (eg)= **[false, true, false, true]**

Main Result (eg)= **[true]**


This code is written in [Rust](https://www.rust-lang.org/). We convert to WebAssembly wasm code with the help 
of [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/) and [wasm-pack](https://rustwasm.github.io/wasm-pack/book/introduction.html).
The result can be gotten but the **rust lib**, **html client** interface and **nodejs** CLI.


## Getting Started and Instillation

Follow the process to get up you working environment

### Prerequisites

Requirements for the software and other tools to build and run this project 
- curl
- Ubuntu 20.04 +
- sudo-enabled 


### Step 1 — Installing Rust dependencies

Reference link: https://www.digitalocean.com/community/tutorials/install-rust-on-ubuntu-linux#step-1-installing-rust-on-ubuntu-using-the-rustup-tool

We use curl to set up rustup tools. Rustup allows us to run rust code. Run the command as below

    curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh

Then follow the installation procedure to install. (default installation recommended)

### Step 2 — Set up rust PATH environment

Next, run the following command to add the Rust toolchain directory to the PATH environment variable:

    source $HOME/.cargo/env


### Step 3 — Verifying the Installation
Verify the Rust installation by requesting the version:

    rustc --version


### Step 4 — Installing a Compiler
Rust requires a GNU Compiler Collection like (gcc). WE need build-essential packager. 

You’ll use apt to install the build-essential package. First, update the Apt package index:

    sudo apt update

Followed by 

    sudo apt upgrade

To install the build-essential package: Enter the command

    sudo apt install build-essential


### Step 5 — Setting up Cargo

Installing rustup includes the installation of cargo. Cargo is a rust package managed that is efficient at handling rust code and its dependencies

To confirm cargo is installed, run the following.

    cargo --version

If cargo is not available, run the following

    curl https://sh.rustup.rs -sSf | sh


### Step 6 — Setting up Tools for cargo to use for Webassembly

For this project, we will be using wasm-pack to hand all wasm conversions:

To install wasm-pack, run the code below

    cargo install wasm-pack

To use node in out project, install nodejs 

    sudo apt install nodejs

Also install nodejs's package manager npm

    sudo apt install npm


#### From the following, all dependencies to run the environment is installed. We can now clone this project and run our code.


## Running the tests

**Important**: Wasm-pack with wasm-bindgen allows or to create one 1 working environment at a time.
To run code

### Sample Tests

Explain what these tests test and why

    Give an example

### Style test

Checks if the best practices and the right coding style has been used.

    Give an example


## Authors

  - **K.AOUADI** :[link](https://github.com/kaouadi)
  - **Manish Naresh Gondhkar**


## License

This project is licensed under [MIT](LICENSE.md)
Creative Commons License - see the [LICENSE.md](LICENSE.md) file for details
