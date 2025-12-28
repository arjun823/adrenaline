# ⚡ Adrenaline: Python → Rust → Native Compiler

A production-grade compiler that transforms slow Python code into blazingly fast native executables. Achieve **10–1000× speedups** automatically with zero annotations.

## 🎯 Core Features

### 1. **Intelligent Compilation Pipeline**

```bash
Python → AST → Type Inference → IR → Optimization → Rust → Native Binary
```

### 2. **Compiler Directives** (Optional)

Guide compilation with simple Python comments:

```python
def hot_function():
    """
    #adrenaline:hot           # Aggressive optimization
    #adrenaline:simd          # Auto-vectorization
    #adrenaline:parallel      # Multi-threading with Rayon
    #adrenaline:inline        # Inline at call sites
    #adrenaline:no-compile    # Fall back to Python
    #adrenaline:cache         # Cache compiled output
    """
    # Your code here
    pass
```

### 3. **Automatic Intelligence**

- **Type Inference**: Automatically infer int, float, array types
- **Hot Path Detection**: Profile and recompile hot functions at higher optimization levels
- **SIMD Detection**: Identify numeric loops suitable for vectorization
- **Parallelization**: Safe auto-threading of independent loops
- **Fallback Execution**: Unsupported Python features automatically fall back to CPython

### 4. **Real Optimizations**

- Loop unrolling
- Constant folding
- Dead code elimination
- Bounds check elimination (proven-safe accesses)
- Common subexpression elimination
- SIMD vectorization
- Parallel execution (Rayon)
- Function inlining for hot paths
- Memoization for pure functions

### 5. **CLI Interface**

```bash
adrenaline build main.py              # Compile Python to native
adrenaline run main.py arg1 arg2      # Execute compiled binary
adrenaline check main.py              # Check for compilation issues
adrenaline cache clear                # Clear compilation cache
adrenaline help directives            # Show directive syntax
adrenaline help features              # Show supported features
```

## 📦 Installation

### Prerequisites

- Rust 1.70+ with rustc
- Python 3.10+
- Cargo

### Build from Source

**Windows (PowerShell):**

```powershell
git clone https://github.com/yourusername/adrenaline.git
cd adrenaline
.\build.ps1
```

**Linux/macOS:**

```bash
git clone https://github.com/yourusername/adrenaline.git
cd adrenaline
chmod +x build.sh
./build.sh
```

The compiled binary will be placed in `dist/adrenaline` (or `dist/adrenaline.exe` on Windows).

## 🚀 Quick Start

### Example 1: Simple Loop

```python
# examples/basic.py
def sum_range(n):
    total = 0
    for i in range(n):
        total += i
    return total

if __name__ == "__main__":
    print(sum_range(1000000))
```

Compile and run:

```bash
adrenaline build examples/basic.py
adrenaline run examples/basic.py
```

### Example 2: Hot Functions

```python
# examples/directives.py
def hot_compute(iterations):
    """
    #adrenaline:hot
    #adrenaline:simd
    Intensive computation - marked for aggressive optimization
    """
    result = 0
    for i in range(iterations):
        result += (i * i) % 97
    return result

if __name__ == "__main__":
    print(hot_compute(10000000))
```

### Example 3: Fallback for Unsupported Features

```python
# examples/fallback.py
def use_dict():
    """
    #adrenaline:no-compile
    Dictionary operations aren't yet supported - fallback to Python
    """
    data = {"key": "value"}
    return data.get("key")

def regular_function(x):
    # This will be compiled
    return x * x + 2 * x + 1

if __name__ == "__main__":
    print(use_dict())
    print(regular_function(42))
```

## 📊 Supported Python Features

### ✅ Fully Supported

- Function definitions and calls
- Local and global variables
- Numeric types: `int`, `float`
- Lists and arrays
- For/while loops
- If/elif/else conditionals
- Binary and unary operators (`+`, `-`, `*`, `/`, `//`, `%`, `**`, `&`, `|`, `^`, `<<`, `>>`)
- Comparison operators (`==`, `!=`, `<`, `<=`, `>`, `>=`)
- Type inference
- Local imports

### ⏳ Planned Support

- Classes and OOP
- Generators and iterators
- Decorators (beyond directives)
- Global state management
- Dictionary/set operations
- String manipulation (partial)

### ❌ Unsupported (Use `#adrenaline:no-compile`)

- Advanced Python features (metaclasses, descriptors)
- Dynamic code generation
- Complex context managers

## 🔧 Architecture

```bash
src/
├── main.rs              # CLI entry point
├── cli.rs               # Command-line interface (clap)
├── parser.rs            # Python source parsing
├── ast_types.rs         # AST type definitions
├── type_inference.rs    # Type inference engine
├── ir.rs                # Intermediate representation
├── optimizer.rs         # IR optimization passes
├── codegen.rs           # Rust code generation
├── compiler.rs          # Main compilation pipeline
├── directives.rs        # Compiler directive system
├── profiler.rs          # Runtime profiling
├── runtime.rs           # Runtime support
├── cache.rs             # SHA256-based compilation cache
└── diagnostics.rs       # Error reporting (miette)
```

## 🎯 Optimization Levels

The compiler automatically applies different optimization strategies:

- **Basic** (default): Constant folding, dead code elimination
- **Aggressive** (hot functions): Loop unrolling, SIMD, bounds check elimination
- **Extreme** (deeply profiled): All of the above + function inlining, escape analysis

## 📈 Performance Tips

1. **Profile First**: Mark hot functions with `#adrenaline:hot`
2. **Use Numeric Types**: Prefer `int`/`float` over generic collections
3. **Enable SIMD**: Use `#adrenaline:simd` for vectorizable loops
4. **Parallelize**: Use `#adrenaline:parallel` for independent iterations
5. **Cache Results**: Enable `#adrenaline:cache` for expensive functions

## 🔄 Compilation Cache

Adrenaline caches compiled outputs based on source code SHA256 hash. Clear cache with:

```bash
adrenaline cache clear
```

## 🛠️ Development

### Build Debug Version

```bash
cargo build
./target/debug/adrenaline build examples/basic.py
```

### Run Tests

```bash
cargo test
```

### Check for Issues

```bash
cargo clippy
cargo fmt
```

## 📄 License

MIT / Apache-2.0 (dual licensed)

## 🤝 Contributing

Contributions welcome! Areas of focus:

- Python AST parsing improvements
- Additional optimization passes
- More language feature support
- Performance benchmarking

## 📚 References

- [Rust Book](https://doc.rust-lang.org/book/)
- [Python AST Module](https://docs.python.org/3/library/ast.html)
- [LLVM for Optimization](https://llvm.org/)

---

## Made with ⚡ for speed-obsessed Python developers
