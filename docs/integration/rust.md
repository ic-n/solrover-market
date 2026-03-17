# Rust Programming Language

The Rust programming language is designed for performance, reliability, and productivity. It provides memory safety guarantees without garbage collection through its ownership system, making it suitable for systems programming, embedded devices, web services, and command-line tools. The language features zero-cost abstractions, move semantics, guaranteed memory safety, threads without data races, trait-based generics, pattern matching, type inference, and efficient C bindings.

This repository contains the complete Rust toolchain including the compiler (`rustc`), the standard library (`std`, `core`, `alloc`), and essential development tools. The standard library provides portable abstractions for common programming tasks including I/O operations, networking, threading, collections, and platform-specific functionality. The compiler is written in Rust itself and uses LLVM for code generation, supporting numerous target platforms and architectures.

## File Operations with std::fs

The `std::fs` module provides filesystem manipulation operations including file reading, writing, creation, and metadata access. All methods represent cross-platform operations with platform-specific extensions available through `std::os`.

```rust
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::path::Path;

fn main() -> io::Result<()> {
    // Create and write to a file
    let mut file = File::create("example.txt")?;
    file.write_all(b"Hello, Rust!")?;

    // Read entire file to string
    let contents = fs::read_to_string("example.txt")?;
    println!("Contents: {}", contents);

    // Read file with buffering for efficiency
    let file = File::open("example.txt")?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    // Append to existing file
    let mut file = OpenOptions::new()
        .append(true)
        .open("example.txt")?;
    file.write_all(b"\nAppended content")?;

    // Get file metadata
    let metadata = fs::metadata("example.txt")?;
    println!("File size: {} bytes", metadata.len());
    println!("Is file: {}", metadata.is_file());
    println!("Modified: {:?}", metadata.modified()?);

    // Create directory and copy file
    fs::create_dir_all("backup/data")?;
    fs::copy("example.txt", "backup/example.txt")?;

    // Read directory contents
    for entry in fs::read_dir("backup")? {
        let entry = entry?;
        println!("Found: {:?}", entry.path());
    }

    // Cleanup
    fs::remove_file("example.txt")?;
    fs::remove_dir_all("backup")?;

    Ok(())
}
```

## Threading with std::thread

The `std::thread` module provides native OS thread management including spawning, joining, thread-local storage, and synchronization primitives. Threads can be configured with custom names and stack sizes using the `Builder` pattern.

```rust
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, mpsc};

fn main() {
    // Basic thread spawning
    let handle = thread::spawn(|| {
        println!("Hello from spawned thread!");
        42
    });
    let result = handle.join().unwrap();
    println!("Thread returned: {}", result);

    // Named thread with custom stack size
    let builder = thread::Builder::new()
        .name("worker".to_string())
        .stack_size(32 * 1024);

    let handle = builder.spawn(|| {
        println!("Thread name: {:?}", thread::current().name());
    }).unwrap();
    handle.join().unwrap();

    // Sharing data between threads with Arc and Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented counter", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final counter value: {}", *counter.lock().unwrap());

    // Message passing with channels
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["hello", "from", "thread"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // Get available parallelism
    let parallelism = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    println!("Available parallelism: {}", parallelism);
}
```

## Process Management with std::process

The `std::process` module handles spawning and interacting with child processes, including I/O redirection, environment configuration, and process termination. It provides a builder-style interface through the `Command` struct.

