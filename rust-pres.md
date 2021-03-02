---
title: Introduction to Rust
author: Fe Crabs
date: 2021-02-26
extensions:
  - image_ueberzug
---

![25](images/title.png)

---
# History of Rust
  - Rust was a personal project that began in 2006 by Mozilla employee Graydon Hoare.
  - C++ was difficult to write in.
  - Rust was intended to be a language for highly concurrent and safe systems.
  - Mozilla began to sponsor the project in 2009.
  - The first prealpha release of Rust occurred in Janurary 2012.
  - Rust 1.0 was released on May 15, 2015.
![16](images/ifz2rkk8ugw51.png)

---

# What is Rust?
  - Systems language
  - Statically-type multi paradigm language (functional, OOP, imperative etc.)
  - Focus on Performance, Reliability, and Productivity
  - Safe!
  - Stack Overflow's most loved language
![16](images/theydontknow.jpg)

---
# Terms to know

| Word                    | Definition                                                             |
|-------------------------|------------------------------------------------------------------------|
| Pointer                 | Variable that stores the address of another variable                   |
| Free                    | Function that clears memory allocated to a variable                    |
| Garbage Collector       | A feature in PLs that automatically frees memory not needed in Runtime |
| &                       | Operation to view a memory address of a variable                       |
| UB (Undefined Behavior) | When a program runs unexpectedly                                       |

![14](images/heap.jpg)

---

# Basic Buliding Blocks

| Type  | Meaning                                             |
|-------|-----------------------------------------------------|
| i32   | Signed integer with 32 bits                         |
| u32   | Unsigned integer with 32 bits                       |
| f32   | Double with 32 bites                                |
| usize | Uses your computer architecture's to determine size |
| char  | UTF-8 - 4 bits                                      |
| &str  | Slice (String Literal)                              |

```rust
// Variables - Rust gueses the type of your variable most of the time
let x = 10;

// But you can also tell rust the exact type you expect.
let y: i32 = 10;

// Rust also has tuples
let pair: (usize, f64) = (176, 87.123);

// Arrays which are non-dynamic
let arr: [i32;5] = [1, 2, 3, 4, 5];

// Dynamic Arrays - think ArrayList
let vec: Vec = vec![1, 2, 3];
```

---

# Introduction to Memory Safety
  - Memory is extremely dangerous
  
---

# Essential Features
  - A language that can check for errors before runtime
  - Ownership
    - Each value in Rust has a variable that’s called its owner.
    - There can only be one owner at a time.
    - When the owner goes out of scope, the value will be dropped.
  - Borrow checker
  - Slices
![16](images/unsafe.png)
---

# Features
  - Compiler Messages
  - Great libraries
  - Toolchain

---

# What you can use Rust for:
  - Writing system software
  - Web projects (ie. Dropbox, OpenDNS, Coursera, Discord)
  - Games (theoretically)
  - Networking due to security and reliability
  - Machine learning (low-level memory control and performance)
![16](images/useit.png)
---

# Why you shouldn't use Rust
  - Compiler is very slow due to many features
  - APIs are changing frequently due to how new Rust is.
  - Language is difficult for people coming from other languages
  - Code is a lot harder to develop than on c or c++
  - No garbage collector, which isn't for everyone
![16](images/kidnap.png)
