#[allow(non_snake_case)]
pub fn knapsack_01(w: Vec<usize>, v: Vec<u64>, W: usize) -> u64 {
    let mut dp = vec![0; W + 1];
    for (&w, &v) in w.iter().zip(v.iter()) {
        for i in (w..=W).rev() {
            dp[i] = dp[i].max(dp[i - w] + v);
        }
    }
    dp[W]
}

#[allow(non_snake_case)]
pub fn knapsack_01_2(w: Vec<u64>, v: Vec<usize>, W: u64) -> usize {
    let sum = v.iter().sum();
    let mut dp = vec![W + 1; sum + 1];
    dp[0] = 0;
    for (&w, &v) in w.iter().zip(v.iter()) {
        for i in (v..=sum).rev() {
            dp[i] = dp[i].min(dp[i - v] + w);
        }
    }
    dp.into_iter().rposition(|x| x <= W).unwrap()
}
