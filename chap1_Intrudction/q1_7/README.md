# 問1.7

## List

![add](https://user-images.githubusercontent.com/10915207/87259461-6be65200-c4e6-11ea-9b13-8217ca26b369.png)

![add+remove](https://user-images.githubusercontent.com/10915207/87259464-6ee14280-c4e6-11ea-95e9-538035ad9fb4.png)

## USetとSSetの`find(x)`の操作性能向上について

今回、自分で実装した`find(x)`は、USetとSSetいずれも`O(N)`である。

`add`操作を末尾に要素を追加する方式で実装を行った。  
`find(x)`操作は配列に対する線形探索となっている。  
`add`操作時に整列し、配列に追加し、`find(x)`では二分探索を用いることで`O(logN)`に改善できる。  
`add`操作のコストが大きくなるので、`add`頻度は高いが`find(x)`操作が多いアプリケーションなどでは有用だろう。
