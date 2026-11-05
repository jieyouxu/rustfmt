//@ rustfmt-style_edition: 2024

fn implicit_ret_ty_1() where {}
fn implicit_ret_ty_2() /* comment */ where {}
fn implicit_ret_ty_3() where /* comment */ {}
fn implicit_ret_ty_4() /* comment */ where /* comment */ {}

fn foo() -> () where {}
fn bar() -> () /* comment */ where {}
fn baz() -> () where /* comment */ {}
fn qux() -> () /* comment */ where /* comment */ {}
