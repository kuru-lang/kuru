// Iterative Greatest Common Divisor `~math.gcd.i`.
fn gcd_i(mut a: u32, mut b: u32) -> u32 {
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
fn gcd_r(mut a: u32, b: u32) -> u32 {
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
