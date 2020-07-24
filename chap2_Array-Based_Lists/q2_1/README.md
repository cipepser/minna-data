# 問2.1

本章で言及されたデータ構造がまとめられた表を引用する。

![image](https://user-images.githubusercontent.com/10915207/88262306-e7c07580-cd02-11ea-9742-ede6eaac5ee0.png)

※P.25より引用

いずれのデータ構造も`add(i, x)`の操作に、`O(n-i)`または`O(min{i, n-i})`の実行時間を要する。  
Collection`c`の要素数を`N`としたとき、`addAll(i, c)`を、`add(i, x)`を繰り返し実行することで実装すると、`O(N(n-i))`または`O(N * min{i, n-i})`の実行時間を要してしまい、効率がよくない。  
形式的には、`O(N(n-i))`または`O(N*min{i, n-i})`の実行時間を要する。

ArrayStackを例に、効率的に実装することを考える。
Collection`c`の要素数を`N`がわかっているので、`add`操作を行う際に、1つずつ右にずらすのではなく、`N`だけまとめて右にずらす。
これによって`O(N + n-i)`の実行時間にできる。

![image](https://user-images.githubusercontent.com/10915207/88371640-67b41180-cdcf-11ea-8ab3-288c8e0db2d3.png)

## ベンチマーク

以下にベンチマーク結果を示す。  
どちらも`N`に対して線形だが、効率的な実装のほうが傾きは小さくなっている。  
なお、図は測定をし直したので、CLIの結果と数値は異なることに注意。

### #collection = 1

```sh
addAll benchmarks, #collection=1000/Benchmark array_stack
                        time:   [143.73 us 149.17 us 153.74 us]
Benchmarking addAll benchmarks, #collection=1000/Benchmark array_stack_kai: Collecting 100 samples in estimated 5.0356                                                                                                                      addAll benchmarks, #collection=1000/Benchmark array_stack_kai
                        time:   [189.16 us 194.09 us 198.32 us]
```

![image](https://user-images.githubusercontent.com/10915207/88375582-caf57200-cdd6-11ea-82d5-19d5833b6fd3.png)

![image](https://user-images.githubusercontent.com/10915207/88375599-d47eda00-cdd6-11ea-9294-985e51fc4cfa.png)

### #collection = 10

```sh
addAll benchmarks, #collection=1000/Benchmark array_stack
                        time:   [1.7941 ms 1.8526 ms 1.9027 ms]
Benchmarking addAll benchmarks, #collection=1000/Benchmark array_stack_kai: Collecting 100 samples in estimated 5.4174                                                                                                                      addAll benchmarks, #collection=1000/Benchmark array_stack_kai
                        time:   [522.34 us 541.98 us 558.98 us]
```

![image](https://user-images.githubusercontent.com/10915207/88375618-da74bb00-cdd6-11ea-801e-88d3306e4784.png)

![image](https://user-images.githubusercontent.com/10915207/88375630-e2345f80-cdd6-11ea-959d-878fe0bb114c.png)

### #collection = 100

```sh
addAll benchmarks/Benchmark array_stack
                        time:   [11.868 ms 12.469 ms 13.073 ms]
Benchmarking addAll benchmarks/Benchmark array_stack_kai: Collecting 100 samples in estimated 6.8799 s (15k iterations                                                                                                                      addAll benchmarks/Benchmark array_stack_kai
                        time:   [2.0656 ms 2.2378 ms 2.3856 ms]
```


![image](https://user-images.githubusercontent.com/10915207/88375645-e791aa00-cdd6-11ea-9015-e95adb8d815d.png)

![image](https://user-images.githubusercontent.com/10915207/88375657-ef514e80-cdd6-11ea-97da-0fb5ef4d7ec8.png)

### #collection = 1000

```sh
addAll benchmarks, #collection=1000/Benchmark array_stack
                        time:   [150.11 ms 157.97 ms 165.98 ms]
Benchmarking addAll benchmarks, #collection=1000/Benchmark array_stack_kai: Collecting 100 samples in estimated 5.2563                                                                                                                      addAll benchmarks, #collection=1000/Benchmark array_stack_kai
                        time:   [6.7171 ms 6.8737 ms 7.0317 ms]
```

![image](https://user-images.githubusercontent.com/10915207/88375918-671f7900-cdd7-11ea-8d9d-243691d61a88.png)

![image](https://user-images.githubusercontent.com/10915207/88375693-fd06d400-cdd6-11ea-9f97-38ef33cc787e.png)
