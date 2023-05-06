use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Fn {
    fn eval(self, input: f32) -> (f32, f32);
}

// TODO: Multiple variables using vector
#[derive(Clone, Copy)]
pub struct Var;

impl Fn for Var {
    // f(x) = x, f'(x) = 1
    fn eval(self, input: f32) -> (f32, f32) {
        (input, 1.0)
    }
}

/// Constants
#[derive(Clone, Copy)]
pub struct Const {
    value: f32,
}

impl Fn for Const {
    // f(x) = k, f'(x) = 0
    fn eval(self, _input: f32) -> (f32, f32) {
        (self.value, 0.0)
    }
}

/// Adding 2 expressions
#[derive(Clone, Copy)]
pub struct AddOp<T1: Fn, T2: Fn> {
    lhs: T1,
    rhs: T2,
}

impl<T1: Fn, T2: Fn> Fn for AddOp<T1, T2> {
    // f(x) = u + v, f'(x) = u' + v'
    fn eval(self, input: f32) -> (f32, f32) {
        let (u, du) = self.lhs.eval(input);
        let (v, dv) = self.rhs.eval(input);

        (u + v, du + dv)
    }
}

/// Substracting 2 expressions
#[derive(Clone, Copy)]
pub struct SubOp<T1: Fn, T2: Fn> {
    lhs: T1,
    rhs: T2,
}

impl<T1: Fn, T2: Fn> Fn for SubOp<T1, T2> {
    fn eval(self, input: f32) -> (f32, f32) {
        let (u, du) = self.lhs.eval(input);
        let (v, dv) = self.rhs.eval(input);

        (u - v, du - dv)
    }
}

/// Negating an expression
#[derive(Clone, Copy)]
pub struct NegOp<T: Fn> {
    expr: T,
}

impl<T: Fn> Fn for NegOp<T> {
    fn eval(self, input: f32) -> (f32, f32) {
        let (y, dy) = self.expr.eval(input);

        (-y, -dy)
    }
}

/// Multiplying 2 expressions
#[derive(Clone, Copy)]
pub struct MulOp<T1: Fn, T2: Fn> {
    lhs: T1,
    rhs: T2,
}

impl<T1: Fn, T2: Fn> Fn for MulOp<T1, T2> {
    // f(x) = uv, f'(x) = uv' + vu'
    fn eval(self, input: f32) -> (f32, f32) {
        let (u, du) = self.lhs.eval(input);
        let (v, dv) = self.rhs.eval(input);

        (u * v, u * dv + v * du)
    }
}

/// Dividing 2 expressions
#[derive(Clone, Copy)]
pub struct DivOp<T1: Fn, T2: Fn> {
    lhs: T1,
    rhs: T2,
}

impl<T1: Fn, T2: Fn> Fn for DivOp<T1, T2> {
    // f(x) = u/v, f'(x) = (u'v - v'u) / v^2
    fn eval(self, input: f32) -> (f32, f32) {
        let (u, du) = self.lhs.eval(input);
        let (v, dv) = self.rhs.eval(input);

        (u / v, (du * v - dv * u) / (v * v))
    }
}

// Power
#[derive(Clone, Copy)]
pub struct PowOp<T: Fn> {
    expr: T,
    order: f32,
}

impl<T: Fn> Fn for PowOp<T> {
    // f(x) = u^n, f'(x) = u'nu^(n - 1)
    fn eval(self, input: f32) -> (f32, f32) {
        let (y, dy) = self.expr.eval(input);

        (
            y.powf(self.order),
            dy * self.order * y.powf(self.order - 1.0),
        )
    }
}

// Exponentation
#[derive(Clone, Copy)]
pub struct ExpOp<T: Fn> {
    expr: T,
}

impl<T: Fn> Fn for ExpOp<T> {
    // f(x) = e^u, f'(x) = u'e^u
    fn eval(self, input: f32) -> (f32, f32) {
        let (y, dy) = self.expr.eval(input);
        let exp = y.exp();

        (exp, dy * exp)
    }
}

