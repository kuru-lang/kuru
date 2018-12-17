// Iterative Greatest Common Divisor `.math.gcd.i`.
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

// Recursive Greatest Common Divisor `.math.gcd.r`.
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

/*// Hoare's Quicksort Iterative (middle element is pivot): HQR `.math.sort.hqi`
pub fn sort_hqi(array: &mut [u32]) {
    let stack: Vec<(usize, usize)> = vec![];

    loop {
        if array.len() >> 1 == 0 { return; } // 0 or 1.

        let mut first = std::usize::MAX;
        let mut last = array.len();
        let mid = (last - 1) >> 1; // Shift right to divide by two (floor).
        let piv = array[mid]; // Pivot.

        // Continuously loop (bringing ends closer together).
        let pivot_index = 'gpi: loop {
            'up: loop {
                first = first.wrapping_add(1);
                if !(array[first] < piv) { break 'up }
            }

            'dn: loop {
                last = last.wrapping_sub(1);
                if !(piv < array[last]) { break 'dn }
            }

            if (first - last) >> 1 == 0 { // first is last or last + 1
                break 'gpi last;
            }

            array.swap(first, last);
        } + 1;

        sort_hqr(&mut array[0..pivot_index]);
        let end = array.len();
        sort_hqr(&mut array[pivot_index..end]);
    }
}*/

// Hoare's Quicksort Recursive (middle element is pivot): HQR `.math.sort.hqr`
pub fn sort_hqr(array: &mut [u32]) {
    if array.len() >> 1 == 0 { return; } // 0 or 1.

    let mut first = std::usize::MAX;
    let mut last = array.len();
    let mid = (last - 1) >> 1; // Shift right to divide by two (floor).
    let piv = array[mid]; // Pivot.

    // Continuously loop (bringing ends closer together).
    let pivot_index = 'gpi: loop {
        'up: loop {
            first = first.wrapping_add(1);
            if !(array[first] < piv) { break 'up }
        }

        'dn: loop {
            last = last.wrapping_sub(1);
            if !(piv < array[last]) { break 'dn }
        }

        if (first - last) >> 1 == 0 { // first is last or last + 1
            break 'gpi last;
        }

        array.swap(first, last);
    } + 1;

    sort_hqr(&mut array[0..pivot_index]);
    let end = array.len();
    sort_hqr(&mut array[pivot_index..end]);
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
pub struct Num4(i32);

impl Num4 {
    pub fn new(whole: i16, fraction: u16) -> Num4 {
        Num4(((whole as i32) << -16) | (fraction as i32))
    }
}

/// 

fn main() {
    let mut array_a = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut array_b = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    let mut array_c = [1, 1, 2, 3, 6, 5, 6, 0, 9, 9];
    let mut array_d = [10, 40, 3, 3, 40, 10, 0, 0, 9, 10];
    let mut array_e = [5, 5, 5, 5, 5, 5, 5, 5, 5, 5];

    println!("Unsorted: {:?}", array_a);
    println!("Unsorted: {:?}", array_b);
    println!("Unsorted: {:?}", array_c);
    println!("Unsorted: {:?}", array_d);
    println!("Unsorted: {:?}", array_e);

    sort_hqr(&mut array_a);
    sort_hqr(&mut array_b);
    sort_hqr(&mut array_c);
    sort_hqr(&mut array_d);
    sort_hqr(&mut array_e);

    println!("Sorted: {:?}", array_a);
    println!("Sorted: {:?}", array_b);
    println!("Sorted: {:?}", array_c);
    println!("Sorted: {:?}", array_d);
    println!("Sorted: {:?}", array_e);

/*
    println!("gcd(1, 25) {} {} {} {}", gcd_r(1, 25), gcd_r(25, 1), gcd_i(1, 25), gcd_i(25, 1));
    println!("gcd(5, 25) {} {} {} {}", gcd_r(5, 25), gcd_r(25, 5), gcd_i(5, 25), gcd_i(25, 5));
    println!("gcd(6, 12) {} {} {} {}", gcd_r(6, 12), gcd_r(12, 6), gcd_i(6, 12), gcd_i(12, 6));
    println!("gcd(6, 24) {} {} {} {}", gcd_r(6, 24), gcd_r(24, 6), gcd_i(6, 24), gcd_i(24, 6));
    println!("gcd(6, 25) {} {} {} {}", gcd_r(6, 25), gcd_r(25, 6), gcd_i(6, 25), gcd_i(25, 6));
    println!("gcd(3, 9) {} {} {} {}", gcd_r(3, 9), gcd_r(9, 3), gcd_i(3, 9), gcd_i(9, 3));
    println!("gcd(6, 9) {} {} {} {}", gcd_r(6, 9), gcd_r(9, 6), gcd_i(6, 9), gcd_i(9, 6));
    println!("gcd(24, 36) {} {} {} {}", gcd_r(24, 36), gcd_r(36, 24), gcd_i(24, 36), gcd_i(36, 24));
    println!("gcd(0, 36) {} {} {} {}", gcd_r(0, 36), gcd_r(36, 0), gcd_i(0, 36), gcd_i(36, 0));
*/
}
