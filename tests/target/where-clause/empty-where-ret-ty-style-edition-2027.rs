//@ rustfmt-style_edition: 2027

fn implicit_ret_ty_1() {}
fn implicit_ret_ty_2() /* comment */ where {}
fn implicit_ret_ty_3() where /* comment */ {}
fn implicit_ret_ty_4() /* comment */ where /* comment */ {}

fn foo() -> () {}
fn bar() -> () /* comment */ {}
fn baz() -> () /* comment */ {}
fn qux() -> () /* comment */ /* comment */ {}

fn line_comments() -> () // c1
where // c2
{
}
