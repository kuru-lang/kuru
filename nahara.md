# Nahar Assembly
This is an intermediate state between Nahar and native assembly code (ARM,RISV-V,X86).

## Registers
There are 32 64-bit registers, register represented by 6 bits.

```
$0     $zero        Zero Register (always zero).                                  [READ-ONLY]
$1     $hi          High Register / Overflow ($d when SHIFT=-64)                  [READ-ONLY]
$2     $lo          Low Register / Underflow ($d when SHIFT=0)                    [READ-ONLY]
$3     $in          Indicator Register (1 if overflow/underflow $d when SHIFT=64) [READ-ONLY]
$4     $pc          Program Counter (6 bytes used for address)
$5     $sp          Stack Pointer (6 bytes used for address)
$6     $ra          Return Address (6 bytes used for address)
$7     $tmp         Temporary Register
$8-$31 $r0-$r23     General Use Registers
```

## Instructions
All instructions are 48 bits.  Opcode is 8 bits.  Style of instructions:
```
NONE [
	OPCODE:4, UNUSED:44
]
IMM. [
	OPCODE:3, SRC_IS_IMM=1:1, SIGNED:1, RESERVED:1, SRC:5, DST:5, IMM_VAL:32
]
	DST : SRC ~ IMM_VAL (for some operation ~)
VEC. [
	OPCODE:3, SRC_IS_IMM=0:1, SIGNED:1, RESERVED:1, SRC:5, DST:5, DST_END:5, SRCB:5, USE_SRC: 1,
	DATA_SIZE(8,16,32,64 bytes):2, BYTE_MASK:8, RESERVED:2, PAR:1, NEGATE:1, SHIFT(-64 to 63):7
]
	USE_SRC=1
		[DST to DST_END] : (SRC ~ SRCB) << SHIFT
	USE_SRC=0
		[DST to DST_END] : (SELF ~ SRCB) << SHIFT
```

Only 8 opcodes:

| 0       | 1         |
|---------|-----------|
| 000 NOP | 100 ADD   |
| 001 SYS | 101 MUL   |
| 010 GET | 110 LOGIC |
| 011 SET | 111       |

