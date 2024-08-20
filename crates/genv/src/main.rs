mod run;

use std::io::Read;

fn genv_ast(source: &str) {
    println!("{source}");
    use koto_bytecode::{Compiler, CompilerSettings};
    use koto_parser::Parser;

    match Parser::parse(source) {
        Ok(ast) => {

            // if Compiler::compile(&ast, CompilerSettings::default()).is_ok() {
            //     panic!("\nUnexpected success while compiling: {source}");
            // }
            //ast.span
            //ast.consume_constants()

            let a=Compiler::compile(&ast, CompilerSettings::default());
            //println!("compile (),{:?}",a);
            match a {
                Ok(a) => {


                    let mut byte_vec = Vec::new();
                    for byte in a.0.bytes() {
                        byte_vec.push(byte.unwrap());
                    }
                    let byte_str = String::from_utf8(byte_vec).unwrap();
                    println!("Byte to String: {}", byte_str);
                    println!("a.1.source: {}", a.1.source);
                    //a.0.bytes()
                }
                Err(e) => {
                    panic!("\nUnexpected success while compiling: {source}");
                }
            }
            //println!("ast.constants(),{:?}",ast.constants());

            for node in ast.nodes() {
               // println!("ast node {:?},{:?}",node,node.span);

            }


        }
        Err(error) => panic!("{error} - {:?}", error.span.start),
    }
}


fn main() {
    let source = "
hi = |a| 'Hello!'
hi(2)
f = |x, y, z|
  x *= 100
  y *= 10
  x + y + z
f(2, 3, 4)
";

    genv_ast(source);
}
