// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_F

use dp::knapsack_01_2;
use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        W: u64,
        vw: [(usize, u64); n],
    }
    let w = vw.iter().map(|&p| p.1).collect::<Vec<_>>();
    let v = vw.iter().map(|&p| p.0).collect::<Vec<_>>();
    println!("{}", knapsack_01_2(w, v, W));
}
