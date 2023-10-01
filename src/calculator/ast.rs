
#[derive(std::fmt::Debug)]
pub enum Expression {
    Number(f32),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Pow(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn num(num: f32) -> Box<Self> {
        Box::new(Expression::Number(num))
    }
    pub fn add(e1: Box<Self>, e2: Box<Self>) -> Box<Self> {
        Box::new(Expression::Add(e1, e2))
    }
    pub fn sub(e1: Box<Self>, e2: Box<Self>) -> Box<Self> {
        Box::new(Expression::Sub(e1, e2))
    }
    pub fn mul(e1: Box<Self>, e2: Box<Self>) -> Box<Self> {
        Box::new(Expression::Mul(e1, e2))
    }
    pub fn div(e1: Box<Self>, e2: Box<Self>) -> Box<Self> {
        Box::new(Expression::Div(e1, e2))
    }
    pub fn pow(e1: Box<Self>, e2: Box<Self>) -> Box<Self> {
        Box::new(Expression::Pow(e1, e2))
    }
}

impl Expression {
    pub fn evaluate(&self) -> f32 {
        match self {
            Expression::Number(num) => *num,
            Expression::Add(e1, e2) => e1.evaluate() + e2.evaluate(),
            Expression::Sub(e1, e2) => e1.evaluate() - e2.evaluate(),
            Expression::Mul(e1, e2) => e1.evaluate() * e2.evaluate(),
            Expression::Div(e1, e2) => e1.evaluate() / e2.evaluate(),
            Expression::Pow(e1, e2) => e1.evaluate().powf(e2.evaluate()),
        }
    }
}

