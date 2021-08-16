use crate::*;

test_case!(sin, async move {
    use Value::F64;

    let test_cases = vec![
        (
            "CREATE TABLE SingleItem (id INTEGER PRIMARY KEY)",
            Ok(Payload::Create),
        ),
        (
            r#"INSERT INTO SingleItem VALUES (0)"#,
            Ok(Payload::Insert(1)),
        ),
        (
            "SELECT SIN(0.5) AS sin FROM SingleItem",
            Ok(select!(
                "sin";
                F64;
                0.5_f64.sin()
            )),
        ),
        (
            "SELECT SIN(1) AS sin FROM SingleItem",
            Ok(select!(
                "sin";
                F64;
                1.0_f64.sin()
            )),
        ),
        (
            "SELECT SIN(-1) AS sin FROM SingleItem",
            Ok(select!(
                "sin";
                F64;
                (-1.0_f64).sin()
            )),
        ),
        (
            "SELECT SIN(0.976543125) AS sin FROM SingleItem",
            Ok(select!(
                "sin";
                F64;
                0.976543125_f64.sin()
            )),
        ),
        (
            "SELECT SIN(3) AS sin FROM SingleItem",
            Ok(select!(
                "sin";
                F64;
                3_f64.sin()
            )),
        ),
        (
            "SELECT SIN('3.15') AS sin FROM SingleItem",
            Ok(select!(
                "sin";
                F64;
                3.15_f64.sin()
            )),
        ),
        (
            "SELECT SIN(true) AS sin FROM SingleItem",
            Ok(select!(
                "sin";
                F64;
                1.0_f64.sin()
            )),
        ),
        (
            "SELECT SIN(false) AS sin FROM SingleItem",
            Ok(select!(
                "sin";
                F64;
                0.0_f64.sin()
            )),
        ),
        (
            "SELECT SIN('string') AS sin FROM SingleItem",
            Err(EvaluateError::FunctionRequiresFloatValue("SIN".to_string()).into()),
        ),
        (
            "SELECT SIN(null) AS sin FROM SingleItem",
            Err(EvaluateError::FunctionRequiresFloatValue("SIN".to_string()).into()),
        ),
        (
            "SELECT SIN() AS sin FROM SingleItem",
            Err(TranslateError::FunctionArgsLengthNotMatching {
                name: "SIN".to_owned(),
                expected: 1,
                found: 0,
            }
            .into()),
        ),
        (
            "SELECT SIN(1.0, 2.0) AS sin FROM SingleItem",
            Err(TranslateError::FunctionArgsLengthNotMatching {
                name: "SIN".to_owned(),
                expected: 1,
                found: 2,
            }
            .into()),
        ),
    ];

    for (sql, expected) in test_cases.into_iter() {
        test!(expected, sql);
    }
});

