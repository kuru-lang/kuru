/// Matrices
pub mod matrix;

pub fn matrix_chain_order(p: [u32; 7]) -> ([[u32;7];7], [[usize;6];7]) {
    #[allow(non_upper_case_globals)] 
    const n: usize = 6;

//    let n = p.len() - 1;
    let mut m = [[0; n + 1]; n + 1];
    let mut s = [[0; n]; n + 1];

    for l in 2..=n {
        for i in 1..=(n-l+1) {
            let j = i + l - 1;
            m[i][j] = std::u32::MAX;
            for k in i..=(j-1) {
                let q = m[i][k] + m[k + 1][j] + p[i-1] * p[k] * p[j];
                if q < m[i][j] {
                    m[i][j] = q;
                    s[i-1][j-1] = k;
                }
            }
        }
    }

    (m, s)
}

pub fn print_optimal_parens(s: [[usize;6];7], i: usize, j: usize) {
    if i == j {
        print!("A{}", i);
    } else {
        print!("(");
        print_optimal_parens(s,i,s[i-1][j-1]);
        print_optimal_parens(s,s[i-1][j-1]+1,j);
        print!(")");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
