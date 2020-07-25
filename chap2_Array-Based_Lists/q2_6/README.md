# 問2.6

[std::ptr::copy](https://doc.rust-lang.org/beta/std/ptr/fn.copy.html)を用いる。

- `add`: `a[0..i]`を左へ1つずつずらす or `a[i..n-1]`を右へ1つずつずらす
- `remove`: `a[0..i-1]`を右へ1つずつずらす or `a[i+1..n-1]`を左へ1つずつずらす
- `resize`: `a[j..j+n-1]`を`b[0..n-1]`にcopy

`resize`はoverlapしないので、`copy_nonoverlapping`を利用する。