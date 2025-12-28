# Example 2: Hot function with aggressive optimization directives

def hot_compute(iterations):
    """
    #adrenaline:hot
    #adrenaline:inline
    Compute-intensive loop - marked as hot path for aggressive optimization
    """
    result = 0
    for i in range(iterations):
        result += (i * i) % 97
    return result


def numeric_loop(count):
    """
    #adrenaline:simd
    Numeric loop suitable for SIMD vectorization
    """
    total = 0.0
    for i in range(count):
        total += float(i) * 1.5
    return total


def process_list(items):
    """
    #adrenaline:parallel
    Process list in parallel
    """
    result = 0
    for item in items:
        result += item * 2
    return result


if __name__ == "__main__":
    print("Hot compute(10000):", hot_compute(10000))
    print("Numeric loop(5000):", numeric_loop(5000))
    print("Process list:", process_list([1, 2, 3, 4, 5, 10, 20]))
