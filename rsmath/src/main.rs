// Iterative Greatest Common Divisor `~math.gcd.i`.
pub fn gcd_i(mut a: u32, mut b: u32) -> u32 {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }

    loop {
        a %= b;
        if a == 0 {
            return b;
        }
        b %= a;
        if b == 0 {
            return a;
        }
    }
}

// Recursive Greatest Common Divisor `~math.gcd.r`.
pub fn gcd_r(mut a: u32, b: u32) -> u32 {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }

    let new_b = a % b;
    a = b;

    if new_b == 0 {
        a
    } else {
        gcd_r(a, new_b)
    }
}

/// Unsigned 8 Bit Fixed Point (4.4: 0-31)
pub struct Uf1(u8);

/// Signed 8 Bit Fixed Point (4/3.4: -16-15)
pub struct Sf1(i8);

/// Unsigned 16 Bit Fixed Point (8.8: 0-255)
pub struct Uf2(u16);

/// Signed 16 Bit Fixed Point (8/7.8: -128-127)
pub struct Sf2(i16);

/// Unsigned 32 Bit Fixed Point (8.8: 0-65,535)
pub struct Uf4(u32);

/// Signed 32 Bit Fixed Point (8/7.8: TODO)
pub struct Sf4(i32);

/// Unsigned 64 Bit Fixed Point (8.8: TODO)
pub struct Uf8(u32);

/// Signed 54 Bit Fixed Point (8/7.8: TODO)
pub struct Sf8(i32);

// The Types:
//    Uf1 / Sf1 / Nf1 - Unsigned/Signed/Normalized(0-1) 1 byte (4.4) Fixed Point
//    Uf2 / Sf2 / Nf2 - Unsigned/Signed/Normalized(0-1) 2 byte (8.8) Fixed Point
//    Uf4 / Sf4 / Nf4 - Unsigned/Signed/Normalized(0-1) 4 byte (16.16) Fixed Point
//    Uf8 / Sf8 / Nf8 - Unsigned/Signed/Normalized(0-1) 8 byte (32.32) Fixed Point
//    Uf_ / Sf_ / Nf_ - Unsigned/Signed/Normalized(0-1) Big Fixed Point
//    Ui1 / Si1 - Unsigned/Signed 1 byte Integer
//    Ui2 / Si2 - Unsigned/Signed 2 byte Integer
//    Ui4 / Si4 - Unsigned/Signed 4 byte Integer
//    Ui8 / Si8 - Unsigned/Signed 8 byte Integer
//    Ui_ / Si_ - Unsigned/Signed Big Int
//    Hx1 - Hexadecimal 1 byte.
//    Hx2 - Hexadecimal 2 byte.
//    Hx4 - Hexadecimal 4 byte.
//    Hx8 - Hexadecimal 8 byte.
//    Bi1 - Binary 1 byte.
//    Bi2 - Binary 2 byte.
//    Bi4 - Binary 4 byte.
//    Bi8 - Binary 8 byte.
//    Chr - Character (4 bytes unicode)
//    Lst<T> - List of T (Dynamic Size)
//    Cxy - 8 bytes Coordinates X and Y: (Sf4, Sf4).
//    Vec - 16 bytes A Vector or Position: (Sf4, Sf4, Sf4, Sf4)
//    Txt - Text (Wrapper around Lst<Chr>)
//    Opt<T> - Optional (Y (yes) or N (no))
//          :$x Opt Y # Create a Y or N type:1 byte
//          :$x Opt Y(45) # Create a Optional Si4 type (value -2147483648 is used to store N):4 byte
//    Ref<T> - Reference
//    Fnc<T, U> - Function reference.  Generic T is return type, Generic U is tuple of parameters.
//    (_, _) Tuple
//    [_, _] Array
//    {code} Either a code block or parenthesis for math: { $x $x / {$y + $z} }.

/// A 4-byte general purpose number in range: [-32_766, 32_766]
///
/// -32_768 = NaN
/// 32_767 = +Infinity
/// -32_767 = -Infinity
pub struct Num(i32);

impl Num {
    pub fn new(whole: i16, fraction: u16) -> Num {
        Num4((whole as i32 << -16) | (fraction as i32))
    }
}

/// 

fn main() {
    println!("gcd(1, 25) {} {} {} {}", gcd_r(1, 25), gcd_r(25, 1), gcd_i(1, 25), gcd_i(25, 1));
    println!("gcd(5, 25) {} {} {} {}", gcd_r(5, 25), gcd_r(25, 5), gcd_i(5, 25), gcd_i(25, 5));
    println!("gcd(6, 12) {} {} {} {}", gcd_r(6, 12), gcd_r(12, 6), gcd_i(6, 12), gcd_i(12, 6));
    println!("gcd(6, 24) {} {} {} {}", gcd_r(6, 24), gcd_r(24, 6), gcd_i(6, 24), gcd_i(24, 6));
    println!("gcd(6, 25) {} {} {} {}", gcd_r(6, 25), gcd_r(25, 6), gcd_i(6, 25), gcd_i(25, 6));
    println!("gcd(3, 9) {} {} {} {}", gcd_r(3, 9), gcd_r(9, 3), gcd_i(3, 9), gcd_i(9, 3));
    println!("gcd(6, 9) {} {} {} {}", gcd_r(6, 9), gcd_r(9, 6), gcd_i(6, 9), gcd_i(9, 6));
    println!("gcd(24, 36) {} {} {} {}", gcd_r(24, 36), gcd_r(36, 24), gcd_i(24, 36), gcd_i(36, 24));
    println!("gcd(0, 36) {} {} {} {}", gcd_r(0, 36), gcd_r(36, 0), gcd_i(0, 36), gcd_i(36, 0));
}
