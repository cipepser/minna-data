# 問2.2

## 要件

実装する`RandomQueue`が満たすべきインターフェースは以下2つ。

- `fn add(&mut self, x: T);`
- `fn remove(&mut self) -> Option<T>;`

いずれも`O(1)^A`であること。

また、`remove`は`RandomQueue`内の要素から、一様な確率で要素を選択し、削除する。

## 設計

`add`を定数時間で実装するため、要素数`n`を保持する。  
要素を追加する際には、`heap`の末尾に追加することで定数時間での実行を実現する。

`heap`のサイズが不足する場合には、`resize`を行う。  
`resize`中および初期化には、`allocate`を用いる。  
なお、`resize`および`allocate`は`ArrayStack`と同一の実装とする。

`resize`が発生した場合は、新たに`allocate`した領域に既存の要素をすべてコピーするため、`O(n)`となるが、`resize`が発生するタイミングは`heap`のサイズが`n`を超えた場合のみのため、償却される。

```rust
pub struct RandomQueue<T> {
    pub n: usize,
    pub heap: Box<[T]>,
}
```

`remove`の要素選択には、`rand` creteを用いる。  
`rand`により、乱数を生成し、これを削除する要素のindexとする。  
このindexを単純に用いて要素を削除すると、indexよりもあとの要素を一つずつ前へ移動させる必要がある。  
これには`O(min{i, n-1})`の実行時間を要する。

要件である`O(1)^A`を実現するためのヘルパー関数として`swap`を導入する。  
`swap`は、指定されたindexの要素と`heap`末尾の要素を入れ替える関数である。  
`remove`を実行する際には、`swap`後の末尾の要素を削除する。
削除後の要素数と`heap`のサイズを確認し、`ArrayStack`と同様の条件で`resize`を行う。  
`add`時に言及した通り、`resize`の実行時間は償却される。  
`swap`は定数時間で実行可能であるので、`remove`の要件である一様ランダムに要素を選択しつつ、`O(1)^A`の実行時間を達成する。