```rust
use std::process::{Command, Stdio, exit};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    // Simple command execution
    let output = Command::new("echo")
        .arg("Hello, world!")
        .output()?;

    println!("Status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // Command with environment variables
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo $MY_VAR")
        .env("MY_VAR", "custom_value")
        .output()?;
    println!("With env: {}", String::from_utf8_lossy(&output.stdout));

    // Spawn long-running process and interact with it
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    // Write to child's stdin
    if let Some(ref mut stdin) = child.stdin {
        stdin.write_all(b"Hello from parent\n")?;
    }
    child.stdin.take(); // Close stdin to signal EOF

    // Read from child's stdout
    let mut output = String::new();
    if let Some(ref mut stdout) = child.stdout {
        stdout.read_to_string(&mut output)?;
    }
    println!("Child output: {}", output);

    // Wait for child to finish
    let status = child.wait()?;
    println!("Child exited with: {}", status);

    // Pipe output from one command to another
    let echo = Command::new("echo")
        .arg("hello world")
        .stdout(Stdio::piped())
        .spawn()?;

    let tr = Command::new("tr")
        .args(["a-z", "A-Z"])
        .stdin(echo.stdout.unwrap())
        .output()?;

    println!("Piped result: {}", String::from_utf8_lossy(&tr.stdout));

    // Check if command exists
    let status = Command::new("rustc")
        .arg("--version")
        .status()?;

    if status.success() {
        println!("rustc is installed");
    }

    Ok(())
}
```

## Networking with std::net

The `std::net` module provides TCP and UDP networking primitives including listeners, streams, and socket address types. It supports both IPv4 and IPv6 communication with cross-platform compatibility.

```rust
use std::net::{TcpListener, TcpStream, UdpSocket, SocketAddr};
use std::io::{Read, Write, BufRead, BufReader};
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // TCP Server
    thread::spawn(|| -> std::io::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:8080")?;
        println!("Server listening on {}", listener.local_addr()?);

        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [0; 1024];
            let bytes_read = stream.read(&mut buffer)?;
            println!("Server received: {}",
                String::from_utf8_lossy(&buffer[..bytes_read]));
            stream.write_all(b"HTTP/1.1 200 OK\r\n\r\nHello!")?;
        }
        Ok(())
    });

    thread::sleep(Duration::from_millis(100));

    // TCP Client
    let mut client = TcpStream::connect("127.0.0.1:8080")?;
    client.set_read_timeout(Some(Duration::from_secs(5)))?;
    client.set_write_timeout(Some(Duration::from_secs(5)))?;

    client.write_all(b"GET / HTTP/1.1\r\n\r\n")?;

    let mut response = String::new();
    let mut reader = BufReader::new(&client);
    reader.read_line(&mut response)?;
    println!("Client received: {}", response);

    // UDP Communication
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    let target: SocketAddr = "127.0.0.1:9000".parse().unwrap();

    // Send UDP packet
    socket.send_to(b"Hello UDP", target).ok();

    // Set socket options
    socket.set_read_timeout(Some(Duration::from_secs(1)))?;
    socket.set_broadcast(true)?;

    // Parse IP addresses
    let ipv4: std::net::Ipv4Addr = "192.168.1.1".parse().unwrap();
    let ipv6: std::net::Ipv6Addr = "::1".parse().unwrap();
    println!("IPv4: {}, IPv6: {}", ipv4, ipv6);

    // Socket address handling
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("Address: {}, Port: {}", addr.ip(), addr.port());

    Ok(())
}
```

## Collections with std::collections

The `std::collections` module provides efficient implementations of common data structures including vectors, hash maps, sets, and queues. These collections offer different performance characteristics for various use cases.

