// use minijinja::{context, Environment};
//
// pub fn fff() {
//     log::debug!("this is ffff");
//     let env = Environment::new();
//     let expr = env.compile_expression("number < 42").unwrap();
//     let result = expr.eval(context!(number => 23)).unwrap();
//     log::debug!("result {result}", result = result);
//     assert_eq!(result.is_true(), true);
//     log::debug!("result {result}", result = result);
// }
