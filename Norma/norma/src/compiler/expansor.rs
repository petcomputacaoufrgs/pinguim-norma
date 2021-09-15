use crate::compiler::ast;
use std::collections::HashMap;
use indexmap::IndexSet;
use crate::interpreter::program::Program;
use crate::compiler::error::Diagnostics;

fn expand(ast: &ast::Program, diagnostics: &mut Diagnostics) -> Program {
    todo!()
}

struct Expansor {
    precompileds: HashMap<String, PreCompiled>,
    target_macros: IndexSet<String>,
    expanding_macros: Vec<ExpandingMacro>,
}

struct PreCompiled {
    name: ast::Symbol,
    macro_type: ast::MacroType,
    program: Program,  // Program do interpretador
}

struct ExpandingMacro {
    precompiled: PreCompiled,
    label: String,
}

// tirar uma macro do target_macros
// faz um laço pelas instruções
// enquanto for uma operação builtin traduz ela
// se for outra chamada de macro, ve sem tem ela no precompiled
// se já tiver la faz a expansao direto
// se nao estiver, monta o expanding_macro com a macro atual e coloca na pilha
// tenta fazer a expansao do macro que dependemos (retira da do target_macros)
// quando chegar no final, coloca-o no precompiled
// enquanto tiver macros no expanding_macros e a medida que precisa compila os macros que são dependencias
// quando nao tiver mais nada no expanding_macros, tenta tirar outra do target_macros
// quando não tiver mais nada no target_macro, faz a expansão da main que é o mesmo algoritmo porém:

// se uma macro nao tiver precompilada, é por que nao existe aquela macro (por que todas ja foram precompiladas)

// expansao:
// 1- traduzir instruções
// 2- prefixar os rótulos dos macros aninhados (internos)
// 3- mapear rótulos de saída (true, false, invalidos) para rótulos externos