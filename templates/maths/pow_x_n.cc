/*
 * Binary exponentiation computes x raised to n in logarithmic time.
 * The absolute value of n must fit in int. Use a nonzero base for a negative exponent.
 */

#include <cstdlib>

double solve(double x, int n) {
    if (n == 0)
        return 1;

    if (n == 1)
        return x;
    
    double res = solve(x * x, n / 2);
    return n & 1 ? res * x : res;
}

double pow(double x, int n) {
    double res = solve(x, std::abs(n));
    return n < 0 ? 1 / res : res;
}
  