Different forms:
```
# NOP
nop

# SYS: 

# GET:

# SET:

# ADD: add & sub
# OPCODE=100 SRC_IS_IMM=0 SIGNED=0-1 USE_SRC=1 DATA_SIZE=64 BYTE_MASK=1111_1111 NEGATE=0-1 SHIFT=0
addu64 $d, $sa, $sb, shift # d : sa + sb          {SIGNED=0 PAR=0 NEGATE=0}
adds64 $d, $sa, $sb, shift # d : sa + sb          {SIGNED=1 PAR=0 NEGATE=0}
subu64 $d, $sa, $sb, shift # d : sa + (-sb)       {SIGNED=0 PAR=0 NEGATE=1}
subs64 $d, $sa, $sb, shift # d : sa + (-sb)       {SIGNED=1 PAR=0 NEGATE=1}
addu64 $d, $sa, $sb, shift # d : sa + sb          {SIGNED=0 PAR=1 NEGATE=0}
adds64 $d, $sa, $sb, shift # d : sa + sb          {SIGNED=1 PAR=1 NEGATE=0}
sltu64 $d, $sa, $sb, shift # d : (sa + (-sb)).neg {SIGNED=0 PAR=1 NEGATE=1} 65-bit sign extend
slts64 $d, $sa, $sb, shift # d : (sa + (-sb)).neg {SIGNED=1 PAR=1 NEGATE=1}

# MUL: mul(128 bit result HI(overflow) and LO, $d:result(HILO) << SHIFT=-64 thru 0)
#    & div(128 bit result HI and LO(underflow), $d:result(HILO) << SHIFT=-64 thru 0)
#    & snz
#    & inv(64 bit result HI and 64 bit remainder LO, $d:result(HILO) << SHIFT=-64 or 0)
#
# MUL: $hi:128-64, $lo:63-0, $in: 1 if overflowed ans
# DIV: $hi:128-64, $lo:63-0, $in: modulo value
# OPCODE=101 ...
mulu64 $d, $sa, $sb, shift # d : sa * sb           {SIGNED=0 PAR=0 NEGATE=0}
muls64 $d, $sa, $sb, shift # d : sa * sb           {SIGNED=1 PAR=0 NEGATE=0}
divu64 $d, $sa, $sb, shift # d : sa /% sb          {SIGNED=0 PAR=0 NEGATE=1}
divs64 $d, $sa, $sb, shift # d : sa /% sb          {SIGNED=1 PAR=0 NEGATE=1}
svnz64 $d, $sa, $sb, shift # d : sa * (sb=0)       {SIGNED=0 PAR=1 NEGATE=0} "Set Value on Non-Zero"
ssnz64 $d, $sa, $sb, shift # d : sa * ((sb=0)*2-1) {SIGNED=1 PAR=1 NEGATE=0} "Set Sign on Non-Zero"
invu64 $d, $sa, shift      # d : 1.0 / sa          {SIGNED=0 PAR=1 NEGATE=1}
invs64 $d, $sa, shift      # d : 1.0 / sa          {SIGNED=1 PAR=1 NEGATE=1}

# LOGIC: and & nand & butnot & imply & or & nor & xor & xnor
# OPCODE=110 SRC_IS_IMM=0 SIGNED=0-1 USE_SRC=1 DATA_SIZE=64 BYTE_MASK=1111_1111 NEGATE=0-1 SHIFT=0
and64 $d, $sa, $sb    # d : sa & sb                    {SIGNED=0 PAR=0 NEGATE=0}
nand64 $d, $sa, $sb   # d : !(sa & sb)                 {SIGNED=1 PAR=0 NEGATE=0}
butnot64 $d, $sa, $sb # d : sa & !sb                   {SIGNED=0 PAR=0 NEGATE=1}
imply64 $d, $sa, $sb  # d : !(sa & !sb)                {SIGNED=1 PAR=0 NEGATE=1} = (!sa | sb)
or64 $d, $sa, $sb     # d : sa | sb                    {SIGNED=0 PAR=1 NEGATE=0}
nor64 $d, $sa, $sb    # d : !(sa | sb)                 {SIGNED=1 PAR=1 NEGATE=0}
xor64 $d, $sa, $sb    # d : (sa & !sb) | (!sa & sb)    {SIGNED=0 PAR=1 NEGATE=1}
xnor64 $d, $sa, $sb   # d : !((sa & !sb) | (!sa & sb)) {SIGNED=1 PAR=1 NEGATE=1}


# Other Variants
addu64 # DOUBLE_WORD
addu32_01 # LO WORD
addu32_10 # HI WORD
addu32_11 # BOTH WORDS
addu16_0000
addu8_00000000
```

Zeroing Data:
```asm
#################################
# Set one $r register to zero.  #
#################################

# TODO: Change this to be correct
# Left Shift 64 bits
#
# `opc: xor, imm:0, vec:0, dst:$8, src:$8, val: 64`
# `1100 `
adds64 $d, $zero, 0

#################################
# Set all $r registers to zero. #
#################################

# TODO: Change this to be correct
# Set mask.
#
# `add imm:1 mask:0 $3 $0 00000000_11111111_11111111_11111111`
# `0010 1 0 00011 00000 00000000_11111111_11111111_11111111`
add $tmp $zero 0b_00000000_11111111_11111111_11111111 # syntax for immediate instructions.

# Vector Left Shift 64 bits on all the registers in the mask stored in $tmp.
#
# `ls imm:1`
# `1001 `
adds64 [$d $e] $zero 0 # syntax for vectorized instructions.
```

