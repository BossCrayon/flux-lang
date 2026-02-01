use crate::ast::{Statement, Expression, BlockStatement};
use crate::object::{Object, HashKey};
use crate::environment::Environment;

pub fn eval_program(program: &[Statement], env: &mut Environment) -> Object {
    let mut result = Object::Null;
    for stmt in program {
        result = eval_statement(stmt, env);
        if let Object::Return(val) = result { return *val; }
        if let Object::Error(_) = result { return result; }
    }
    result
}

fn eval_statement(stmt: &Statement, env: &mut Environment) -> Object {
    match stmt {
        Statement::Expression(exp) => eval(exp, env),
        Statement::Return(val) => {
            let value = eval(val, env);
            if is_error(&value) { return value; }
            Object::Return(Box::new(value))
        },
        Statement::Let { name, value } => {
            let val = eval(value, env);
            if is_error(&val) { return val; }
            env.set(name.clone(), val);
            Object::Null
        },
        _ => Object::Null,
    }
}

// MAKE THIS PUBLIC or accessible within the file
fn eval(node: &Expression, env: &mut Environment) -> Object {
    match node {
        Expression::IntegerLiteral(i) => Object::Integer(*i),
        Expression::Boolean(b) => Object::Boolean(*b),
        Expression::StringLiteral(s) => Object::String(s.clone()),
        Expression::Prefix { operator, right } => {
            let right_val = eval(right, env);
            if is_error(&right_val) { return right_val; }
            eval_prefix(operator, right_val)
        },
        Expression::Infix { left, operator, right } => {
            let left_val = eval(left, env);
            if is_error(&left_val) { return left_val; }
            let right_val = eval(right, env);
            if is_error(&right_val) { return right_val; }
            eval_infix(operator, left_val, right_val)
        },
        Expression::Identifier(name) => match env.get(name) {
            Some(obj) => obj,
            None => Object::Error(format!("Variable '{}' not found", name)),
        },
        Expression::If { condition, consequence, alternative } => {
            let cond = eval(condition, env);
            if is_error(&cond) { return cond; }
            if is_truthy(&cond) {
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
                let cond = eval(condition, env);
                if is_error(&cond) { return cond; }
                if !is_truthy(&cond) { break; }
                result = eval_block(body, env);
                if let Object::Return(_) = result { return result; }
            }
            result
        },
        Expression::FunctionLiteral { parameters, body } => {
            Object::Function { parameters: parameters.clone(), body: body.clone(), env: env.clone() }
        },
        // CORRECT: Matches Call (not CallExpression)
        Expression::Call { function, arguments } => {
            let func = eval(function, env);
            if is_error(&func) { return func; }
            let args = eval_expressions(arguments, env);
            if args.len() == 1 && is_error(&args[0]) { return args[0].clone(); }
            apply_function(func, args)
        },
        // CORRECT: Matches Tuple Variant
        Expression::ArrayLiteral(elements) => {
            let elems = eval_expressions(elements, env);
            if elems.len() == 1 && is_error(&elems[0]) { return elems[0].clone(); }
            Object::Array(elems)
        },
        Expression::IndexExpression { left, index } => {
            let l = eval(left, env);
            if is_error(&l) { return l; }
            let i = eval(index, env);
            if is_error(&i) { return i; }
            eval_index(l, i)
        },
        // NEW: Hash Map
        Expression::HashLiteral(node) => eval_hash_literal(node, env),
    }
}

fn eval_hash_literal(node: &crate::ast::HashLiteral, env: &mut Environment) -> Object {
    let mut pairs = std::collections::HashMap::new();
    for (key_node, value_node) in &node.pairs {
        let key = eval(key_node, env);
        if is_error(&key) { return key; }
        let hash_key = match crate::object::get_hash_key(&key) {
            Some(k) => k,
            None => return Object::Error(format!("Unusable as hash key: {}", key)),
        };
        let value = eval(value_node, env);
        if is_error(&value) { return value; }
        pairs.insert(hash_key, value);
    }
    Object::Hash(pairs)
}

fn eval_expressions(exps: &[Expression], env: &mut Environment) -> Vec<Object> {
    let mut result = vec![];
    for e in exps {
        let val = eval(e, env);
        if is_error(&val) { return vec![val]; }
        result.push(val);
    }
    result
}

fn eval_block(block: &BlockStatement, env: &mut Environment) -> Object {
    let mut result = Object::Null;
    for stmt in &block.statements {
        result = eval_statement(stmt, env);
        if let Object::Return(_) = result { return result; }
        if let Object::Error(_) = result { return result; }
    }
    result
}

fn eval_index(left: Object, index: Object) -> Object {
    match (left, index) {
        (Object::Array(arr), Object::Integer(idx)) => {
            if idx < 0 || idx >= arr.len() as i64 { return Object::Null; }
            arr[idx as usize].clone()
        },
        (Object::Hash(pairs), index_obj) => {
            match crate::object::get_hash_key(&index_obj) {
                Some(key) => match pairs.get(&key) {
                    Some(obj) => obj.clone(),
                    None => Object::Null,
                },
                None => Object::Error(format!("Unusable as hash key: {}", index_obj)),
            }
        },
        _ => Object::Error("Index operator not supported".to_string()),
    }
}

fn apply_function(func: Object, args: Vec<Object>) -> Object {
    match func {
        Object::Function { parameters, body, env } => {
            let mut enclosed = crate::environment::Environment::new_enclosed(env);
            for (param, arg) in parameters.iter().zip(args.iter()) {
                enclosed.set(param.clone(), arg.clone());
            }
            let result = eval_block(&body, &mut enclosed);
            if let Object::Return(val) = result { *val } else { result }
        },
        Object::Builtin(builtin_fn) => builtin_fn(args),
        _ => Object::Error("Not a function".to_string()),
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(true) => true,
        Object::Boolean(false) => false,
        _ => true,
    }
}

fn is_error(obj: &Object) -> bool {
    match obj {
        Object::Error(_) => true,
        _ => false,
    }
}

fn eval_prefix(op: &str, right: Object) -> Object {
    match op {
        "!" => match right {
            Object::Boolean(true) => Object::Boolean(false),
            Object::Boolean(false) => Object::Boolean(true),
            Object::Null => Object::Boolean(true),
            _ => Object::Boolean(false),
        },
        "-" => match right {
            Object::Integer(val) => Object::Integer(-val),
            _ => Object::Error("Unknown operator: -".to_string()),
        },
        _ => Object::Error(format!("Unknown operator: {}", op)),
    }
}

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
        (Object::String(l), Object::String(r)) => match op {
            "+" => Object::String(format!("{}{}", l, r)),
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            _ => Object::Error("Unknown string op".to_string()),
        },
        (Object::String(l), Object::Integer(r)) => match op {
             "+" => Object::String(format!("{}{}", l, r)),
             _ => Object::Error("Type mismatch".to_string()),
        },
        (Object::Integer(l), Object::String(r)) => match op {
             "+" => Object::String(format!("{}{}", l, r)),
             _ => Object::Error("Type mismatch".to_string()),
        },
        (Object::Boolean(l), Object::Boolean(r)) => match op {
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            _ => Object::Error("Unknown op".to_string()),
        },
        _ => Object::Error("Type mismatch".to_string()),
    }
}