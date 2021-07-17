use nmath::*;

fn main() {
    let (_m,s) = matrix_chain_order([5,10,3,12,5,50,6]);
    print_optimal_parens(s, 1, 7 - 1);
}
