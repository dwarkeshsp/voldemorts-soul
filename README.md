# Voldemort's Soul

Splits a message up and encrypts the parts such they can only be decrypted and reassembled if you have all the encrypted parts.

Maybe you're part of a secret society. You don't trust any member individually to keep a secret message safe, so you use this program to encrypt and split up the message and then give every member one of these encrypted chunks. You can only decrypt the message if every single member gives you a copy of their chunk.

I named it Voldemort's soul because it's like when Harry had to get every single horcruxes in order to kill the dark lord. Except we're not destroying the message so much as decrypting it :).

## Goal

Our goal is to split up a file into parts and then encrypt these chunks in such a way that they can only be unencrypted by having every chunk. Here's how we'll do it:

## Method

##### TLDR: Encrypt and decrypt by xoring blocks of the message with the root of the xor tree they form.

The xor operation is reversible, meaning (A ⊕ B) ⊕ B = A. I found that a binary xor tree is also reversible in the following sense. If you xor the root of a binary xor tree with all the leaves, and then construct a new xor tree with these xored leaves, you get the same root xor! 

For example, let's say we split a message into four strings, A, B, C, D. If we make an xor tree of out of these strings:

![img](https://raw.githubusercontent.com/dwarkeshsp/voldemorts-soul/bf04a7b8d26c1776c9ce8acfee25a9375d1daec9/diagrams/xor-Page-1.svg)

and encrypt our strings by xoring them with the root such that A' = A ⊕ root, we will get a tree with the same exact root:

![img](https://raw.githubusercontent.com/dwarkeshsp/voldemorts-soul/bf04a7b8d26c1776c9ce8acfee25a9375d1daec9/diagrams/xor-Page-2.svg)

So if we have all the encrypted strings A', B', ..., we can make the original xor tree root. Since xor is reversible (A' ⊕ root = A) , by xoring our encrypted strings with the root, we can decrypt these encrypted strings. **To put is simply, since the root of the xor tree is the same with decrypted and encrypted strings, we can use that root to both encrypt and decrypt these strings. And since we need every string in order to create the correct root,  we ensure that all the parts of message are present before it are decrypted**

## Proof of reversibility of binary xor tree

Let's consider the simple case of an xor tree with two leaf string

![img](https://raw.githubusercontent.com/dwarkeshsp/voldemorts-soul/6f43657923ab790153913f8b422343c853c51a52/images/3.svg)

If we wanted to encrypt our strings by xoring them with the root as explained above, we would get 

**A' = A ⊕ (A ⊕ B) = (A ⊕ A) ⊕ B = 0 ⊕ B = B**

**B' = B ⊕ (A ⊕ B) = (B ⊕ B) ⊕ A = 0 ⊕ A = A**

If we make an xor tree out of our encrypted strings, we get:

![img](https://raw.githubusercontent.com/dwarkeshsp/voldemorts-soul/6f43657923ab790153913f8b422343c853c51a52/images/4.svg)

Therefore, the root of the original tree (A ⊕ B) equals the root or the encrypted tree (A' ⊕ B'). 

This proof can be generalized to any sized binary xor tree by collapsing branches until you arrive at the simple case discussed above. For each of the branches (and their subbranches), the simple case applies as well. Therefore, the entire tree can be treated as the simple case which we just proved is reversible.

![img](https://raw.githubusercontent.com/dwarkeshsp/voldemorts-soul/6f43657923ab790153913f8b422343c853c51a52/images/5.svg)