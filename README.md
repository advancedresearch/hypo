# Hypo - Automatic hypothesis testing

This library consists of various algorithms for automatic hypothesis testing.

### Introduction

In the scientific method, one formulates a falsifiable hypothesis
which is tested against experiments.

Instead of designing one experiment to test a single hypothesis,
one can design an experiment to eliminate as many hypotheses as possible.
This method is somewhat efficient when hypotheses are easy to generate
and computationally fast to make predictions.

Automatic hypothesis testing can be used when hypotheses are generated
in advance and the experiments can be configured and executed automatically.

### Optimal guesses

An optimal guess is an experiment which is picked based on its capability
to eliminate potentially false hypotheses.

Assume that an experiment simply returns a conclusion `true` or `false`.
Before performing the experiment, the conclusion is unknown.

To maximize the number of eliminated false hypotheses,
one exploits the property that the conclusion of the experiment is yet unknown.
The optimal guess is picked such that competing hypotheses disagree with each other.

The ideal experiment is one that eliminates half the hypotheses
no matter what the conclusion of the experiment will be.
This works in a similar way to binary search,
except that hypotheses do not need to be sorted.
In practice one can choose the experiment for which the number of hypotheses
that returns `true` is as close as possible to half the total number of hypotheses.

In [path semantical notation](https://github.com/advancedresearch/path_semantics),
the following measure is minimized over `e`:

```text
abs(|h : (prediction e)| - |Hypothesis| / 2)

h : Hypothesis
e : Experiment
prediction : Hypothesis x Experiment -> bool
```

By repeating such experiments,
one is more likely to minimize the number of experiments required
to eliminate as many false hypotheses as possible.
