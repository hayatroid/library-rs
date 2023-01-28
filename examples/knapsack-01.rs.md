---
data:
  _extendedDependsOn: []
  _extendedRequiredBy: []
  _extendedVerifiedWith: []
  _isVerificationFailed: false
  _pathExtension: rs
  _verificationStatusIcon: ':warning:'
  attributes:
    PROBLEM: https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B
    links:
    - https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.1/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.1/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DPL_1_B\n\
    \nuse library::dp::*;\nuse proconio::input;\n\n#[allow(non_snake_case)]\nfn main()\
    \ {\n    input! {\n        n: usize,\n        W: usize,\n        vw: [(u32, usize);\
    \ n],\n    }\n    let w = vw.iter().map(|&p| p.1).collect::<Vec<_>>();\n    let\
    \ v = vw.iter().map(|&p| p.0).collect::<Vec<_>>();\n    println!(\"{}\", knapsack_01(w,\
    \ v, W));\n}\n"
  dependsOn: []
  isVerificationFile: false
  path: examples/knapsack-01.rs
  requiredBy: []
  timestamp: '2023-01-29 01:05:50+09:00'
  verificationStatus: LIBRARY_NO_TESTS
  verifiedWith: []
documentation_of: examples/knapsack-01.rs
layout: document
redirect_from:
- /library/examples/knapsack-01.rs
- /library/examples/knapsack-01.rs.html
title: examples/knapsack-01.rs
---
