# c03-sphere

## 一、光线与球体的相交

简单的解析几何，光线 $\mathbf{P}(t) = \mathbf{Q} + t\mathbf{d}$ 和半径为 $r$，中心位于 $\mathbf{C}$ 的球体相交的方程为：
$$
\begin{align}
r^2 &= (\mathbf{C} - \mathbf{P})^2\\
r^2 &= (\mathbf{C} - \mathbf{Q} - t\mathbf{d})^2\\
r^2 &= t^2\mathbf{d}^2 - 2t\mathbf{d}(\mathbf{C} - \mathbf{Q}) + (\mathbf{C} - \mathbf{Q})^2\\
0 &= \mathbf{d}^2t^2 - 2\mathbf{d}(\mathbf{C} - \mathbf{Q})t + (\mathbf{C} - \mathbf{Q})^2 - r^2
\end{align}
$$
根据根的数量即可得到光线与球体的相交情况：