// Trigonometry
#[derive(Clone, Copy)]
pub struct SinOp<T: Fn> {
    expr: T,
}

impl<T: Fn> Fn for SinOp<T> {
    // f(x) = sin(u), f'(x) = u'cos(u)
    fn eval(self, input: f32) -> (f32, f32) {
        let (y, dy) = self.expr.eval(input);
        let (sin, cos) = y.sin_cos();

        (sin, dy * cos)
    }
}

#[derive(Clone, Copy)]
pub struct CosOp<T: Fn> {
    expr: T,
}

impl<T: Fn> Fn for CosOp<T> {
    // f(x) = sin(u), f'(x) = u'cos(u)
    fn eval(self, input: f32) -> (f32, f32) {
        let (y, dy) = self.expr.eval(input);
        let (sin, cos) = y.sin_cos();

        (cos, dy * -sin)
    }
}

// Inverse trigonometry

#[derive(Clone, Copy)]
pub struct AtanOp<T: Fn> {
    expr: T,
}

impl<T: Fn> Fn for AtanOp<T> {
    // f(x) = atan(u), f'(x) = u'/(1 + u^2)
    fn eval(self, input: f32) -> (f32, f32) {
        let (y, dy) = self.expr.eval(input);

        (y.atan(), dy / (1.0 + y * y))
    }
}

// Logarithm
#[derive(Clone, Copy)]
pub struct LnOp<T: Fn> {
    expr: T,
}

impl<T: Fn> Fn for LnOp<T> {
    // f(x) = ln(u), f'(x) = u'/u
    fn eval(self, input: f32) -> (f32, f32) {
        let (y, dy) = self.expr.eval(input);

        (y.ln(), dy / y)
    }
}

// Composition of 2 functions
pub struct ComposeOp<T1: Fn, T2: Fn> {
    lhs: T1,
    rhs: T2,
}

impl<T1: Fn, T2: Fn> Fn for ComposeOp<T1, T2> {
    // f(x) = g(h(x)), f'(x) = h'(x)g'(h(x))
    fn eval(self, input: f32) -> (f32, f32) {
        let (h, dh) = self.rhs.eval(input);
        let (g, dg) = self.lhs.eval(h);

        (g, dh * dg)
    }
}

/// The generic expression struct
#[derive(Clone, Copy)]
pub struct Expr<T> {
    expr: T,
}

impl<T: Fn> Fn for Expr<T> {
    fn eval(self, input: f32) -> (f32, f32) {
        self.expr.eval(input)
    }
}

// Addition operator overloading

impl<T1: Fn, T2: Fn> Add<Expr<T2>> for Expr<T1> {
    type Output = Expr<AddOp<T1, T2>>;

    fn add(self, rhs: Expr<T2>) -> Self::Output {
        Self::Output {
            expr: AddOp {
                lhs: self.expr,
                rhs: rhs.expr,
            },
        }
    }
}

impl<T: Fn> Add<f32> for Expr<T> {
    type Output = Expr<AddOp<T, Const>>;

    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            expr: AddOp {
                lhs: self.expr,
                rhs: Const { value: rhs },
            },
        }
    }
}

impl<T: Fn> Add<Expr<T>> for f32 {
    type Output = Expr<AddOp<Const, T>>;

    fn add(self, rhs: Expr<T>) -> Self::Output {
        Self::Output {
            expr: AddOp {
                lhs: Const { value: self },
                rhs: rhs.expr,
            },
        }
    }
}

// Multiplication operator overloading

impl<T1: Fn, T2: Fn> Mul<Expr<T2>> for Expr<T1> {
    type Output = Expr<MulOp<T1, T2>>;

    fn mul(self, rhs: Expr<T2>) -> Self::Output {
        Self::Output {
            expr: MulOp {
                lhs: self.expr,
                rhs: rhs.expr,
            },
        }
    }
}

impl<T: Fn> Mul<f32> for Expr<T> {
    type Output = Expr<MulOp<T, Const>>;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            expr: MulOp {
                lhs: self.expr,
                rhs: Const { value: rhs },
            },
        }
    }
}

impl<T: Fn> Mul<Expr<T>> for f32 {
    type Output = Expr<MulOp<Const, T>>;

    fn mul(self, rhs: Expr<T>) -> Self::Output {
        Self::Output {
            expr: MulOp {
                lhs: Const { value: self },
                rhs: rhs.expr,
            },
        }
    }
}

// Substraction operator overloading

impl<T1: Fn, T2: Fn> Sub<Expr<T2>> for Expr<T1> {
    type Output = Expr<SubOp<T1, T2>>;

    fn sub(self, rhs: Expr<T2>) -> Self::Output {
        Self::Output {
            expr: SubOp {
                lhs: self.expr,
                rhs: rhs.expr,
            },
        }
    }
}

impl<T: Fn> Sub<f32> for Expr<T> {
    type Output = Expr<SubOp<T, Const>>;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::Output {
            expr: SubOp {
                lhs: self.expr,
                rhs: Const { value: rhs },
            },
        }
    }
}

impl<T: Fn> Sub<Expr<T>> for f32 {
    type Output = Expr<SubOp<Const, T>>;

    fn sub(self, rhs: Expr<T>) -> Self::Output {
        Self::Output {
            expr: SubOp {
                lhs: Const { value: self },
                rhs: rhs.expr,
            },
        }
    }
}

// Negation operator overloading

impl<T: Fn> Neg for Expr<T> {
    type Output = Expr<NegOp<T>>;

    fn neg(self) -> Self::Output {
        Self::Output {
            expr: NegOp { expr: self.expr },
        }
    }
}

// Division operator overloading

impl<T1: Fn, T2: Fn> Div<Expr<T2>> for Expr<T1> {
    type Output = Expr<DivOp<T1, T2>>;

    fn div(self, rhs: Expr<T2>) -> Self::Output {
        Self::Output {
            expr: DivOp {
                lhs: self.expr,
                rhs: rhs.expr,
            },
        }
    }
}

impl<T: Fn> Div<f32> for Expr<T> {
    type Output = Expr<DivOp<T, Const>>;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            expr: DivOp {
                lhs: self.expr,
                rhs: Const { value: rhs },
            },
        }
    }
}

impl<T: Fn> Div<Expr<T>> for f32 {
    type Output = Expr<DivOp<Const, T>>;

    fn div(self, rhs: Expr<T>) -> Self::Output {
        Self::Output {
            expr: DivOp {
                lhs: Const { value: self },
                rhs: rhs.expr,
            },
        }
    }
}

impl<T: Fn> Expr<T> {
    pub fn pow(self, order: f32) -> Expr<PowOp<T>> {
        Expr {
            expr: PowOp {
                expr: self.expr,
                order,
            },
        }
    }

    pub fn sqrt(self) -> Expr<PowOp<T>> {
        Expr {
            expr: PowOp {
                expr: self.expr,
                order: 0.5,
            },
        }
    }

    pub fn exp(self) -> Expr<ExpOp<T>> {
        Expr {
            expr: ExpOp { expr: self.expr },
        }
    }

    pub fn sin(self) -> Expr<SinOp<T>> {
        Expr {
            expr: SinOp { expr: self.expr },
        }
    }

    pub fn cos(self) -> Expr<CosOp<T>> {
        Expr {
            expr: CosOp { expr: self.expr },
        }
    }

    pub fn atan(self) -> Expr<AtanOp<T>> {
        Expr {
            expr: AtanOp { expr: self.expr },
        }
    }

    pub fn ln(self) -> Expr<LnOp<T>> {
        Expr {
            expr: LnOp { expr: self.expr },
        }
    }

    pub fn compose<T1: Fn>(self, other: Expr<T1>) -> Expr<ComposeOp<T, T1>> {
        Expr {
            expr: ComposeOp {
                lhs: self.expr,
                rhs: other.expr,
            },
        }
    }
}

/// The identity function f(x) = x
pub const X: Expr<Var> = Expr { expr: Var {} };
