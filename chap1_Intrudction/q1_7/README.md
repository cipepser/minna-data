# 問1.7

## List


!(add)[https://gyazo.com/b88dcfef16e0632a7b240a8bbb84e67f]

!(add+remove)[https://gyazo.com/ec695eb6acf1bea7f292fde5874388fa]

## USetとSSetの`find(x)`の操作性能向上について

今回、自分で実装した`find(x)`は、USetとSSetいずれも`O(N)`である。

`add`操作を末尾に要素を追加する方式で実装を行った。  
`find(x)`操作は配列に対する線形探索となっている。  
`add`操作時に整列し、配列に追加し、`find(x)`では二分探索を用いることで`O(logN)`に改善できる。  
`add`操作のコストが大きくなるので、`add`頻度は高いが`find(x)`操作が多いアプリケーションなどでは有用だろう。