```rust
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap, BTreeMap};

fn main() {
    // HashMap - key-value storage with O(1) average lookup
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Alice".to_string(), 100);
    scores.insert("Bob".to_string(), 85);

    // Entry API for conditional insertion
    scores.entry("Alice".to_string()).or_insert(0);
    *scores.entry("Charlie".to_string()).or_insert(0) += 50;

    // Iterate over key-value pairs
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // Get with default
    let score = scores.get("Unknown").unwrap_or(&0);
    println!("Unknown score: {}", score);

    // HashSet - unique value collection
    let mut tags: HashSet<&str> = HashSet::new();
    tags.insert("rust");
    tags.insert("programming");
    tags.insert("rust"); // Duplicate ignored

    println!("Contains rust: {}", tags.contains("rust"));

    // Set operations
    let other_tags: HashSet<&str> = ["rust", "systems"].iter().cloned().collect();
    let intersection: HashSet<_> = tags.intersection(&other_tags).collect();
    let union: HashSet<_> = tags.union(&other_tags).collect();
    println!("Intersection: {:?}", intersection);
    println!("Union: {:?}", union);

    // VecDeque - double-ended queue
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_front(0);

    while let Some(item) = queue.pop_front() {
        println!("Queue item: {}", item);
    }

    // BinaryHeap - priority queue (max-heap)
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(1);
    heap.push(5);

    while let Some(max) = heap.pop() {
        println!("Max: {}", max); // Prints 5, 4, 3, 1, 1
    }

    // BTreeMap - sorted map
    let mut sorted_map: BTreeMap<i32, &str> = BTreeMap::new();
    sorted_map.insert(3, "three");
    sorted_map.insert(1, "one");
    sorted_map.insert(2, "two");

    // Iterate in sorted order
    for (key, value) in &sorted_map {
        println!("{}: {}", key, value);
    }

    // Range queries
    for (key, value) in sorted_map.range(1..3) {
        println!("Range: {}: {}", key, value);
    }
}
```

## Error Handling with Result and Option

Rust's error handling is built around the `Result<T, E>` and `Option<T>` types, providing explicit and composable error management without exceptions. The `?` operator enables concise error propagation.

```rust
use std::fs::File;
use std::io::{self, Read, ErrorKind};
use std::num::ParseIntError;

// Custom error type
#[derive(Debug)]
enum AppError {
    IoError(io::Error),
    ParseError(ParseIntError),
    Custom(String),
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<ParseIntError> for AppError {
    fn from(err: ParseIntError) -> Self {
        AppError::ParseError(err)
    }
}

fn read_number_from_file(path: &str) -> Result<i32, AppError> {
    let mut file = File::open(path)?; // ? converts io::Error to AppError
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let number: i32 = contents.trim().parse()?; // ? converts ParseIntError
    Ok(number)
}

fn main() -> Result<(), AppError> {
    // Basic Result handling
    let result: Result<i32, &str> = Ok(42);

    match result {
        Ok(value) => println!("Success: {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // Unwrap variants
    let value = result.unwrap(); // Panics on Err
    let value = result.unwrap_or(0); // Default on Err
    let value = result.unwrap_or_else(|_| -1); // Computed default
    let value = result.expect("Should have value"); // Panic with message

    // Option handling
    let maybe_value: Option<i32> = Some(42);

    if let Some(v) = maybe_value {
        println!("Got value: {}", v);
    }

    let value = maybe_value.unwrap_or_default();
    let value = maybe_value.map(|v| v * 2);
    let value = maybe_value.and_then(|v| if v > 0 { Some(v) } else { None });

    // Converting between Option and Result
    let opt: Option<i32> = Some(42);
    let res: Result<i32, &str> = opt.ok_or("missing value");

    let res: Result<i32, &str> = Ok(42);
    let opt: Option<i32> = res.ok();

    // Error handling with match
    let file_result = File::open("config.txt");
    let file = match file_result {
        Ok(f) => f,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            File::create("config.txt")?
        }
        Err(e) => return Err(AppError::IoError(e)),
    };

    // Chaining with map and and_then
    let result: Result<i32, AppError> = File::open("number.txt")
        .map_err(AppError::from)
        .and_then(|mut f| {
            let mut s = String::new();
            f.read_to_string(&mut s).map_err(AppError::from)?;
            s.trim().parse::<i32>().map_err(AppError::from)
        });

    Ok(())
}
```

## Synchronization with std::sync

The `std::sync` module provides thread synchronization primitives including mutexes, read-write locks, atomic types, condition variables, and barriers for safe concurrent programming.

