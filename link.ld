# cf. https://github.com/riscv/riscv-test-env/blob/master/p/link.ld

OUTPUT_ARCH( "riscv" )
ENTRY(_start)

SECTIONS
{
  . = 0x80000000;
  .text.init : { *(.init); *(.init*) }

  .tohost : { KEEP(*(.tohost)) }

  .text : { *(.text) }
  .data : { *(.data) }
  .bss : { *(.bss) }
  .rodata : { *(.rodata*) }

  . = ALIGN(0x1000);
  _end = .;

  .eh_frame (INFO) : { *(.eh_frame) }
  .eh_frame_hdr (INFO) : { *(.eh_frame_hdr) }
}

PROVIDE(_max_hart_id = 0);
PROVIDE(_hart_stack_size = 2K);
PROVIDE(_stack_start = _end);

ASSERT(_start == 0x80000000, "
BUG(hello-riscv): _start in wrong place");