Logic Stuff:
```
 A | B | and | nand | butnot | imply | or | nor | xor | xnor |
---|---|-----|------|--------|-------|----|-----|-----|------|
 0 | 0 | 0   | 1    | 0      | 1     | 0  | 1   | 0   |  1   |
 0 | 1 | 0   | 1    | 0      | 1     | 1  | 0   | 1   |  0   |
 1 | 0 | 0   | 1    | 1      | 0     | 1  | 0   | 1   |  0   |
 1 | 1 | 1   | 0    | 0      | 1     | 1  | 0   | 0   |  1   |

# Chart of all Logical Operations (Truth Tables).
0000 Falsity (No Instruction)
0001 "and $d, $a, $b"
0010 "butnot $d, $a, $b" (Nonimplication)
0011 Identity: A
0100 "nimply $d, $b, $a" (But Not, Reverse Parameters)
0101 Identity: B
0110 "xor $d, $a, $b" (Nonequivalence)
0111 "or $d, $a, $b"
1000 "nor $d, $a, $b"
1001 "xnor $d, $a, $b" (Equivalence)
1010 "nor $d, $b, $zero" (XNOR, IMPLY also works for NOT B)
1011 "imply $d, $b, $a" (Implication, Reverse Parameters)
1100 "nor $d, $a, $zero" (XNOR, IMPLY also works for NOT A)
1101 "imply $d, $a, $b" (Implication)
1110 "nand $d, $a, $b"
1111 Tautology (No instruction)
```