```rust
use std::sync::{Arc, Mutex, RwLock, Condvar, Barrier, Once};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    // Mutex - mutual exclusion lock
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let mut handles = vec![];

    for i in 0..3 {
        let data = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let mut vec = data.lock().unwrap();
            vec.push(i + 4);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final data: {:?}", *data.lock().unwrap());

    // RwLock - multiple readers, single writer
    let config = Arc::new(RwLock::new(String::from("initial")));

    // Multiple readers
    let readers: Vec<_> = (0..3).map(|i| {
        let config = Arc::clone(&config);
        thread::spawn(move || {
            let value = config.read().unwrap();
            println!("Reader {}: {}", i, *value);
        })
    }).collect();

    // Single writer
    {
        let mut value = config.write().unwrap();
        *value = String::from("updated");
    }

    for r in readers { r.join().unwrap(); }

    // Atomic types - lock-free operations
    let counter = Arc::new(AtomicUsize::new(0));
    let flag = Arc::new(AtomicBool::new(false));

    let handles: Vec<_> = (0..10).map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            counter.fetch_add(1, Ordering::SeqCst);
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    println!("Atomic counter: {}", counter.load(Ordering::SeqCst));

    // Condvar - condition variable for waiting
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let (lock, cvar) = &*pair2;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    while !*ready {
        ready = cvar.wait(ready).unwrap();
    }
    println!("Condition met!");

    // Barrier - synchronization point
    let barrier = Arc::new(Barrier::new(3));
    let handles: Vec<_> = (0..3).map(|i| {
        let barrier = Arc::clone(&barrier);
        thread::spawn(move || {
            println!("Thread {} waiting at barrier", i);
            barrier.wait();
            println!("Thread {} passed barrier", i);
        })
    }).collect();

    for h in handles { h.join().unwrap(); }

    // Once - one-time initialization
    static INIT: Once = Once::new();
    static mut CONFIG: Option<String> = None;

    INIT.call_once(|| {
        unsafe { CONFIG = Some(String::from("initialized")); }
    });
}
```

## Building Rust from Source

The Rust compiler uses a Python-based build system called `x.py` for bootstrapping. The build process compiles the compiler using a pre-compiled snapshot, then uses that to build the final compiler and standard library.

```bash
# Clone the repository
git clone https://github.com/rust-lang/rust.git
cd rust

# Interactive setup (recommended for first-time contributors)
./x.py setup

# Configure with specific options
./configure --enable-debug --enable-llvm-assertions

# Build the compiler
./x.py build

# Build specific components
./x.py build library/std
./x.py build compiler/rustc

# Run tests
./x.py test
./x.py test src/test/ui
./x.py test library/std

# Build documentation
./x.py doc

# Install to custom prefix
./x.py install --prefix=/usr/local

# Check formatting and lints
./x.py fmt --check
./x.py clippy

# Clean build artifacts
./x.py clean

# Build with specific configuration profile
./x.py build --config profile=compiler

# Cross-compile for different target
./x.py build --target aarch64-unknown-linux-gnu
```

## I/O Operations with std::io

The `std::io` module provides core I/O traits (`Read`, `Write`, `Seek`, `BufRead`) and types for working with input/output streams, buffers, and standard I/O handles.

