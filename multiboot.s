/* multiboot2 header */
.section .multiboot
.align 8

.long 0xe85250d6          /* multiboot2 magic */
.long 0                  /* architecture (0 = i386, also used for x86_64) */
.long header_end - header_start
.long -(0xe85250d6 + 0 + (header_end - header_start))

header_start:
    /* end tag */
    .short 0
    .short 0
    .long 8
header_end:

