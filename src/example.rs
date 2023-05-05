mod autodiff;
use autodiff::{Differentiable, Var};

fn main() {
    let x = Var::get();

    {
        println!("Basic derivative calculation");
        println!("f(x)  = x^3 / 2 + sin(2x)");
        let f = x.pow(3.0) / 2.0 + (2.0 * x).sin();

        let (value, derivative) = f.eval(3.0);

        println!("f(3)  = {value}");
        println!("f'(3) = {derivative}");
    }

    println!();

    {
        println!("Solving for x^2 = 2^x");
        let f_x = x.pow(2.0);
        let g_x = (x * f32::ln(2.0)).exp();

        let mut input = 0.0;

        for _ in 0..100 {
            let cost = (f_x - g_x) * (f_x - g_x);

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
        let f_x = x.sin() + x.cos();
        let mut input = 0.0;

        for _ in 0..100 {
            let (_, derivative) = f_x.eval(input);

            input += derivative * 0.1;
        }

        let (y, _) = f_x.eval(input);
        println!("sin({input}) + cos({input}) = {y}");
    }
}
