use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Differentiable {
    fn eval(self, input: f32) -> (f32, f32);
}

// TODO: Multiple variables using vector
#[derive(Clone, Copy)]
pub struct Var;

impl Differentiable for Var {
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

impl Differentiable for Const {
    // f(x) = k, f'(x) = 0
    fn eval(self, _input: f32) -> (f32, f32) {
        (self.value, 0.0)
    }
}

/// Adding 2 expressions
#[derive(Clone, Copy)]
pub struct AddOp<T1: Differentiable + Copy, T2: Differentiable + Copy> {
    lhs: T1,
    rhs: T2,
}

impl<T1: Differentiable + Copy, T2: Differentiable + Copy> Differentiable for AddOp<T1, T2> {
    // f(x) = u + v, f'(x) = u' + v'
    fn eval(self, input: f32) -> (f32, f32) {
        let (u, du) = self.lhs.eval(input);
        let (v, dv) = self.rhs.eval(input);

        (u + v, du + dv)
    }
}

/// Substracting 2 expressions
#[derive(Clone, Copy)]
pub struct SubOp<T1: Differentiable + Copy, T2: Differentiable + Copy> {
    lhs: T1,
    rhs: T2,
}

impl<T1: Differentiable + Copy, T2: Differentiable + Copy> Differentiable for SubOp<T1, T2> {
    fn eval(self, input: f32) -> (f32, f32) {
        let (u, du) = self.lhs.eval(input);
        let (v, dv) = self.rhs.eval(input);

        (u - v, du - dv)
    }
}

/// Negating an expression
#[derive(Clone, Copy)]
pub struct NegOp<T: Differentiable + Copy> {
    expr: T,
}

impl<T: Differentiable + Copy> Differentiable for NegOp<T> {
    fn eval(self, input: f32) -> (f32, f32) {
        let (y, dy) = self.expr.eval(input);

        (-y, -dy)
    }
}

/// Multiplying 2 expressions
#[derive(Clone, Copy)]
pub struct MulOp<T1: Differentiable + Copy, T2: Differentiable + Copy> {
    lhs: T1,
    rhs: T2,
}

impl<T1: Differentiable + Copy, T2: Differentiable + Copy> Differentiable for MulOp<T1, T2> {
    // f(x) = uv, f'(x) = uv' + vu'
    fn eval(self, input: f32) -> (f32, f32) {
        let (u, du) = self.lhs.eval(input);
        let (v, dv) = self.rhs.eval(input);

        (u * v, u * dv + v * du)
    }
}

/// Dividing 2 expressions
#[derive(Clone, Copy)]
pub struct DivOp<T1: Differentiable + Copy, T2: Differentiable + Copy> {
    lhs: T1,
    rhs: T2,
}

impl<T1: Differentiable + Copy, T2: Differentiable + Copy> Differentiable for DivOp<T1, T2> {
    // f(x) = u/v, f'(x) = (u'v - v'u) / v^2
    fn eval(self, input: f32) -> (f32, f32) {
        let (u, du) = self.lhs.eval(input);
        let (v, dv) = self.rhs.eval(input);

        (u / v, (du * v - dv * u) / (v * v))
    }
}

// Power
#[derive(Clone, Copy)]
pub struct PowOp<T: Differentiable + Copy> {
    expr: T,
    order: f32
}

impl<T: Differentiable + Copy> Differentiable for PowOp<T> {
    // f(x) = u^n, f'(x) = u'nu^(n - 1)
    fn eval(self, input: f32) -> (f32, f32) {
        let (y, dy) = self.expr.eval(input);
        
        (y.powf(self.order), dy * self.order * y.powf(self.order - 1.0))
    }
}

// Exponentation
#[derive(Clone, Copy)]
pub struct ExpOp<T: Differentiable + Copy> {
    expr: T,
}

impl<T: Differentiable + Copy> Differentiable for ExpOp<T> {
    // f(x) = e^u, f'(x) = u'e^u
    fn eval(self, input: f32) -> (f32, f32) {
        let (y, dy) = self.expr.eval(input);
        let exp = y.exp();

        (exp, dy * exp)
    }
}

// Trigonometry
#[derive(Clone, Copy)]
pub struct SinOp<T: Differentiable + Copy> {
    expr: T,
}

impl<T: Differentiable + Copy> Differentiable for SinOp<T> {
    // f(x) = sin(u), f'(x) = u'cos(u)
    fn eval(self, input: f32) -> (f32, f32) {
        let (y, dy) = self.expr.eval(input);
        let (sin, cos) = y.sin_cos();

        (sin, dy * cos)
    }
}

#[derive(Clone, Copy)]
pub struct CosOp<T: Differentiable + Copy> {
    expr: T,
}

impl<T: Differentiable + Copy> Differentiable for CosOp<T> {
    // f(x) = sin(u), f'(x) = u'cos(u)
    fn eval(self, input: f32) -> (f32, f32) {
        let (y, dy) = self.expr.eval(input);
        let (sin, cos) = y.sin_cos();

        (cos, dy * -sin)
    }
}

// Logarithm
#[derive(Clone, Copy)]
pub struct LnOp<T: Differentiable + Copy> {
    expr: T,
}

impl<T: Differentiable + Copy> Differentiable for LnOp<T> {
    // f(x) = ln(u), f'(x) = u'/u
    fn eval(self, input: f32) -> (f32, f32) {
        let (y, dy) = self.expr.eval(input);

        (y.ln(), dy / y)
    }
}

/// The generic expression struct
#[derive(Clone, Copy)]
pub struct Expression<T> {
    expr: T,
}

impl<T: Differentiable + Copy> Differentiable for Expression<T> {
    fn eval(self, input: f32) -> (f32, f32) {
        self.expr.eval(input)
    }
}

// Addition operator overloading

impl<T1: Differentiable + Copy, T2: Differentiable + Copy> Add<Expression<T2>> for Expression<T1> {
    type Output = Expression<AddOp<T1, T2>>;

