# Nahar Assembly
This is an intermediate state between Nahar and native assembly code (ARM,RISV-V,X86).

## Registers
There are 64 64-bit registers, register represented by 6 bits.

```
$0, $1, $2, $3, $4, $5, $6, $7, $8, $9, $A, $B, $C, $D, $E, $F, ..., $3E, $3F

$0 Zero Register (always zero). [READ-ONLY]
$1 Program Counter (5 bytes).
$2 Error / Overflow
$3 Mask
$4 Stack Pointer
$5 Return Address
$6 High Register
$7 Low Register
$8-$15 (8) 
$16-$31 (16) Temp / Arg / Return
$32-$64 (32) Saved Temporaries
```

This is scaled down if needed, when converted to actual assembly.

## Address
Memory addresses are 40-bits / 5 bytes.  32 bits gives ~4 GB of memory, so 40 bits gives ~1 TB of
memory.

## Instructions
Instructions are 8 bytes.  Opcode is 1 byte.


## ALU Instructions
Prefix `0000`.  Opcodes:

* `0000_0000` - Add (for both signed and unsigned integers)
* `0000_0001` - Subtract (for both signed and unsigned integers)
* `0000_0010` - Bitwise And (data)
* `0000_0011` - Bitwise Or (data)
* `0000_1010` - Bitwise Nand (data) / `AND + NEGATION`
* `0000_1011` - Bitwise Nor (data) / `OR + NEGATION`
* `0000_1111` - Bitwise Xor (data) / `OR AND NAND`

Limited to 32 registers at a time in vector instruction.
`1 byte OPCODE`, `2 bits data size, 6 bits first register`, `2 bits multi-byte, 6 bits last register`, `1 byte mask register`,
`4 bytes immediate value`

### Data Size Bits:
* `00` - 8 byte
* `01` - 4 byte
* `10` - 2 byte
* `11` - 1 byte

### Vectorized Bits:
* `00` - Use Data Size
* `01` - 16 byte (128 bit)
* `10` - 32 byte (256 bit)
* `11` - 64 byte (512 bit)

## Comparison Instructions


```
0x00; 0000_0000 - Hardware Instructions [1B: func, 6B params]
    func 0x00 - (nop)
        [6B unused]
            Does nothing.
    func 0x01 - Power Off (off)
        [6B unused]
            Turns off the computer.
    func 0x02 - Storage Device Read/Write. (sdd)
        [1: Write?, 7: unused][5B Device-Specific App ID]
            Read & Write to SD Card
    func 0x04 - WiFi/Ethernet Device Send/Recv. (net)
        [1: Send?, 1: UDP?, 1: Switch to WiFi, 1: Switch to Data, 4: unused][5B internet protocol]
            Send & Receive WiFi/Data/Ethernet packets
    func 0x08 - USB Device Send/Recv. (usb)
        [1: Send?, 8: unused][6B usb protocol]
            Send & Receive USB packets
    func 0x10 - Audio Read/Write (aud)
        Write Audio for Headphones or Built-In Speaker (depending if aux is plugged in).
        & Read Audio from Microphone
    func 0x20 - Pixels Read/Write. (pix)
        * Read Camera Pixels
        * Write Screen Pixels, Flash
    func 0x40 - Read Buttons, Touchscreen & Accelerometer. (get)
        * Read Input
        * Write Haptic Vibration
    func 0x80 - Read GPS (gps)
        * Read GPS Signal.

0x01; 0001_0001 - ALU Instructions

0x02; 0010_0010 - Comparison Instructions
    0001_0000 - Set On Less Than
    0001_0001 - 

0x10; 0001_0000 - RAM Instructions [3: Control, 5: Register End Offset][8: SpOffset][40: Address]
    Control 0xx: Save Register / SAVE.
    Control 1xx: Load Register / LOAD.

    Control x0x: Register $0 / ABSOLUTE.
    Control x1x: Register $4 / RELATIVE.

    Control xx0: Unused.
    Control xx1: Unused.

    {
        SAVE:
            RELATIVE: // Stack
                *($4) = Registers ($32) through ($32 | Register End Offset)
                $4 = $4 - SpOffset
            ABSOLUTE: // Global
                *(Address) = Registers ($32) through ($32 | Register End Offset)
        LOAD:
            RELATIVE: // Stack
                Registers ($32) through ($32 | Register End Offset) = *($4)
                $4 = $4 + SpOffset
            ABSOLUTE: // Global
                *(Address) = Registers ($32) through ($32 | Register End Offset)
    }
```
