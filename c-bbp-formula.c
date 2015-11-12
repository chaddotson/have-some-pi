// compile with: gcc c-bbp-formula.c -o c-bbp-formula

// https://en.wikipedia.org/wiki/Bailey%E2%80%93Borwein%E2%80%93Plouffe_formula

#include <stdio.h>
#include <math.h>

int main(int argv, char **argc) {
    int i = 0;
    int numSteps = 11;
    double piSummation = 0.0;

    for(i = 0; i < numSteps; i++) {
        piSummation += (1.0/(pow(16.0, (double)i))) * ((4.0/(8.0*i + 1.0)) - (2.0/(8.0*i + 4.0)) - (1.0/(8.0*i + 5.0)) - (1.0/(8.0*i + 6.0)));
    }

    printf("Pi = %.15f\n", piSummation);
}