    fn add(self, rhs: Expression<T2>) -> Self::Output {
        Self::Output {
            expr: AddOp {
                lhs: self.expr,
                rhs: rhs.expr,
            },
        }
    }
}

impl<T: Differentiable + Copy> Add<f32> for Expression<T> {
    type Output = Expression<AddOp<T, Const>>;

    fn add(self, rhs: f32) -> Self::Output {
        Self::Output {
            expr: AddOp {
                lhs: self.expr,
                rhs: Const { value: rhs },
            },
        }
    }
}

impl<T: Differentiable + Copy> Add<Expression<T>> for f32 {
    type Output = Expression<AddOp<Const, T>>;

    fn add(self, rhs: Expression<T>) -> Self::Output {
        Self::Output {
            expr: AddOp {
                lhs: Const { value: self },
                rhs: rhs.expr,
            },
        }
    }
}

// Multiplication operator overloading

impl<T1: Differentiable + Copy, T2: Differentiable + Copy> Mul<Expression<T2>> for Expression<T1> {
    type Output = Expression<MulOp<T1, T2>>;

    fn mul(self, rhs: Expression<T2>) -> Self::Output {
        Self::Output {
            expr: MulOp {
                lhs: self.expr,
                rhs: rhs.expr,
            },
        }
    }
}

impl<T: Differentiable + Copy> Mul<f32> for Expression<T> {
    type Output = Expression<MulOp<T, Const>>;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            expr: MulOp {
                lhs: self.expr,
                rhs: Const { value: rhs },
            },
        }
    }
}

impl<T: Differentiable + Copy> Mul<Expression<T>> for f32 {
    type Output = Expression<MulOp<Const, T>>;

    fn mul(self, rhs: Expression<T>) -> Self::Output {
        Self::Output {
            expr: MulOp {
                lhs: Const { value: self },
                rhs: rhs.expr,
            },
        }
    }
}

// Substraction operator overloading

impl<T1: Differentiable + Copy, T2: Differentiable + Copy> Sub<Expression<T2>> for Expression<T1> {
    type Output = Expression<SubOp<T1, T2>>;

    fn sub(self, rhs: Expression<T2>) -> Self::Output {
        Self::Output {
            expr: SubOp {
                lhs: self.expr,
                rhs: rhs.expr,
            },
        }
    }
}

impl<T: Differentiable + Copy> Sub<f32> for Expression<T> {
    type Output = Expression<SubOp<T, Const>>;

    fn sub(self, rhs: f32) -> Self::Output {
        Self::Output {
            expr: SubOp {
                lhs: self.expr,
                rhs: Const { value: rhs },
            },
        }
    }
}

impl<T: Differentiable + Copy> Sub<Expression<T>> for f32 {
    type Output = Expression<SubOp<Const, T>>;

    fn sub(self, rhs: Expression<T>) -> Self::Output {
        Self::Output {
            expr: SubOp {
                lhs: Const { value: self },
                rhs: rhs.expr,
            },
        }
    }
}

// Negation operator overloading

impl<T: Differentiable + Copy> Neg for Expression<T> {
    type Output = Expression<NegOp<T>>;

    fn neg(self) -> Self::Output {
        Self::Output {
            expr: NegOp { expr: self.expr },
        }
    }
}

// Division operator overloading

impl<T1: Differentiable + Copy, T2: Differentiable + Copy> Div<Expression<T2>> for Expression<T1> {
    type Output = Expression<DivOp<T1, T2>>;

    fn div(self, rhs: Expression<T2>) -> Self::Output {
        Self::Output {
            expr: DivOp {
                lhs: self.expr,
                rhs: rhs.expr,
            },
        }
    }
}

impl<T: Differentiable + Copy> Div<f32> for Expression<T> {
    type Output = Expression<DivOp<T, Const>>;

    fn div(self, rhs: f32) -> Self::Output {
        Self::Output {
            expr: DivOp {
                lhs: self.expr,
                rhs: Const { value: rhs },
            },
        }
    }
}

impl<T: Differentiable + Copy> Div<Expression<T>> for f32 {
    type Output = Expression<DivOp<Const, T>>;

    fn div(self, rhs: Expression<T>) -> Self::Output {
        Self::Output {
            expr: DivOp {
                lhs: Const { value: self },
                rhs: rhs.expr,
            },
        }
    }
}

impl<T: Differentiable + Copy> Expression<T> {
    pub fn pow(self, order: f32) -> Expression<PowOp<T>> {
        Expression {
            expr: PowOp {
                expr: self.expr,
                order
            }
        }
    }

    pub fn sqrt(self) -> Expression<PowOp<T>> {
        Expression {
            expr: PowOp {
                expr: self.expr,
                order: 0.5
            }
        }
    }

    pub fn exp(self) -> Expression<ExpOp<T>> {
        Expression {
            expr: ExpOp { expr: self.expr },
        }
    }

    pub fn sin(self) -> Expression<SinOp<T>> {
        Expression {
            expr: SinOp { expr: self.expr },
        }
    }

    pub fn cos(self) -> Expression<CosOp<T>> {
        Expression {
            expr: CosOp { expr: self.expr },
        }
    }

    pub fn ln(self) -> Expression<LnOp<T>> {
        Expression {
            expr: LnOp { expr: self.expr },
        }
    }
}

impl Var {
    pub fn get() -> Expression<Var> {
        Expression { expr: Var {} }
    }
}
