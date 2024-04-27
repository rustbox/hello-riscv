#![no_std]
#![no_main]

use core::arch::global_asm;
use panic_halt as _;

fn main() {
    //println!("Hello, world!");
}

#[link_section = ".init.rust"]
#[export_name = "_start_rust"]
pub unsafe extern "C" fn start_rust() -> ! {
    main();

    _exit(0);
}

pub unsafe extern "C" fn _exit(rc: u32) -> ! {
    extern "C" {
        static mut tohost: u32;
    }
    loop {
        unsafe {
            tohost = (rc << 1) | 1;
        };
    }
}

// cf. https://github.com/riscv/riscv-test-env/blob/1c577dc7c7d6aee27b8d5cb0e2e87c8473e3ad12/p/riscv_test.h#L168
global_asm!(
    r#"

/*
    Entry point of all programs (_start).

    It initializes DWARF call frame information, the stack pointer, the
    frame pointer (needed for closures to work in start_rust) and the global
    pointer. Then it calls _start_rust.
*/

.section .init, "ax"
.global _start

_start:
    /* Jump to the absolute address defined by the linker script. */
    j _abs_start

_abs_start:
    .option norelax
    .cfi_startproc
    .cfi_undefined ra

    li  x1, 0
    li  x2, 0
    li  x3, 0
    li  x4, 0
    li  x5, 0
    li  x6, 0
    li  x7, 0
    li  x8, 0
    li  x9, 0
    li  x10,0
    li  x11,0
    li  x12,0
    li  x13,0
    li  x14,0
    li  x15,0
    li  x16,0
    li  x17,0
    li  x18,0
    li  x19,0
    li  x20,0
    li  x21,0
    li  x22,0
    li  x23,0
    li  x24,0
    li  x25,0
    li  x26,0
    li  x27,0
    li  x28,0
    li  x29,0
    li  x30,0
    li  x31,0

    la gp, __global_pointer$

    // Allocate stacks
    la sp, _stack_start
    // Leaving this as the spelled-out `la` pseudoinstruction, because although
    // the linker only speaks in terms of addresses (i.e. there's no such thing
    // as a link-time constant, just a symbol at a fixed address), it still
    // feels a little weird to say "load the address of the size" here.
    lui t0, %hi(_hart_stack_size)
    add t0, t0, %lo(_hart_stack_size)

    beqz t2, 2f  // Jump if single-hart
    mv t1, t2
    mv t3, t0
1:
    add t0, t0, t3
    addi t1, t1, -1
    bnez t1, 1b
2:
    sub sp, sp, t0

    // Set frame pointer
    add s0, sp, zero

    jal zero, _start_rust

    .cfi_endproc

    .pushsection .tohost,"aw",@progbits;
    .align 6; .global tohost; tohost: .dword 0;
    .align 6; .global fromhost; fromhost: .dword 0;
    .popsection;
"#
);
