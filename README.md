# hello-riscv

A simple example of booting to Rust. Built in the style of the [RISC-V ISA tests](https://github.com/riscv-software-src/riscv-tests), with a dash of [riscv-rt](https://github.com/rust-embedded/riscv/tree/master/riscv-rt) for some of the more highly oxidized bits. It's a single flat directory with only a single trivial dependency because I'm really a gopher wearing a crab costume.

One possible use: instantiating up a cycle-accurate RTL simulator by way of [Chipyard](https://github.com/ucb-bar/chipyard)[^chipyard-setup][^chipyard-sim] and running the program, as in:

```
RUSTFLAGS='-C link-arg=-Tlink.ld' cargo build --target riscv64imac-unknown-none-elf && ~/Code/src/github.com/ucb-bar/chipyard/sims/verilator/simulator-chipyard.harness-RocketConfig ./target/riscv64imac-unknown-none-elf/debug/main
```

If you see your terminal prompt come back after a dozen seconds, then it's a success! It will take a while, though, and the most common failure mode is "silent."

[^chipyard-setup]: https://chipyard.readthedocs.io/en/stable/Chipyard-Basics/Initial-Repo-Setup.html
[^chipyard-sim]: https://chipyard.readthedocs.io/en/stable/Simulation/Software-RTL-Simulation.html#sw-rtl-sim-intro

# Suggested Exercises

Besides changing the exit code in `_start_rust` to see a non-success result, here's a few other ideas.

## Debugging the Linker Script

Try deleting a line in the linker script, and seeing what happens: with the exception of comments, they're all load-bearing.

Some tools you might find helpful:

* llvm-objdump
* readelf
* passing `-C link-args=--print-map` in `RUSTFLAGS`


## Setting up the Trap Vector

Try dropping an `unimp` somewhere in _start: what should happen? What does happen (hint: it's a hang!)?

The RISC-V tests all install an exception handler that does some degree of host commnication. They also use an ecall/exception handler pair to do their host communication, which makes them a lot easier to port to new targets or communication patterns. Those both seem patterns worth investigating.

## Debugging with JTAG + GDB

Can you get stepwise debugging working right from reset? I didn't, but I spent a couple of hours comparing objdump output that could've be saved by a simple `info registers` and an `si` or two.

This looks handy: https://chipyard.readthedocs.io/en/stable/Advanced-Concepts/Chip-Communication.html#debugging-with-jtag

## Multiple HARTs

I mean, just change "max_hart_id" in the linker script, and you're good, right?

# Future Work

Getting the commented-out `println!` working over the emulated UART might be a nice challenge. Doing so will probably tug on all the invariants rustc expects that the simulator's ELF loader doesn't provide: hours of learning fun await!

To turn this into something approximating a HAL or even a kernel, you might want to consider modularizing the `_exit` into some kind of a system call.  about how it ought to behave the same (or differently) across different targets.
