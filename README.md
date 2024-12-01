# A Linear Algebraic Approach to the PageRank Algorithm

This project contains everything necessary for a super simple search engine with result priority based on the PageRank algorithm. This includes:

1. A web crawler that can find and log all connections within a page
2. A lightweight linear algebra library that contains type-system abiding Matrix and Vector types. Including stochastic matrices and the ability to find a steady state solution.
3. A graph-based relationship struct that can take node connections with travel probabilities and find the highest 'ranked' node based on the steady state solution of the stochastic representation
4. PageRank implementation that brings all of this together to cater web pages based on title, sorted by highest ranking as the algorithm deems accurate.
5. A RataTui terminal app that allows for searching a term and viewing the catered results (with vim motions of course)

![sample search engine](https://github.com/user-attachments/assets/10584ede-1c16-4ae8-948d-1c0f649f6fff)
