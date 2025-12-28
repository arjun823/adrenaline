def fibonacci(n):
    if n <= 1:
        return n
    a, b = 0, 1
    for i in range(n - 1):
        a, b = b, a + b
    return b

def sum_range(n):
    total = 0
    for i in range(n):
        total = total + i
    return total

def main():
    print(fibonacci(10))
    print(sum_range(100))

if __name__ == "__main__":
    main()
