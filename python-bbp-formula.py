#!/usr/bin/env python 

# https://en.wikipedia.org/wiki/Bailey%E2%80%93Borwein%E2%80%93Plouffe_formula

import math

num_steps = 11
pi_summation = 0.0

for i in range(0, num_steps):
    pi_summation += (1.0/math.pow(16.0, i)) * ((4.0/(8.0*i + 1.0)) - (2.0/(8.0*i + 4.0)) - (1.0/(8.0*i + 5.0)) - (1.0/(8.0*i + 6.0)))

print("Pi = {0:.20f}".format(pi_summation))
