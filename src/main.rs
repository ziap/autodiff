use autodiff::{Fn, X};

fn main() {
    {
        println!("Basic derivative calculation");
        println!("f(x) = x^3 / 2 + sin(2x)");
        println!("g(x) = 3x + 5");
        println!("h(x) = f(g(x))");

        println!();

        let f = X.pow(3.0) / 2.0 + (2.0 * X).sin();
        let g = X / 3.0 - 5.0;
        let h = f.compose(g);

        let (value, derivative) = h.eval(25.0);

        println!("h(25)  = {value}");
        println!("h'(25) = {derivative}");
    }

    println!();

    {
        println!("Solving for x^2 = 2^x");
        let f_x = X.pow(2.0);
        let g_x = (X * f32::ln(2.0)).exp();

        let mut input = 0.0;

        for _ in 0..100 {
            let cost = (f_x - g_x).pow(2.0);

            let (_, derivative) = cost.eval(input);

            input -= derivative * 0.1;
        }

        let (y1, _) = f_x.eval(input);
        let (y2, _) = g_x.eval(input);

        println!("f({input}) = {y1}");
        println!("g({input}) = {y2}");
    }

    println!();

    {
        println!("Find the max of sin(x) + cos(x)");
        let f_x = X.sin() + X.cos();
        let mut input = 0.0;

        for _ in 0..100 {
            let (_, derivative) = f_x.eval(input);

            input += derivative * 0.1;
        }

        let (y, _) = f_x.eval(input);
        println!("sin({input}) + cos({input}) = {y}");
    }
}
