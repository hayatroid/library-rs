---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith:
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
  code: "#[allow(non_snake_case)]\npub fn knapsack_01<T>(w: Vec<usize>, v: Vec<T>,\
    \ W: usize) -> T\nwhere\n    T: Copy + std::ops::Add<Output = T> + std::cmp::Ord\
    \ + std::default::Default,\n{\n    let mut dp = vec![T::default(); W + 1];\n \
    \   for (w, v) in w.into_iter().zip(v) {\n        for i in (w..=W).rev() {\n \
    \           dp[i] = dp[i].max(dp[i - w] + v);\n        }\n    }\n    dp[W]\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: crates/dp/src/lib.rs
  requiredBy: []
  timestamp: '2023-01-29 00:04:11+09:00'
  verificationStatus: LIBRARY_ALL_AC
  verifiedWith:
  - examples/knapsack-01.rs
documentation_of: crates/dp/src/lib.rs
layout: document
redirect_from:
- /library/crates/dp/src/lib.rs
- /library/crates/dp/src/lib.rs.html
title: crates/dp/src/lib.rs
---
