# rustedbytes-bytepusher-rombuilder

BytePusher VM ROM Builder

## NOP256 waste 256 instruction cycles

### Subroutine definition starting at address NOP256

```asm
# NOP16: 16 instructions / 144 bytes
# do nothing for 16 instructions
NOP16     cpy RET_ADDR, NOP16+141  # RET_ADDR overwritten by subroutine caller
NOP16+009 nop
NOP16+018 nop
NOP16+027 nop
NOP16+036 nop
NOP16+045 nop
NOP16+054 nop
NOP16+063 nop
NOP16+072 nop
NOP16+081 nop
NOP16+090 nop
NOP16+099 nop
NOP16+108 nop
NOP16+117 nop
NOP16+126 nop
NOP16+135 jmp 0x000000               # jump address overwritten by the first instruction
```

### Calling the subroutine

nop16

```asm
OFFSET     bbj OFFSET+9, NOP16, NOP16
```
