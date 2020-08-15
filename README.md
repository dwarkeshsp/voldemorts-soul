(not yet finished)

Splits a message into parts and encrypts the parts such they can only be decrypted and reassembled if you have all the parts.

Kinda like Voldemort's soul, where Harry had to get all the horcruxes (parts of the Voldemort's soul) in order to kill the dark lord. Except we're not destroying the message so much as decrypting it :).

### How it works

Our goal is to split up a file into parts and then encrypt these chunks in such a way that they can only be unencrypted by having every chunk. 

### Explanation

The xor operation is reversible, meaning (A ⊕ B) ⊕ B = A.

I found that a binary xor tree is also reversible. If you xor the root of an xor tree with all the leaves, and then construct a new xor tree with these xored leaves, you get the same root! 

Let's say we split a message into four strings, A, B, C, D. If we make an xor tree of out of these strings.

​						root

​				/						\

​		A ⊕ B						C ⊕ D

​	/			\					/			\

A				B				C				D

and encrypt our strings by xoring them with the root such that A' = A ⊕ root, we will get a tree with the same root:

​							root

​				/							\

​		A' ⊕ B'						C' ⊕ D'

​	/			\						/			\

A'				B'				C'				D'

So if we have all the encrypted strings A', B', ..., we make the original xor tree root. By xoring our encrypted strings with the root, we can decrypt our strings, since A' ⊕ root = A. **To put is simply, since the root of the xor tree is the same for the decrypted and encrypted leaves, we use the root to both encrypt and decrypt the leaves.**

### Proof of reversibility of binary xor tree

Let's consider the simple case of an xor tree with two leaf string

​	A ⊕ B

​	/		\

A			B

If we wanted to encrypt our strings by xoring them with the root as explained above, we would get 

A' = A ⊕ (A ⊕ B) = (A ⊕ A) ⊕ B = 0 ⊕ B = B

B' = B ⊕ (A ⊕ B) = (B ⊕ B) ⊕ A = 0 ⊕ A = A

If we make an xor tree out of our encrypted strings, we get:

​	A' ⊕ B' = A ⊕ B

​	/					\

A' = B			B' = A

Therefore, the root of the original tree (A ⊕ B) equals the root or the encrypted tree (A' ⊕ B'). 

This proof can be generalized to any sized binary xor tree by treated branches as a single leaf node, for example: 

let A = 					

​	a ⊕ b		

​	/		\				

a			b	

and let B = 

​	c ⊕ d		

​	/		\				

c			d	

This allows us to treat:

​				(a ⊕ b) ⊕ (c ⊕ d) 

​				/						\

​		a ⊕ b						c ⊕ d

​	/			\					/			\

a				b				c				d

as: 

​	A ⊕ B

​	/		\

A			B

which we already proved was reversible.