test_case!(cos, async move {
    use Value::F64;

    let test_cases = vec![
        (
            "CREATE TABLE SingleItem (id INTEGER PRIMARY KEY)",
            Ok(Payload::Create),
        ),
        (
            r#"INSERT INTO SingleItem VALUES (0)"#,
            Ok(Payload::Insert(1)),
        ),
        (
            "SELECT COS(0.5) AS cos FROM SingleItem",
            Ok(select!(
                "cos";
                F64;
                0.5_f64.cos()
            )),
        ),
        (
            "SELECT COS(1) AS cos FROM SingleItem",
            Ok(select!(
                "cos";
                F64;
                1.0_f64.cos()
            )),
        ),
        (
            "SELECT COS(-1) AS cos FROM SingleItem",
            Ok(select!(
                "cos";
                F64;
                (-1.0_f64).cos()
            )),
        ),
        (
            "SELECT COS(0.976543125) AS cos FROM SingleItem",
            Ok(select!(
                "cos";
                F64;
                0.976543125_f64.cos()
            )),
        ),
        (
            "SELECT COS(3) AS cos FROM SingleItem",
            Ok(select!(
                "cos";
                F64;
                3_f64.cos()
            )),
        ),
        (
            "SELECT COS('3.15') AS cos FROM SingleItem",
            Ok(select!(
                "cos";
                F64;
                3.15_f64.cos()
            )),
        ),
        (
            "SELECT COS(true) AS cos FROM SingleItem",
            Ok(select!(
                "cos";
                F64;
                1.0_f64.cos()
            )),
        ),
        (
            "SELECT COS(false) AS cos FROM SingleItem",
            Ok(select!(
                "cos";
                F64;
                0.0_f64.cos()
            )),
        ),
        (
            "SELECT COS('string') AS cos FROM SingleItem",
            Err(EvaluateError::FunctionRequiresFloatValue("COS".to_string()).into()),
        ),
        (
            "SELECT COS(null) AS cos FROM SingleItem",
            Err(EvaluateError::FunctionRequiresFloatValue("COS".to_string()).into()),
        ),
        (
            "SELECT COS() AS cos FROM SingleItem",
            Err(TranslateError::FunctionArgsLengthNotMatching {
                name: "COS".to_owned(),
                expected: 1,
                found: 0,
            }
            .into()),
        ),
        (
            "SELECT COS(1.0, 2.0) AS cos FROM SingleItem",
            Err(TranslateError::FunctionArgsLengthNotMatching {
                name: "COS".to_owned(),
                expected: 1,
                found: 2,
            }
            .into()),
        ),
    ];

    for (sql, expected) in test_cases.into_iter() {
        test!(expected, sql);
    }
});

test_case!(tan, async move {
    use Value::F64;

    let test_cases = vec![
        (
            "CREATE TABLE SingleItem (id INTEGER PRIMARY KEY)",
            Ok(Payload::Create),
        ),
        (
            r#"INSERT INTO SingleItem VALUES (0)"#,
            Ok(Payload::Insert(1)),
        ),
        (
            "SELECT TAN(0.5) AS tan FROM SingleItem",
            Ok(select!(
                "tan";
                F64;
                0.5_f64.tan()
            )),
        ),
        (
            "SELECT TAN(1) AS tan FROM SingleItem",
            Ok(select!(
                "tan";
                F64;
                1.0_f64.tan()
            )),
        ),
        (
            "SELECT TAN(-1) AS tan FROM SingleItem",
            Ok(select!(
                "tan";
                F64;
                (-1.0_f64).tan()
            )),
        ),
        (
            "SELECT TAN(0.976543125) AS tan FROM SingleItem",
            Ok(select!(
                "tan";
                F64;
                0.976543125_f64.tan()
            )),
        ),
        (
            "SELECT TAN(3) AS tan FROM SingleItem",
            Ok(select!(
                "tan";
                F64;
                3_f64.tan()
            )),
        ),
        (
            "SELECT TAN('3.15') AS tan FROM SingleItem",
            Ok(select!(
                "tan";
                F64;
                3.15_f64.tan()
            )),
        ),
        (
            "SELECT TAN(true) AS tan FROM SingleItem",
            Ok(select!(
                "tan";
                F64;
                1.0_f64.tan()
            )),
        ),
        (
            "SELECT TAN(false) AS tan FROM SingleItem",
            Ok(select!(
                "tan";
                F64;
                0.0_f64.tan()
            )),
        ),
        (
            "SELECT TAN('string') AS tan FROM SingleItem",
            Err(EvaluateError::FunctionRequiresFloatValue("TAN".to_string()).into()),
        ),
        (
            "SELECT TAN(null) AS tan FROM SingleItem",
            Err(EvaluateError::FunctionRequiresFloatValue("TAN".to_string()).into()),
        ),
        (
            "SELECT TAN() AS tan FROM SingleItem",
            Err(TranslateError::FunctionArgsLengthNotMatching {
                name: "TAN".to_owned(),
                expected: 1,
                found: 0,
            }
            .into()),
        ),
        (
            "SELECT TAN(1.0, 2.0) AS tan FROM SingleItem",
            Err(TranslateError::FunctionArgsLengthNotMatching {
                name: "TAN".to_owned(),
                expected: 1,
                found: 2,
            }
            .into()),
        ),
    ];

    for (sql, expected) in test_cases.into_iter() {
        test!(expected, sql);
    }
});
