# --- Day 13: Claw Contraption ---

## Math

We have a system of linear equations:

$$
\begin{cases}
    ax_1 + bx_2 = X \\
    ay_1 + by_2 = Y
\end{cases}
$$

And we want to minimize:

$$
p = 3a + b
$$

Subject to the constraints:

$$
a,b > 0
$$

## Formal Notation

$$
\begin{aligned}
    &\min_{a, b \in \mathbb{N}_0} (3a + b) \\
    &\text{subject to: }
    \begin{cases}
        ax_1 + bx_2 = X, \\
        ay_1 + by_2 = Y, \\
        a, b \geq 0
    \end{cases}
\end{aligned}
$$

## Description

A linear Diophantine system with an objective function.

We need integer solutions to satisfy the equations while minimizing our objective function _p_.

## Reduction

If $(X - ax_1)/x_2 = (Y - ay_1)/y_2$,

cross multiply:

$(X - ax_1)y_2 = (Y - ay_1)x_2$

expand:

$Xy_2 - ax_1y_2 = Yx_2 - ay_1x_2$

collect terms with a:

$-ax_1y_2 + ay_1x_2 = Yx_2 - Xy_2$

$a(y_1x_1 - x_1y_2) = Yx_2 - Xy_2$

therefore:

$a = (Yx_2 - Xy_2)/(y_1x_2 - x_1y_2)$
