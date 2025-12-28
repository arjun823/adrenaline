# Example 1: Basic arithmetic and loops (eligible for SIMD optimization)

def fibonacci(n):
    """Calculate fibonacci number - good candidate for memoization"""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)


def sum_range(n):
    """Sum numbers 1 to n - good for loop optimization"""
    total = 0
    for i in range(n):
        total += i
    return total


def matrix_multiply(size):
    """Matrix multiplication - demonstrates SIMD and parallel opportunities"""
    # #adrenaline:hot
    # #adrenaline:simd
    # #adrenaline:parallel
    result = 0
    for i in range(size):
        for j in range(size):
            result += i * j
    return result


if __name__ == "__main__":
    # Test the functions
    print("Fibonacci(10):", fibonacci(10))
    print("Sum(1000):", sum_range(1000))
    print("Matrix multiply(100):", matrix_multiply(100))
