# Rust Concurrency Examples

This project contains several examples demonstrating the use of concurrency in Rust. The examples include working with mutexes, atomic reference counting, and threading.

## Project Structure

- `ArrayWrapper<T, const N: usize>`: A generic array wrapper with fixed-size arrays.
- `SingletonWrapper<T>`: A singleton implementation using `Arc`, `Mutex`, and `Once`.
- `ThreadSafeStack<T>`: A thread-safe stack implemented with a mutex.

## Running the Examples

To run the examples, you will need to have Rust installed on your system. You can install Rust from [rust-lang.org](https://www.rust-lang.org/).

1. Clone the repository:
   ```sh
   git clone https://github.com/enosuity/fearless-concurrency.git
   cd fearless-concurrency