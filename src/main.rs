use apollo_compiler::{ApolloCompiler, HirDatabase};

const SCHEMA: &str = include_str!("../schema.graphql");
const QUERY: &str = include_str!("../query.graphql");

fn main() {
    has_errors();

    // no_errors()
}

fn has_errors() {
    let mut compiler = ApolloCompiler::new();
    compiler.add_type_system(SCHEMA, "schema.graphql");

    let type_system = compiler.db.type_system();
    print_compiler_errors(compiler);

    let mut another_compiler = ApolloCompiler::new();
    another_compiler.set_type_system_hir(type_system);
    another_compiler.add_executable(QUERY, "query.graphql");

    print_compiler_errors(another_compiler);
}

fn no_errors() {
    let mut compiler = ApolloCompiler::new();
    compiler.add_type_system(SCHEMA, "schema.graphql");
    compiler.add_executable(QUERY, "query.graphql");

    print_compiler_errors(compiler);
}

fn print_compiler_errors(compiler: ApolloCompiler) {
    let diags = compiler.validate();
    for diag in diags.iter() {
        if diag.data.is_error() {
            println!("{}", diag);
        }
    }
}
