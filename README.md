# Optimized arithmetic operations for small values in Arkworks

This library provides efficient arithmetic operations for small values in R1CS circuits implemented in Arkworks.
The main idea is to avoid expensive comparisons and instead use auxiliary slack variables.

Below is a description of the `min` gadget, which computes the minimum of two small field elements, which presents the core concept of this library.

## 1. Problem Statement

Let $\mathbb{F}$ be a prime field and fix an integer $\ell$ with $\ell < \lfloor\log_{2}|\mathbb{F}|\rfloor$.
Given two inputs $a, b \in \mathbb{F}$ satisfying $0 \le a, b < 2^{\ell}$, we want to
compute $c = \min(a, b)$ as a field element.

*Note:* By $\ell < \log_{2} \lfloor|\mathbb{F}|\rfloor$ we assume $a,b$ do not exceed $|\mathbb{F}|/2$, so no modular wraparound
occurs.

## 2. Protocol

Introduce two auxiliary witnesses $over$ and $under$ and enforce the following constraints:

1. **(Balance constraint)**  
   $a + under = b + over$
2. **(Mutual-exclusion constraint)**  
   $over \cdot under = 0$
3. **(Bit-length bounds)**  
   $over,under < 2^{\ell}$

Finally, output $c=a-over$

## 3. Correctness

1. From constraint 2, at most one of $over$ or $under$ is nonzero.
2. Since $\ell < \log_{2} \lfloor|\mathbb{F}|\rfloor$, adding $under$ or $over$ to $a$ or $b$ never wraps around the modulus: summing two $\ell$-bit numbers will yield a number strictly less than $|\mathbb{F}|$.

- **Case $a \le b$**  
  Balance implies $a + under = b + over$, forcing $over = 0$ (via mutual exclusion).  
  Hence $\min(a,b) = a - over = a$

- **Case $a > b$**  
  Then $under = 0$ and $over = a - b$.  
  Hence $\min(a,b) = a - over = b$.

## 4. R1CS Realization

Constraint 1 and constraint 2 each require one rank-1 constraint (R1C).

Constraint 3 (bit-length check) uses a standard bit-decomposition.
For $i = 0,\dots,\ell-1$, introduce Boolean variables $over_{i}$ and $under_{i}$ and enforce:

- $over = \sum_{i=0}^{\ell-1}\ 2^{i} \cdot over_{i}$
- $under = \sum_{i=0}^{\ell-1}\ 2^{i} \cdot under_{i}$
- $over_{i} \cdot (1 - over_{i}) = 0$
- $under_{i} \cdot  (1 - under_{i}) = 0$

Each of these four equations is one R1C.

### 4.1 Gadget Overhead

| Component                 |       Count |
|---------------------------|------------:|
| Witness variables         | $2\ell + 2$ |
| Rank-1 constraints (R1Cs) | $2\ell + 4$ |

## 5. Empirical Evaluation

| Bits ($\ell$) | Lib Gadget Constraints | Lib Gadget Vars | Std Gadget Constraints | Std Gadget Vars |
|:-------------:|-----------------------:|----------------:|-----------------------:|----------------:|
|       2       |                     12 |              13 |                   1920 |            1458 |
|       4       |                     16 |              17 |                   1920 |            1458 |
|       8       |                     24 |              25 |                   1920 |            1458 |
|      16       |                     40 |              41 |                   1920 |            1458 |
|      32       |                     72 |              73 |                   1920 |            1458 |
|      64       |                    136 |             137 |                   1920 |            1458 |
|      128      |                    264 |             265 |                   1920 |            1458 |
|      250      |                    508 |             509 |                   1920 |            1458 |

`Lib Gadget *` refers to a circuit using the above idea.
`Std Gadget *` refers to a circuit using the standard approach of comparing $a$ and $b$.

To reproduce, run `cargo run --release`

## 6. Extensions

This idea can be adapted for computing other functions like maximum or absolute difference.
