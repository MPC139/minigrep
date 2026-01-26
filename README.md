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

### 3. All-in-One Benchmark Command
Run both consecutively to see the comparison immediately:
```bash
echo "--- SYSTEM GREP ---" && time grep "the" big.txt > /dev/null && echo -e "\n--- MINIGREP ---" && time ./target/release/minigrep "the" big.txt > /dev/null
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

./target/release/minigrep "'" big.txt
```

## Trade-offs & Design Decisions

This implementation prioritizes code clarity and learning over absolute raw performance, though it is still quite fast.

### 1. Line-by-Line vs Block Processing
*   **Current Approach**: We use `BufReader::read_until(b'\n')`. This is idiomatic Rust and easy to reason about.
*   **Trade-off**: It requires scanning every byte for a newline character *before* we can search for the query. System `grep` reads massive fixed-size blocks (e.g., 32KB) and searches them directly, which is significantly faster but much more complex to implement correctly.

### 2. Memory Usage
*   **Current Approach**: The buffer grows to fit the longest line found in the file.
*   **Trade-off**: If the file has a massive single line (e.g., a minified JS file of 500MB), this program will attempt to allocate 500MB of RAM. A production-grade tool uses a fixed-size circular buffer window to handle this safely.

### 3. SIMD Optimization
*   **Current Approach**: We use standard byte iteration (`data.windows()`).
*   **Trade-off**: This checks bytes sequentially. Modern `grep` uses SIMD (Single Instruction, Multiple Data) instructions to compare 16-32 bytes in a single CPU cycle, resulting in massive speedups.

### 4. Output Buffering
*   **Optimization**: We lock `stdout` (`io::stdout().lock()`) to manually acquire the lock once before the loop.
*   **The Problem**: By default, `print!` acquires and releases a lock on stdout for *every single call* to ensure thread safety. Doing this millions of times is slow.
*   **Analogy**:
    *   **Default**: Key-card into the server room, type one line, walk out. Repeat.
    *   **Locked**: Key-card in once, prop the door open, type everything, then leave.



