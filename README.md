# MiniGrep vs Grep Performance Comparison

This project implements a simplified version of `grep` in Rust. This document guides you through measuring and comparing its performance against the system's native `grep` command.

## Prerequisites

1.  **Build the Release Binary**
    Rust code runs significantly faster when optimized.
    ```bash
    cargo build --release
    ```

2.  **Download a Large Test Data File**
    To see meaningful differences, we need a large text file. We use "The Complete Works of William Shakespeare" (~5.5MB).
    ```bash
    curl -o big.txt https://www.gutenberg.org/files/100/100-0.txt
    ```

## Benchmarking

We use the `time` command to measure execution speed. We send output to `/dev/null` to measure only the processing time, ignoring terminal printing latency.

### 1. System Grep
Run the system grep searching for a common word like "the":
```bash
time grep "the" big.txt > /dev/null
```

### 2. MiniGrep (Rust)
Run our implementation with the same parameters:
```bash
time ./target/release/minigrep "the" big.txt > /dev/null
```

## Understanding Results

The output will show three times:
-   **real**: The actual elapsed wall-clock time. (Look at this one).
-   **user**: CPU time spent in user-mode code.
-   **sys**: CPU time spent in kernel calls (like file reading).

You will likely find that `grep` is faster because it uses highly optimized buffered reading and SIMD instructions, whereas this implementation currently reads the entire file into memory at once.

## Usage Examples

**Basic Search:**
```bash
./target/release/minigrep "To be" big.txt
```

**Case-Sensitive Search with Special Characters:**
(Use quotes to handle spaces or matching quotes)
```bash
./target/release/minigrep "'" big.txt
```

## TODO

- [ ] Improve the performance of minigrep

