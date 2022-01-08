# Almost PI

The number `𝛑=3.1415926535897...` is the ratio of a circle's circumference to its diameter and shows basically everywhere in math and science.

It's like the most famous number in math. It's decimal expansion goes on forever but we can actually calculate it by summing up a bunch of fractions

```
                                                                    k
pi           1     1     1     1      1              __ oo    ( - 1) 
--  =  1  -  -  +  -  -  -  +  -  -  --  +  .... =  \         -------
 4           3     5     7     9     11             /__ k=0    2k + 1 

```

By adding more and more terms, you get a better and better approximation to `𝛑`. Of course, you can keep summing an infinite number of terms so in practice you cut it off after  terms. 

Given the number of terms `n`, sum the first `n` terms and return the result. Remember the `k=0` term (equal to 1) counts as the first term.

```
Input: An integer `n` for the number of terms to sum in calculating `𝛑` using the above equation.

Output: The sum of the first `n` terms giving an approximation to `𝛑`.
```