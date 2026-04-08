# Rust Journey 🦀

Welcome to my Rust learning repository! This project documents my progress as I work through [The Rust Programming Language](https://doc.rust-lang.org/book/) (often referred to as "The Book"). It contains a mix of chapter exercises, small experiments, and more complex projects.

## 📚 Table of Contents (The Book Chapters)

This repository is structured to follow the progression of the official Rust documentation:

- **[Chapter 3: Common Programming Concepts](./3_Common_Programming_Concepts)** - Variables, data types, and basic control flow.
- **[Chapter 4: Understanding Ownership](./4_Understanding_Ownership)** - Ownership, references, borrowing, and slices.
- **[Chapter 5: Using Structs](./5_Using_Structs_to_Structure_Related_Data)** - Defining and instantiating structures.
- **[Chapter 6: Enums and Pattern Matching](./6_Enums_and_Pattern_Matching)** - Enumerations and the `match` operator.
- **[Chapter 7: Managing Growing Projects](./7_Managing_Growing_Projects_with_Packages,_Crates,_and_Modules)** - Packages, crates, and modules.
- **[Chapter 8: Common Collections](./8_Common_Collections)** - Working with Vectors, Strings, and HashMaps.
- **[Chapter 9: Error Handling](./9_Error_Handling)** - Recoverable and unrecoverable errors.
- **[Chapter 10: Generic Types, Traits, and Lifetimes](./10_Generic_Types,_Traits,_and_Lifetimes)** - Code reuse and abstraction.
- **[Chapter 11: Writing Automated Tests](./11_Writing_Automated_Tests)** - Building robust code through testing.
- **[Chapter 13: Functional Language Features](./13_%20Functional_Language_Features)** - Closures and iterators.
- **[Chapter 15: Smart Pointers](./15_smart_pointers)** - `Box<T>`, `Rc<T>`, `RefCell<T>`, and more.
- **[Chapter 16: Fearless Concurrency](./16_Fearless_Concurrency)** - Threads and message passing.
- **[Chapter 17: Object-Oriented Features](./17_Object_Oriented_Programming_Features_of_Rust)** - Traits and OOP patterns.
- **[Chapter 18: Patterns and Matching](./18_Patterns_and_Matching)** - Deep dive into pattern syntax.
- **[Chapter 19: Advanced Features](./19_Advanced_Features)** - Unsafe Rust, Advanced Traits, and Macros.
- **[Chapter 20: Final Project](./20_Final_Project)** - Building a Multithreaded Web Server.

## 🛠️ Custom Works & Projects

Beyond the book exercises, I am building custom projects located in the `works/` directory:

- **[Hangman](./works/hangman)**: A command-line implementation of the classic Hangman game, featuring modular logic and authentication.
- **[Data Pipeline ETL](./works/data_pipeline-etl)**: Experiments with data extraction, transformation, and loading.
- **[Time Utils](./works/time_)**: Utilities for handling time and date operations in Rust.

## 🚀 Getting Started

To run any of the projects or exercises, ensure you have [Rust and Cargo](https://rustup.rs/) installed.

1. Navigate to a specific project directory:
   ```bash
   cd works/hangman
   ```
2. Build and run:
   ```bash
   cargo run
   ```
3. Run tests:
   ```bash
   cargo test
   ```

## 🧹 Maintenance

I use the included `clean_rust_projects.sh` script to clean up `target/` directories and save disk space across the various sub-projects.