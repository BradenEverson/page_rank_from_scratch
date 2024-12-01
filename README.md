# A Linear Algebraic Approach to the PageRank Algorithm

This project contains everything necessary for a super simple search engine with result priority based on the PageRank algorithm. This includes:

1. A lightweight linear algebra library that contains type-system abiding Matrix and Vector types. Including stochastic matrices and the ability to find a steady state solution.
2. A graph-based relationship struct that can take node connections with travel probabilities and find the highest 'ranked' node based on the steady state solution of the stochastic representation
3. A web crawler that can find and log all connections within a page
4. PageRank implementation that brings all of this together to cater web pages based on title, sorted by highest ranking as the algorithm deems accurate.
5. A RataTui terminal app that allows for searching a term and viewing the catered results (with vim motions of course)

![sample search engine](https://github.com/user-attachments/assets/10584ede-1c16-4ae8-948d-1c0f649f6fff)

## Problem Introduction
The PageRank algorithm aims to solve the problem of curating a ranked list of search results when we have such a huge pool of data such as the internet. This specific approach attempts to leverage this data by creating a large graph of all sites that link to other sites within the searched term's domain. The more a site is linked to, the higher it's ranking is. On top of that as a result, the more higher ranked sites that point to your site, the even higher of a weight you'll receive.

This problem can actually be represented very well as an NxN stochastic matrix with each column vector representing the probabilities given you're on site i that you'll go to any other site within the domain. Some more fun math stuff is applied on top of this like the ability to randomly move to a different link not from the site you're on to get a final formula of

Given some Stochastic Matrix A that represents a site dependency graph and some Matrix B that has all entries of `1 / N` (for example if B is a 5x5 matrix, all entries would be 1/5):

```
Rankings(A) = null((0.85A + 0.15B) - I)
```

As such, to correctly represent this in code, we need the following to be implemented in our barebones linear algebra library:

1. Matrices
2. Scalar matrix multiplication
3. Matrix addition
4. Identity matrix construction (convenience)
5. Null Space
6. Reduced Row Echelon Form

## Linear Algebra System
The first component of this simple Linear Algebra "library" is an implementation for Matrices and Vectors. Rust's type system makes these things extremely nice to represent, as we can define an MxN matrix as a unique struct with generic constants for M and N. Therefore, if we then define an operation such as MxN matrix to MxN matrix addition, the type system automatically forces only same-dimensioned matrices to be compatible (compile time error versus runtime). Furthermore, in this implementation we kick things up a notch by also including a generic type parameter for the Matrix's type(General, RREF, Stochastic, etc), allowing us the bar certain method (such as steady state solution) to matrices that hold these elevated labels. 

The basic definition of this Matrix type goes as follows

```Rust
pub struct Matrix<const M: usize, const N: usize, TYPE: Debug = General> {
    data: [[f32; N]; M],
    phantom_type: PhantomData<TYPE>,
}
```

And by default contains methods for getting the identity matrix of MxN-land, alongside methods for going into reduced row echelon form and checking/elevating to a stochastic matrix. Further, scalar multiplication and matrix addition are implemented and are implemented to the appropriate operation traits, allowing expressions such as `A + B` or `A * 2.0` to be valid.

<!--## Graph Based Stochastic Matrix Derivation

## Web Crawler

## PageRank implementation

## RataTui Interface -->
