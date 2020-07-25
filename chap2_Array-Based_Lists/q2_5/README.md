# 問2.5

`ArrayDeque`に`rotate(r)`を実装する。  
実行時間を`O(1+min{r, n-r})`とする。

`resize`が償却されることに注意し、以下図のように実装する。

![image](https://user-images.githubusercontent.com/10915207/88446943-3851e300-ce69-11ea-85e5-4234da31b623.png)


`DualArrayDeque`も端に近い方から要素を移動させ、`balance`を実行することで実装できる。