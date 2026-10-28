ENTRY(_start)

PHDRS
{
    text PT_LOAD FLAGS(5);
    data PT_LOAD FLAGS(6);
}

SECTIONS
{
    . = 0x100000;

    .text ALIGN(0x1000) :
    {
        *(.text .text.*)
    } :text

    .rodata ALIGN(0x1000) :
    {
        *(.rodata .rodata.*)
        *(.eh_frame_hdr)
        *(.eh_frame)
    } :data

    .data ALIGN(0x1000) :
    {
        *(.data .data.*)
        *(.data.rel.ro .data.rel.ro.*)
        *(.got .got.*)
    } :data

    .bss ALIGN(0x1000) (NOLOAD) :
    {
        *(COMMON)
        *(.bss .bss.*)
    } :data
}
