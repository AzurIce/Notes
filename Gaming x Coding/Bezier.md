# 贝塞尔曲线

贝塞尔曲线由一系列  *控制点* $\mathbf{P}_0, \dots, \mathbf{P}_n$ 定义，其中 $n$ 被称为曲线的阶数/次数。

显然，$\mathbf{P}_0$ 和 $\mathbf{P}_n$ 永远都是曲线的端点，然而中间的控制点不一定在曲线上。

## 定义

### 1. 一阶（线性）贝塞尔曲线

![Thumb](./assets/240px-Bézier_1_big.gif)

给定两个 *控制点* $\mathbf{P}_0$ 与 $\mathbf{P}_1$，一个一阶贝塞尔曲线就是两点之间的一条线段，非常简单：
$$
\mathbf{B}_{01}(t)
= \mathbf{P}_0 + t(\mathbf{P}_1 - \mathbf{P_0})
= (1-t)\mathbf{P}_0 + t\mathbf{P}_1
,\;\; 0 \leq t \leq 1
$$
其实，这和线性插值完全一样。

### 2. 二阶贝塞尔曲线

![Thumb](./assets/240px-Bézier_2_big.gif)

那么二阶贝塞尔曲线就由三个 *控制点* $\mathbf{P}_0, \mathbf{P}_1, \mathbf{P}_2$ 定义：
$$
\mathbf{B}_{012}(t)
= (1-t)\mathbf{B}_{01}(t) + t\mathbf{B}_{12}(t)\\
= (1-t)\left[(1-t)\mathbf{P}_0 + t\mathbf{P}_1\right] + t\left[(1-t)\mathbf{P}_1 + t\mathbf{P}_2\right]
,\;\; 0 \leq t \leq 1
$$
二阶贝塞尔曲线的每个点可以看作是由 $\mathbf{P}_0, \mathbf{P}_1$ 以及 $\mathbf{P}_1, \mathbf{P}_2$ 定义的两个一阶贝塞尔曲线上的对应点再进行一次线性插值得到。