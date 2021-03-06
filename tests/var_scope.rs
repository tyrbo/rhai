use rhai::{Engine, EvalAltResult, Scope, INT};

#[test]
fn test_var_scope() -> Result<(), EvalAltResult> {
    let mut engine = Engine::new();
    let mut scope = Scope::new();

    engine.eval_with_scope::<()>(&mut scope, "let x = 4 + 5")?;
    assert_eq!(engine.eval_with_scope::<INT>(&mut scope, "x")?, 9);
    engine.eval_with_scope::<()>(&mut scope, "x = x + 1; x = x + 2;")?;
    assert_eq!(engine.eval_with_scope::<INT>(&mut scope, "x")?, 12);
    engine.eval_with_scope::<()>(&mut scope, "{let x = 3}")?;
    assert_eq!(engine.eval_with_scope::<INT>(&mut scope, "x")?, 12);

    Ok(())
}

#[test]
fn test_scope_eval() -> Result<(), EvalAltResult> {
    let mut engine = Engine::new();

    // First create the state
    let mut scope = Scope::new();

    // Then push some initialized variables into the state
    // NOTE: Remember the default numbers used by Rhai are INT and f64.
    //       Better stick to them or it gets hard to work with other variables in the script.
    scope.push("y", 42 as INT);
    scope.push("z", 999 as INT);

    // First invocation
    engine
        .eval_with_scope::<()>(&mut scope, " let x = 4 + 5 - y + z; y = 1;")
        .expect("y and z not found?");

    // Second invocation using the same state
    let result = engine.eval_with_scope::<INT>(&mut scope, "x")?;

    println!("result: {}", result); // should print 966

    // Variable y is changed in the script
    assert_eq!(
        scope
            .get_value::<INT>("y")
            .expect("variable y should exist"),
        1
    );

    Ok(())
}