```rust
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter, Cursor, Seek, SeekFrom};
use std::fs::File;

fn main() -> io::Result<()> {
    // Standard I/O
    let mut input = String::new();
    print!("Enter your name: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    println!("Hello, {}!", input.trim());

    // Write to stderr
    writeln!(io::stderr(), "This is an error message")?;

    // Buffered reading for efficiency
    let file = File::open("data.txt")?;
    let reader = BufReader::new(file);

    // Read lines efficiently
    for line in reader.lines() {
        let line = line?;
        println!("Line: {}", line);
    }

    // Buffered writing
    let file = File::create("output.txt")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "Line 1")?;
    writeln!(writer, "Line 2")?;
    writer.flush()?; // Ensure all data is written

    // Cursor - in-memory I/O
    let data = b"Hello, World!";
    let mut cursor = Cursor::new(data.to_vec());

    let mut buffer = [0u8; 5];
    cursor.read_exact(&mut buffer)?;
    println!("Read: {:?}", String::from_utf8_lossy(&buffer));

    // Seeking
    cursor.seek(SeekFrom::Start(7))?;
    let mut rest = String::new();
    cursor.read_to_string(&mut rest)?;
    println!("Rest: {}", rest);

    // Chain readers
    let reader1 = Cursor::new(b"Hello, ");
    let reader2 = Cursor::new(b"World!");
    let mut chained = reader1.chain(reader2);

    let mut combined = String::new();
    chained.read_to_string(&mut combined)?;
    println!("Chained: {}", combined);

    // Take limited bytes
    let data = Cursor::new(b"Hello, World!");
    let mut limited = data.take(5);
    let mut buffer = String::new();
    limited.read_to_string(&mut buffer)?;
    println!("Limited: {}", buffer);

    // Copy between streams
    let mut source = Cursor::new(b"Data to copy");
    let mut dest = Vec::new();
    io::copy(&mut source, &mut dest)?;
    println!("Copied: {:?}", String::from_utf8_lossy(&dest));

    Ok(())
}
```

## Iterator Operations

Rust's iterator system provides lazy, composable operations for processing sequences of values. Iterators are zero-cost abstractions that compile to efficient code equivalent to hand-written loops.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Basic iteration
    for n in &numbers {
        println!("{}", n);
    }

    // Map - transform elements
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);

    // Filter - select elements
    let evens: Vec<&i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("Evens: {:?}", evens);

    // Filter and map combined
    let even_squared: Vec<i32> = numbers.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("Even squared: {:?}", even_squared);

    // Fold/reduce
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    let product: i32 = numbers.iter().product();
    println!("Sum: {}, Product: {}", sum, product);

    // Find and position
    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    let position = numbers.iter().position(|x| *x == 5);
    println!("First even: {:?}, Position of 5: {:?}", first_even, position);

    // Take and skip
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    let skip_three: Vec<&i32> = numbers.iter().skip(3).collect();
    println!("First 3: {:?}, After 3: {:?}", first_three, skip_three);

    // Enumerate - with indices
    for (i, n) in numbers.iter().enumerate() {
        println!("Index {}: {}", i, n);
    }

    // Zip - combine iterators
    let letters = vec!['a', 'b', 'c'];
    let zipped: Vec<_> = numbers.iter().zip(letters.iter()).collect();
    println!("Zipped: {:?}", zipped);

    // Chain - concatenate iterators
    let more = vec![11, 12, 13];
    let chained: Vec<&i32> = numbers.iter().chain(more.iter()).collect();
    println!("Chained length: {}", chained.len());

    // Flat map - flatten nested structures
    let nested = vec![vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<&i32> = nested.iter().flat_map(|v| v.iter()).collect();
    println!("Flattened: {:?}", flat);

    // Partition - split by predicate
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers.iter()
        .partition(|x| *x % 2 == 0);
    println!("Evens: {:?}, Odds: {:?}", evens, odds);

    // All and any
    let all_positive = numbers.iter().all(|x| *x > 0);
    let any_large = numbers.iter().any(|x| *x > 5);
    println!("All positive: {}, Any > 5: {}", all_positive, any_large);

    // Custom iterator
    struct Counter { count: u32, max: u32 }

    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let counter = Counter { count: 0, max: 5 };
    let sum: u32 = counter.sum();
    println!("Counter sum: {}", sum);
}
```

The Rust standard library serves as the foundation for building reliable, efficient software across diverse domains. Its design emphasizes zero-cost abstractions, memory safety without garbage collection, and seamless interoperability with existing C code. The ownership system, combined with the borrow checker, enables fearless concurrency and eliminates entire classes of bugs at compile time.

For developers contributing to Rust itself, the bootstrap system provides comprehensive tooling for building, testing, and validating changes. The modular architecture separates the compiler (`rustc`) from the standard library, allowing independent development and testing. The extensive test suite, combined with continuous integration, ensures stability across all supported platforms while enabling rapid iteration on new language features and optimizations.
