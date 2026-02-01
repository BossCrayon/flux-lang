use crate::ast::{Statement, Expression, Program, BlockStatement};
use crate::object::Object;
use crate::environment::Environment;

pub fn eval_program(program: &Program, env: &mut Environment) -> Object {
    let mut result = Object::Null;
    for statement in &program.statements {
        result = eval_statement(statement, env);
        if let Object::Error(_) = result { return result; }
        if let Statement::Return { .. } = statement { return result; }
    }
    result
}

fn eval_statement(stmt: &Statement, env: &mut Environment) -> Object {
    match stmt {
        Statement::ExpressionStatement { expression, .. } => eval_expression(expression, env),
        Statement::Return { value, .. } => {
            let val = eval_expression(value, env);
            if let Object::Error(_) = val { return val; }
            Object::Return(Box::new(val))
        },
        Statement::Let { name, value, .. } => {
            let val = eval_expression(value, env);
            if let Object::Error(_) = val { return val; }
            env.set(name.clone(), val)
        },
    }
}

fn eval_expression(expr: &Expression, env: &mut Environment) -> Object {
    match expr {
        Expression::IntegerLiteral(val) => Object::Integer(*val),
        Expression::Boolean(val) => Object::Boolean(*val),
        // NEW: Evaluate String
        Expression::StringLiteral(val) => Object::String(val.clone()),
        
        Expression::Identifier(name) => {
            match env.get(name) {
                Some(val) => val,
                None => Object::Error(format!("Variable '{}' not found", name)),
            }
        },
        Expression::Prefix { .. } => Object::Error("Prefix math not implemented".to_string()),
        Expression::Infix { left, operator, right } => {
            let l = eval_expression(left, env);
            if let Object::Error(_) = l { return l; }
            let r = eval_expression(right, env);
            if let Object::Error(_) = r { return r; }
            eval_infix(operator, l, r)
        },
        Expression::If { condition, consequence, alternative } => {
            let cond = eval_expression(condition, env);
            if is_truthy(cond) {
                eval_block(consequence, env)
            } else if let Some(alt) = alternative {
                eval_block(alt, env)
            } else {
                Object::Null
            }
        },
        Expression::While { condition, body } => {
            let mut result = Object::Null;
            loop {
                let cond = eval_expression(condition, env);
                if !is_truthy(cond) { break; }
                result = eval_block(body, env);
                if let Object::Return(_) = result { return result; }
                if let Object::Error(_) = result { return result; }
            }
            result
        },
        Expression::FunctionLiteral { parameters, body } => {
            Object::Function {
                parameters: parameters.clone(),
                body: body.clone(),
                env: env.clone(),
            }
        },
        Expression::CallExpression { function, arguments } => {
            let func = eval_expression(function, env);
            if let Object::Error(_) = func { return func; }
            let mut args = vec![];
            for arg in arguments {
                let evaluated = eval_expression(arg, env);
                if let Object::Error(_) = evaluated { return evaluated; }
                args.push(evaluated);
            }
            apply_function(func, args)
        },
        Expression::Material { name } => Object::Material { name: name.clone() },
        Expression::ArrayLiteral { elements } => {
            let mut objs = vec![];
            for el in elements {
                let val = eval_expression(el, env);
                if let Object::Error(_) = val { return val; }
                objs.push(val);
            }
            Object::Array(objs)
        },
        Expression::IndexExpression { left, index } => {
            let left_eval = eval_expression(left, env);
            if let Object::Error(_) = left_eval { return left_eval; }
            let index_eval = eval_expression(index, env);
            if let Object::Error(_) = index_eval { return index_eval; }
            eval_index_expression(left_eval, index_eval)
        },
    }
}

fn eval_index_expression(left: Object, index: Object) -> Object {
    match (left, index) {
        (Object::Array(elements), Object::Integer(idx)) => {
            let i = idx as usize;
            if idx < 0 || i >= elements.len() {
                return Object::Null; 
            }
            elements[i].clone()
        },
        _ => Object::Error("index operator not supported".to_string()),
    }
}

fn apply_function(func: Object, args: Vec<Object>) -> Object {
    match func {
        Object::Function { parameters, body, env } => {
            let mut extended_env = env.clone();
            for (i, param) in parameters.iter().enumerate() {
                extended_env.set(param.clone(), args[i].clone());
            }
            let evaluated = eval_block(&body, &mut extended_env);
            if let Object::Return(val) = evaluated {
                return *val;
            }
            evaluated
        },
        Object::Builtin(func) => func(args),
        _ => Object::Error("not a function".to_string()),
    }
}

fn eval_block(block: &BlockStatement, env: &mut Environment) -> Object { let mut r = Object::Null; for s in &block.statements { r = eval_statement(s, env); if let Object::Return(_) = r { return r; } if let Object::Error(_) = r { return r; } } r }

fn eval_infix(op: &str, left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => match op {
            "+" => Object::Integer(l + r),
            "-" => Object::Integer(l - r),
            "*" => Object::Integer(l * r),
            "/" => Object::Integer(l / r),
            "<" => Object::Boolean(l < r),
            ">" => Object::Boolean(l > r),
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            _ => Object::Error(format!("Unknown op: {}", op)),
        },
        (Object::Boolean(l), Object::Boolean(r)) => match op {
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            _ => Object::Error("Unknown op".to_string()),
        },
        
        // NEW: String Concatenation
        (Object::String(l), Object::String(r)) => match op {
            "+" => Object::String(format!("{}{}", l, r)),
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            _ => Object::Error("Unknown string op".to_string()),
        },
        
        _ => Object::Error("Type mismatch".to_string()),
    }
}

fn is_truthy(obj: Object) -> bool { match obj { Object::Null => false, Object::Boolean(true) => true, Object::Boolean(false) => false, Object::Integer(0) => false, _ => true } }