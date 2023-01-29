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
    PROBLEM: https://judge.yosupo.jp/problem/aplusb
    links:
    - https://judge.yosupo.jp/problem/aplusb
  bundledCode: "Traceback (most recent call last):\n  File \"/opt/hostedtoolcache/Python/3.11.1/x64/lib/python3.11/site-packages/onlinejudge_verify/documentation/build.py\"\
    , line 71, in _render_source_code_stat\n    bundled_code = language.bundle(stat.path,\
    \ basedir=basedir, options={'include_paths': [basedir]}).decode()\n          \
    \         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\
    \  File \"/opt/hostedtoolcache/Python/3.11.1/x64/lib/python3.11/site-packages/onlinejudge_verify/languages/rust.py\"\
    , line 288, in bundle\n    raise NotImplementedError\nNotImplementedError\n"
  code: "// verification-helper: PROBLEM https://judge.yosupo.jp/problem/aplusb\n\n\
    use proconio::input;\n\nfn main() {\n    input! {\n        a: u64,\n        b:\
    \ u64,\n    }\n    println!(\"{}\", a + b);\n}\n"
  dependsOn:
  - crates/dp/src/lib.rs
  isVerificationFile: true
  path: examples/a-plus-b.rs
  requiredBy: []
  timestamp: '2023-01-30 00:36:58+09:00'
  verificationStatus: TEST_ACCEPTED
  verifiedWith: []
documentation_of: examples/a-plus-b.rs
layout: document
redirect_from:
- /verify/examples/a-plus-b.rs
- /verify/examples/a-plus-b.rs.html
title: examples/a-plus-b.rs
---
