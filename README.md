# Hash Finder

Hash Finder is a Rust-based application that helps you search for hash values based on specified parameters.

## How to Build and Run

### Prerequisites

Ensure you have the following tools installed:

- [Rust](https://www.rust-lang.org/): The Rust programming language. You can install it using `rustup`.
- [Cargo](https://doc.rust-lang.org/cargo/): The Rust package manager (installed automatically with Rust).

### Steps to Build

1. Clone the repository:

    ```bash
    git clone https://github.com/fizkultprivet322/hash-finder.git
    cd hash-finder
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

### How to Run the Program

After building the program, you can run it using the following command:

```bash
./target/release/hash-finder.exe -N <number_of_zeros> -F <number_of_results>
```

Where:

- -N specifies the number of leading zeros to search for in the hash (integer value).
- -F specifies how many results to display (integer value).
