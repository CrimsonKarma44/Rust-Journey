# Rust Journey 🦀

> My worked exercises from *The Rust Programming Language* ("The Book"), plus small Rust projects I've built on top of it.

I worked through the whole book, including the chapter exercises and the final project. The repo follows the book's chapter order and also holds my own experiments and projects under [`works/`](./works).

## 🧰 Stack

- **Language:** Rust (editions 2021 and 2024)
- **Tooling:** Cargo. Cargo workspaces are used for multi-crate projects such as `hangman` and `leet_code`.
- **Crates:** mostly the standard library only. `rand` is used for the games.
- **Scripts:** Bash (`clean_rust_projects.sh`)

## 📚 Book Chapters

| Chapter | Topic |
|---|---|
| [3 – Common Programming Concepts](./3_Common_Programming_Concepts) | Variables, data types, functions, control flow |
| [4 – Understanding Ownership](./4_Understanding_Ownership) | Ownership, references, borrowing, slices |
| [5 – Using Structs](./5_Using_Structs_to_Structure_Related_Data) | Defining and instantiating structs, methods |
| [6 – Enums and Pattern Matching](./6_Enums_and_Pattern_Matching) | Enums, `Option`, `match`, `if let` |
| [7 – Packages, Crates, and Modules](./7_Managing_Growing_Projects_with_Packages,_Crates,_and_Modules) | Module system and visibility |
| [8 – Common Collections](./8_Common_Collections) | `Vec`, `String`, `HashMap` |
| [9 – Error Handling](./9_Error_Handling) | `panic!`, `Result`, the `?` operator |
| [10 – Generics, Traits, and Lifetimes](./10_Generic_Types,_Traits,_and_Lifetimes) | Code reuse and abstraction |
| [13 – Functional Language Features](./13_%20Functional_Language_Features) | Closures and iterators |
| [15 – Smart Pointers](./15_smart_pointers) | `Box<T>`, `Rc<T>`, `RefCell<T>`, `Deref`, `Drop` |
| [16 – Fearless Concurrency](./16_Fearless_Concurrency) | Threads, channels, `Mutex`, `Arc` |
| [17 – OOP Features](./17_Object_Oriented_Programming_Features_of_Rust) | Trait objects and the state pattern |
| [18 – Patterns and Matching](./18_Patterns_and_Matching) | All the places patterns can be used |
| [19 – Advanced Features](./19_Advanced_Features) | Unsafe Rust, advanced traits and types, macros |
| [20 – Final Project](./20_Final_Project) | Multithreaded web server with a thread pool |

Chapters 11 (testing), 12 (the `minigrep` project) and 14 (Cargo and crates.io) have no folder of their own. Their material shows up inside the other projects, for example the `#[cfg(test)]` modules.

## 🛠️ Projects (`works/`)

| Project | What it does |
|---|---|
| [hangman](./works/hangman) | Command-line Hangman, split into a Cargo workspace (`cmd`, `hang_logic`, `authentication`, `utils`) |
| [bulls_and_cows](./works/bulls_and_cows) | Number-guessing game that uses `rand`, with hints after repeated tries |
| [leet_code](./works/leet_code) | LeetCode / kata solutions (string merging, GCD of strings, credit-card masking) |
| [iterators](./works/iterators) | Iterator and closure exercises with unit tests |
| [time_](./works/time_) | Reads a text file and searches it line by line (`cargo run -- <file>`) |
| [data_pipeline-etl](./works/data_pipeline-etl) | Early scaffold for an extract-transform-load experiment |
| [mystuff](./works/mystuff) | Scratchpad for traits, trait objects and file I/O |
| [tests](./works/tests) | Small throwaway experiments (shadowing, strings, `Box`/`Vec`) |

## 🚀 How to Run

You need [Rust and Cargo](https://rustup.rs/), stable toolchain. The 2024 edition requires Rust 1.85 or newer.

Each chapter folder and each project is its own Cargo package or workspace, so run Cargo from inside the folder you want:

```bash
git clone https://github.com/CrimsonKarma44/Rust-Journey.git
cd Rust-Journey/works/hangman

cargo run      # build and run
cargo test     # run the tests
```

## 🧹 Maintenance

`clean_rust_projects.sh` runs `cargo clean` in each top-level folder that has a `Cargo.toml`, so the `target/` folders stop taking up disk space. The repo path is set inside the script (`PROJECTS_DIR`), so change it if your clone lives somewhere else:

```bash
./clean_rust_projects.sh
```

## 📄 License

[MIT](./LICENSE) © Vincent Princewill
