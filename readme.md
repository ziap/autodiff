# AutoDiff

Single variable automatic differentiation in Rust, with support for:

- Basic arithmetic ($u + v$, $u - v$, $u*v$, $\frac{u}{v}$)
- Power ($u^n$, $\sqrt{u}$)
- Exponentation ($e^u$)
- Trigonometry ($\sin{u}$ and $\cos{u}$)
- Logarithm ($\ln{u}$)

## Usage

Create a variable (or an identity function $f(x) = x$)

```rust
use diff::Var;

let x = Var::get();
```

Use operators and methods to compose functions

For example: $f(x) = \frac{x^3}{2} + \sin{2x}$

```rust
let f = x.pow(3.0) / 2.0 + (2.0 * x).sin();
```

Compute the value of the function and **its derivative**

For example: $f(3)$ and $f'(3)$

```rust
let (value, derivative) = f.eval(3.0);

println!("f(3)  = {value}");      // 13.220585
println!("f'(3) = {derivative}"); // 15.420341
```

Do cool things with the derivative like finding the local min/max using gradient descent

```rust
let mut input = 0.0;

for _ in 0..100 {
    let (_, grad) = f.eval(input);
    
    input -= grad * 0.1;
}

let (output, _) = f.eval(input);
println!("min of f(x) = {output} with x = {input}");
```

## TODO

- Multiple variables
- High order derivative
- Visual examples

## License

This project is licensed under the [MIT License](LICENSE).
