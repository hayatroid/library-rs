---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
  - icon: ':heavy_check_mark:'
    path: examples/knapsack-01-2.rs
    title: examples/knapsack-01-2.rs
  - icon: ':heavy_check_mark:'
    path: examples/knapsack-01.rs
    title: examples/knapsack-01.rs
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    links: []
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.1/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.1/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "#[allow(non_snake_case)]\npub fn knapsack_01(w: Vec<usize>, v: Vec<u64>,\
    \ W: usize) -> u64 {\n    let mut dp = vec![0; W + 1];\n    for (&w, &v) in w.iter().zip(v.iter())\
    \ {\n        for i in (w..=W).rev() {\n            dp[i] = dp[i].max(dp[i - w]\
    \ + v);\n        }\n    }\n    dp[W]\n}\n\n#[allow(non_snake_case)]\npub fn knapsack_01_2(w:\
    \ Vec<u64>, v: Vec<usize>, W: u64) -> usize {\n    let sum = v.iter().sum();\n\
    \    let mut dp = vec![W + 1; sum + 1];\n    dp[0] = 0;\n    for (&w, &v) in w.iter().zip(v.iter())\
    \ {\n        for i in (v..=sum).rev() {\n            dp[i] = dp[i].min(dp[i -\
    \ v] + w);\n        }\n    }\n    dp.into_iter().rposition(|x| x <= W).unwrap()\n\
    }\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/dp/src/lib.rs
  requiredBy: []
  timestamp: '2023-01-30 00:11:35+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - examples/knapsack-01-2.rs
  - examples/knapsack-01.rs
documentation_of: crates/dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/dp/src/lib.rs
- /library/crates/dp/src/lib.rs.html
title: crates/dp/src/lib.rs
---
