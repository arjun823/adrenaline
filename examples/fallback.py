# Example 3: Functions marked for Python fallback

def use_unsupported_feature():
    """
    #adrenaline:no-compile
    This function uses Python features not yet supported by Adrenaline.
    It will fall back to pure Python execution.
    """
    data = {"key": "value", "count": 42}
    # Dictionary operations, lambdas, etc.
    return data.get("count", 0)


def regular_function(x):
    """Regular function without directives - will be compiled if possible"""
    return x * x + 2 * x + 1


def compute_factorial(n):
    """Calculate factorial"""
    if n <= 1:
        return 1
    return n * compute_factorial(n - 1)


if __name__ == "__main__":
    print("Regular(5):", regular_function(5))
    print("Factorial(10):", compute_factorial(10))
    print("Unsupported:", use_unsupported_feature())
