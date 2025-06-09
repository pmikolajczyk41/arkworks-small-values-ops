# Arkworks `min` gadget

This repository provides an alternative implementation for the binary _min_ function, that is not based on comparisons.

# Problem statement

Fix $\mathbb{F}$ and $\ell < \log_2|\mathbb{F}|$. Given $a, b \in \mathbb{F}$ such that $a, b < 2^\ell$,
compute $c = \min(a, b)$.

In other words, we want to compute the minimum of two numbers in a finite field $\mathbb{F}$, where the numbers are
represented with at most $\ell$ bits.

Notice, that we don't support elements over $\frac{|\mathbb{F}|}{2}$.

# Protocol

We introduce two auxiliary witness variables: $over$ and $under$, together with the following constraints:

1. $a + under = b + over$
2. $over · under = 0$
3. $over, under < 2^{\ell}$

As a result, we return $a - over$.

## Correctness

Firstly, notice that:

4. (due to the 2. constraint) only one of the variables $over$ and $under$ can be non-zero
5. (since $\ell < \log_2|\mathbb{F}|$) neither $a+under$ nor $b+over$ can overflow.

### Case: $a \leq b$
- $over = 0$ (from 1, 4 and 5)
- $min(a, b) = a - over = a$

### Case: $a > b$
- $under = 0$ (from 1, 4 and 5)
- $over = a + under - b = a - b$ (from 1)
- $min(a, b) = a - over = b$

## Constraint realization

Constraints 1 and 2 can be realized by a single R1C each.

Constraint 3 can be realized by adding $2·\ell$ new binary witness variables and ensuring that their linear combinations
sum up to $over$ and $under$ respectively.
To be more precise, we introduce $over_i$ and $under_i$ for $i = 0, \ldots, \ell - 1$, and add the following rank-1
constraints (R1C):

- $over = \sum_{i=0}^{\ell - 1} 2^i \cdot over_i$
- $under = \sum_{i=0}^{\ell - 1} 2^i \cdot under_i$
- $over_i \cdot (1-over_i) = 0$ for $i = 0, \ldots, \ell - 1$
- $under_i \cdot (1-under_i) = 0$ for $i = 0, \ldots, \ell - 1$

## Overhead

In total, we introduce: $2 · \ell + 2$ witness variables, and $2·\ell + 4$ R1Cs.

# Evaluation

_(reproducible by running `cargo run --relase`)_
```
Bits      LibConstr      LibVars       StdConstr      StdVars
-------------------------------------------------------------
2                12           13            1920         1458
4                16           17            1920         1458
8                24           25            1920         1458
16               40           41            1920         1458
32               72           73            1920         1458
64              136          137            1920         1458
128             264          265            1920         1458
250             508          509            1920         1458
```

# Broader usage

This idea can be easily adapted for multiple other applications, including _max_ and _abs_.
