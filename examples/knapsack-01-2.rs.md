---
data:
  _extendedDependsOn:
  - icon: ':heavy_check_mark:'
    path: crates/dp/src/lib.rs
    title: crates/dp/src/lib.rs
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':heavy_check_mark:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_F
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_F
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.1/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.1/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_F\n\
    \nuse dp::knapsack_01_2;\nuse proconio::input;\n\n#[allow(non_snake_case)]\nfn\
    \ main() {\n    input! {\n        n: usize,\n        W: u64,\n        vw: [(usize,\
    \ u64); n],\n    }\n    let w = vw.iter().map(|&p| p.1).collect::<Vec<_>>();\n\
    \    let v = vw.iter().map(|&p| p.0).collect::<Vec<_>>();\n    println!(\"{}\"\
    , knapsack_01_2(w, v, W));\n}\n"
  dependsOn:
  - crates/dp/src/lib.rs
  isVerificationFile: true
  path: examples/knapsack-01-2.rs
  requiredBy: []
  timestamp: '2023-01-30 00:18:30+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: examples/knapsack-01-2.rs
layout: document
redirect_from:
- /verify/examples/knapsack-01-2.rs
- /verify/examples/knapsack-01-2.rs.html
title: examples/knapsack-01-2.rs
---
