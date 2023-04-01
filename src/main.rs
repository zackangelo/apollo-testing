use std::time::Instant;

use apollo_compiler::{ApolloCompiler, HirDatabase};

const SCHEMA: &str = include_str!("../schema.graphql");
const QUERY: &str = include_str!("../query.graphql");

fn main() {
    fast();
    slow();
}

fn fast() {
    let mut compiler = ApolloCompiler::new();
    compiler.add_type_system(SCHEMA, "schema.graphql");

    let file_id = compiler.add_executable(QUERY, "query.graphql");

    for _ in 0..10 {
        compiler.update_executable(file_id, QUERY);
        let start = Instant::now();
        let _diags = compiler.validate();
        println!(
            "fast validation took {} us",
            Instant::now().duration_since(start).as_micros()
        )
    }
}

fn slow() {
    let mut compiler = ApolloCompiler::new();
    compiler.add_type_system(SCHEMA, "schema.graphql");
    let ts = compiler.db.type_system();

    drop(compiler);

    for _ in 0..10 {
        let mut another_compiler = ApolloCompiler::new();
        another_compiler.set_type_system_hir(ts.clone());
        another_compiler.add_executable(QUERY, "query.graphql");

        let start = Instant::now();
        let _diags = another_compiler.validate();
        println!(
            "slow validation took {} us",
            Instant::now().duration_since(start).as_micros()
        )
    }
}

fn print_compiler_errors(compiler: ApolloCompiler) {
    let diags = compiler.validate();
    for diag in diags.iter() {
        if diag.data.is_error() {
            println!("{}", diag);
        }
    }
}
