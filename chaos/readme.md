# Chaos
Chaos theory and loops

Let's look at an example of chaos arising from simplicity in nature: the logistic map. It's a simplified model used to describe population sizes and can be written mathematically as

```
xn + 1 = rxn(1-xn)
```

where `0 < r < 4` is a parameter and `0 < xn < 1` represents the population size at time step . Starting with an initial population size of `x0 = 0.5` and `r` as an input parameter, return a list of the first 50 values of the logistic map . You should see different behavior depending on the value of , but we'll leave you to play around with it and we'll have more to say in the solution notes.

