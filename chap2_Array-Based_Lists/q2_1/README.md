# 問2.1

本章で言及されたデータ構造がまとめられた表を引用する。

![image](https://user-images.githubusercontent.com/10915207/88262306-e7c07580-cd02-11ea-9742-ede6eaac5ee0.png)

※P.25より引用

いずれのデータ構造も`add(i, x)`の操作に、`O(n-i)`または`O(min{i, n-i})`の実行時間を要する。  
Collection`c`の要素数を`N`としたとき、`addAll(i, c)`を、`add(i, x)`を繰り返し実行することで実装すると、`O(N(n-i))`または`O(N * min{i, n-i})`の実行時間を要してしまい、効率がよくない。


