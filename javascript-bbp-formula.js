// https://en.wikipedia.org/wiki/Bailey%E2%80%93Borwein%E2%80%93Plouffe_formula

var i = 0,
    numSteps = 11,
    piSummation = 0.0;

for(i = 0; i < numSteps; i++) {
    piSummation += (1.0/Math.pow(16.0, i)) * ((4.0/(8.0*i + 1.0)) - (2.0/(8.0*i + 4.0)) - (1.0/(8.0*i + 5.0)) - (1.0/(8.0*i + 6.0)));
}

console.log("Pi = " + piSummation.toFixed(15));
