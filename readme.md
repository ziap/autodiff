# AutoDiff

Single variable automatic differentiation in Rust, with support for:

- Basic arithmetic: $u + v$, $u - v$, $u*v$, $\frac{u}{v}$
- Power: $u^n$, $\sqrt{u}$
- Exponentation: $e^u$
- Trigonometry: $\sin{u}$ and $\cos{u}$
- Inverse trigonometry: $\arctan{u}$
- Logarithm: $\ln{u}$
- Composition: $u \circ v$

## Usage

Create a function with operators and methods

For example: $f(x) = \frac{x^3}{2} + \sin{2x}$

```rust
use autodiff::X;

let f = X.pow(3.0) / 2.0 + (2.0 * X).sin();
```

Compute the value of the function and **its derivative**

For example: $f(3)$ and $f'(3)$

```rust
use autodiff::Fn; // Import the Fn trait to use .eval()

let (value, derivative) = f.eval(3.0);

println!("f(3)  = {value}");      // 13.220585
println!("f'(3) = {derivative}"); // 15.420341
```

Do cool things with the derivative like finding the local minima/maxima

For example: $f(x) = \frac{4x^4}{5} - \frac{3x^3}{2} - x^2 + 2x + \frac{5}{2}$

```rust
let f = 0.8 * X.pow(4.0) - 1.5 * X.pow(3.0) - X.pow(2.0) + 2.0 * X + 2.5;

let mut input = 0.0;

for _ in 0..100 {
    let (_, grad) = f.eval(input);
    
    input -= grad * 0.1;
}

let (output, _) = f.eval(input);
println!("a local minima of f(x) is f({input}) = {output}");
```

## TODO

- Multiple variables
- High order derivative
- Visual examples

## License

This project is licensed under the [MIT License](LICENSE).
