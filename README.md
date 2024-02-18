# bfc-x86-64

A Brainf*ck compiler and interpreter for Linux x86-64.

## Installing and Builing
Assuming you have [Rust](https://www.rust-lang.org/learn/get-started) installed, you can build the project from source.
1. Clone the repository:
```sh
git clone https://github.com/Imn0/bfc-x86-64.git
```
1. Navigate to the project directory:
```sh
cd bfc-x86-64
```
1. Build the project
```sh
cargo build
```
## Usage
```sh
cargo run -- [options] FILE
```
or by using the compiled binary
```sh
./bfc [options] FILE
```
By default program will compile given file to an executable. 
This behaviour can be changed with options.

| Option | Description                  |
| ------ | ---------------------------- |
| `-o`   | output file name             |
| `-i`   | interpret given file instead |
| `-h`   | print help menu              |
| `-D`   | print code debug info        |

#### Example usage
1. `cargo run -- -o hw.out ./examples/hello_world.bf` will compile `hello_world.bf` to `hw.out` 
2. `cargo run -- -i ./examples/cat.bf` will interpret `cat.bf`


---
###### Do not use in production environment.