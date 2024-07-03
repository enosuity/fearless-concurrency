# Singleton Pattern in Rust

This project demonstrates a thread-safe singleton pattern in Rust. The singleton pattern ensures that a class has only one instance and provides a global point of access to it. This implementation is thread-safe and can be used across multiple threads without causing data races.

## Features

- Thread-safe singleton pattern
- Uses Rust's `Arc`, `Mutex`, and `Once` for safe concurrency
- Example usage with multiple threads updating the singleton instance

## Structure

- `Singleton<T>`: The singleton struct holding the value.
- `SingletonWrapper<T>`: Manages the singleton instance and ensures it is initialized only once.

## Code Explanation

- `Singleton<T>`: A simple struct with methods to set and show its value.
- `SingletonWrapper<T>`: Manages the singleton instance using `Arc`, `Mutex`, and `Once`.
  - `new(init_val: T) -> Self`: Initializes the `SingletonWrapper` with the initial value.
  - `get_instance(&self) -> Arc<Mutex<Singleton<T>>>`: Returns the singleton instance, ensuring it is initialized only once.

## Usage

Here is how you can use the `SingletonWrapper` to ensure thread-safe access to a singleton instance: