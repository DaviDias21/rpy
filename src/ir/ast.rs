pub type Name = String;

use std::collections::HashMap;

#[derive(Clone)]
pub struct Frame<A> {
    pub parent_function: Option<Function>,
    pub parent_key: Option<(Name, i32)>,
    pub variables: HashMap<Name, A>,
}

impl<A> Frame<A> {
    pub fn new(func: Option<Function>, key: Option<(Name, i32)>) -> Frame<A> {
        let variables: HashMap<Name, A> = HashMap::new();

        return Frame {
            parent_function: func,
            parent_key: key,
            variables,
        };
    }
}

#[derive(Clone)]
pub struct Environment<A> {
    pub scope: Function,
    pub recursion: i32,
    pub stack: HashMap<(Name, i32), Frame<A>>,
}

impl<A> Environment<A> {
    pub fn new() -> Environment<A> {
        let frame: Frame<A> = Frame::new(None, None);
        let scope = Function::new();

        return Environment {
            scope,
            recursion: 0,
            stack: HashMap::from([(("__main__".to_string(), 0), frame)]),
        };
    }

    pub fn scope_key(&self) -> (Name, i32) {
        return (self.scope_name(), self.recursion);
    }

    pub fn scope_name(&self) -> Name {
        return self.scope.name.clone();
    }

    pub fn scope_return(&self) -> Option<&A> {
        return self.search_frame(self.scope_name());
    }

    pub fn get_frame(&self, key: (Name, i32)) -> &Frame<A> {
        return self.stack.get(&key).unwrap();
    }

    pub fn search_frame(&self, name: Name) -> Option<&A> {
        return self
            .stack
            .get(&self.scope_key())
            .unwrap()
            .variables
            .get(&name);
    }

    pub fn insert_frame(&mut self, func: Function) -> () {
        let new_frame: Frame<A> = Frame::new(Some(self.scope.clone()), Some(self.scope_key()));

        self.stack
            .insert((func.name.clone(), self.scope_key().1 + 1), new_frame);
        self.scope = func;
        self.recursion += 1;
    }

    pub fn remove_frame(&mut self) -> () {
        let recursion = self.scope_key().1 - 1;
        self.scope = self
            .stack
            .remove(&self.scope_key())
            .unwrap()
            .parent_function
            .unwrap();
        self.recursion = recursion;
    }

    pub fn insert_variable(&mut self, name: Name, kind: A) -> () {
        if let Some(frame) = self.stack.get_mut(&self.scope_key()) {
            frame.variables.insert(name, kind);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: Name,
    pub kind: Option<Type>,
    pub params: Option<Vec<(Name, Type)>>,
    pub body: Option<Box<Statement>>,
}

impl Function {
    pub fn new() -> Function {
        return Function {
            name: "__main__".to_string(),
            kind: None,
            params: None,
            body: None,
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    TInteger,
    TBool,
    TReal,
    TString,
    TFunction(Box<Option<Type>>, Vec<Type>),
    TList(Box<Type>),
    TTuple(Vec<Type>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /* constants */
    CTrue,
    CFalse,
    CInt(i32),
    CReal(f64),
    CString(String),

    /* variable reference */
    Var(Name),

    /* function call */
    FuncCall(Name, Vec<Expression>),

    /* arithmetic expressions over numbers */
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),

    /* boolean expressions over booleans */
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>),

    /* relational expressions over numbers */
    EQ(Box<Expression>, Box<Expression>),
    GT(Box<Expression>, Box<Expression>),
    LT(Box<Expression>, Box<Expression>),
    GTE(Box<Expression>, Box<Expression>),
    LTE(Box<Expression>, Box<Expression>),

    ReadFile(Box<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    VarDeclaration(Name),
    ValDeclaration(Name),
    Assignment(Name, Box<Expression>, Option<Type>),
    IfThenElse(Box<Expression>, Box<Statement>, Option<Box<Statement>>),
    While(Box<Expression>, Box<Statement>),
    Sequence(Box<Statement>, Box<Statement>),
    FuncDef(Function),
    Return(Box<Expression>),
    WriteToFile(Box<Expression>, Box<Expression>),
    Print(Expression),
    //ReadFile(Box<Expression>, Name), 
}