Instruction List:
```
Instruction         OPCODE   BITS    DESCRIPTION     INFO
nop                 0000     NONE    Do nothing.
----------------------------------------------------------------------------------------------------
sys $d, $sa, imm    0001     DATA    Sycall.         $d : south_bridge.send_and_recv imm, $sa
----------------------------------------------------------------------------------------------------
adds $d, $sa, $sb   0010     MATH    Add Signed      $d : $sa + $sb (SIGNED 1, OP_PAR 0, VEC 0)
subs $d, $sa, $sb   0010     MATH    Subtract Signed $d : $sa - $sb (SIGNED 1, OP_PAR 1, VEC 0)
addu $d, $sa, $sb   0010     MATH    Add Unsigned    $d : $sa + $sb (SIGNED 0, OP_PAR 0, VEC 0)
subu $d, $sa, $sb   0010     MATH    Sub. Unsigned   $d : $sa - $sb (SIGNED 0, OP_PAR 1, VEC 0)
addvs $sa, mask     0010     MATH    vector adds     {RMASK} +: $sa (SIGNED 1, OP_PAR 0, VEC 1)
subvs $sa, mask     0010     MATH    vector subs     {RMASK} -: $sa (SIGNED 1, OP_PAR 1, VEC 1)
addvu $sa, mask     0010     MATH    vector addu     {RMASK} +: $sa (SIGNED 0, OP_PAR 0, VEC 1)
subvu $sa, mask     0010     MATH    vector subu     {RMASK} -: $sa (SIGNED 0, OP_PAR 1, VEC 1)
----------------------------------------------------------------------------------------------------
muls $sa, $sb       0011     MATH    Multiply Signed $hi-lo: $sa*$sb(SIGNED 1,OP_PAR 0,VEC 0)
mulu $sa, $sb       0011     MATH    Mul. Unsigned   $hi-lo: $sa*$sb(SIGNED 0,OP_PAR 0,VEC 0)
mulfs $sa, $sb      0011     MATH    muls fixed pt.  $hi-lo: $sa*$sb>>32(SIGNED 1,OP_PAR 1,VEC 0)
mulfu $sa, $sb      0011     MATH    mulu fixed pt.  $hi-lo: $sa*$sb>>32(SIGNED 0,OP_PAR 1,VEC 0)
mulvs $sa, mask     0011     MATH    vector muls     {RMASK} *: $sa (SIGNED 1,OP_PAR 0,VEC 1)
mulvu $sa, mask     0011     MATH    vector mulu     {RMASK} *: $sa (SIGNED 0,OP_PAR 0,VEC 1)
mulvfs $sa, mask    0011     MATH    vector mulfs    {RMASK} *: $sa>>32 (SIGNED 1,OP_PAR 1,VEC 1)
mulvfu $sa, mask    0011     MATH    vector mulfu    {RMASK} *: $sa>>32 (SIGNED 0,OP_PAR 1,VEC 1)
----------------------------------------------------------------------------------------------------
addis $d, imm       0110     DATA    Immediate adds  $d +: imm.SgnExt (SIGNED 1, OP_PAR 0, VEC 0)
addiu $d, imm       0110     DATA    Immediate addu  $d +: imm.SgnExt (SIGNED 0, OP_PAR 0, VEC 0)
addhs $d, imm       0110     DATA    High word addis $d +: imm << 32 (SIGNED 1, OP_PAR 1, VEC 0)
addhu $d, imm       0110     DATA    High word addiu $d +: imm << 32 (SIGNED 0, OP_PAR 1, VEC 0)
addvis $sa, imm     0110     DATA    Immediate adds  {$sa}+: imm.SgnExt (SIGNED 1,OP_PAR 0,VEC 1)
addviu $sa, imm     0110     DATA    Immediate addu  {$sa}+: imm.SgnExt (SIGNED 0,OP_PAR 0,VEC 1)
addvhs $sa, imm     0110     DATA    High word addis {$sa}+: imm << 32 (SIGNED 1,OP_PAR 1,VEC 1)
addvhu $sa, imm     0110     DATA    High word addiu {$sa}+: imm << 32 (SIGNED 0,OP_PAR 1,VEC 1)
----------------------------------------------------------------------------------------------------
divs $sa, $sb       0111     MATH    Divide Signed   $hi-lo: $sa*%$sb(SIGNED 1,OP_PAR 0,VEC 0)
divu $sa, $sb       0111     MATH    Div. Unsigned   $hi-lo: $sa*%$sb(SIGNED 0,OP_PAR 0,VEC 0)
invfs $d, $sa       0111     MATH    invs fixed pt.  $d : 1.0 / $sa (SIGNED 1,OP_PAR 1,VEC 0)
invfu $d, $sa       0111     MATH    invu fixed pt.  $d : 1.0 / $sa (SIGNED 0,OP_PAR 1,VEC 0)
divvs $sa, mask     0111     MATH    vector divs     {RMASK} *%: $sa (SIGNED 1,OP_PAR 0,VEC 1)
divvu $sa, mask     0111     MATH    vector divu     {RMASK} *%: $sa (SIGNED 0,OP_PAR 0,VEC 1)
invvfs mask         0111     MATH    vector invfs    {RMASK}: 1.0 / $sa (SIGNED 1,OP_PAR 1,VEC 1)
invvfu mask         0111     MATH    vector invfu    {RMASK}: 1.0 / $sa (SIGNED 0,OP_PAR 1,VEC 1)
----------------------------------------------------------------------------------------------------

----------------------------------------------------------------------------------------------------
pullv $sa, mask     11_0000     MATH    pull $r0-$r23   {RMASK} : {RAM[$sa]} (OP_PAR 0, VEC 1)
pushv $sa, mask     11_0000     MATH    push $r0-$r23   {RAM[$sa]} : {RMASK} (OP_PAR 1, VEC 1)
```

### Instruction Cache
Quadruple-buffered instruction cache.  The four caches may change places: when next becomes current,
current becomes previous and previous gets loaded with new next.
```
IC[0] Kernel Instructions Cache
IC[1] Previous Instructions
IC[2] Current Instructions Cache
IC[3] Next Instructions
```

## Address
Memory addresses are 64 bits.
```
// Memory addresses are 40-bits / 5 bytes.  32 bits gives ~4 GB of memory, so 40 bits gives ~1 TB of
// memory.
```




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
