# 問2.5

`ArrayDeque`と`DualArrayDeque`に`rotate(r)`を実装する。  
実行時間を`O(1+min{r, n-r})`とする。

`resize`および`balance`が償却されることに注意し、以下図のように実装する。

## ArrayDeque

![image](https://user-images.githubusercontent.com/10915207/88446943-3851e300-ce69-11ea-85e5-4234da31b623.png)

## DualArrayDeque

