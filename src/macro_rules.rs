macro_rules! some_box {
    ($a: expr) => {
        Some(Box::new($a))
    };
}

#[allow(unused_macros)]
macro_rules! oper_ast {
    ($o: expr, $l: expr, $r: expr) => {
        AST {
            operator: Some($o),
            left_operand: some_box!($l),
            right_operand: some_box!($r),
            ..Default::default()
        }
    };
}

macro_rules! num_ast {
    ($a: expr) => {
        AST {
            num: Some($a),
            ..Default::default()
        }
    };
}

macro_rules! some_num_ast {
    ($a: expr) => {
        Some(num_ast!($a))
    };
}

macro_rules! init_opt_oper_ast {
    ($o: expr, $l: expr) => {
        Some(AST {
            operator: Some($o),
            left_operand: some_box!($l),
            ..Default::default()
        })
    };
}

#[allow(unused_imports)]
pub(crate) use {init_opt_oper_ast, num_ast, oper_ast, some_box, some_num_ast};
