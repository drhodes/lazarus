





fn eval(exp: Expr, env: Env) {
    if exp.is_self_evaluating() {
        exp
    } else if exp.is_variable() {
        env.lookup_variable(exp) 
    }
}
