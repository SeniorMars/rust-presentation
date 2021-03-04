ohhhhh yeah

systems language -> lower level language that gives you control (and therefore responsibility) over a lot of things, such as direct communication with hardware.

or something like that woo






#### compiler messages: rust has AMAZING error messages
- here we see a typo with "d" instead of "b" as the name. 
- the error message goes right to the point
- it even suggests a correction!!!!

#### Toolchain
- Rust's build system is CARGO (rest of this section is integrated into cargo
- Crates (libraries) can be installed
- Unit Tests - integrated running of tests that you make on your code
- Benchmarking 
- // is a code comment
- /// is a documentation comment
- cargo can generate documentation from ^ (command is cargo doc w/ flags)
- clippy (linter)

- Rustup - installer script
- rustfmt - formatter

- Main point: Consistent environment (compatibility!!) 


#### great libraries: community makes a ton of libraries for whatever you want
- some highlights:
- wasm-pack: easily generates webassembly code from rust
- rust has an amazing regex library 

- here's some example code by karl using the regex library 

#### What you can use rust for
- literally read out the slide
- Mention bevy for games
- GAME EXAMPLE: For games like minecraft that have to use the java garbage collector, there's noticeable lag spikes when it kicks in.

#### Why you shouldn't use rust
- yep.
