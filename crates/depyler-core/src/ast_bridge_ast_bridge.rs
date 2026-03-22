pub struct AstBridge {
    source_code: Option<String>,
    annotation_extractor: AnnotationExtractor,
    annotation_parser: AnnotationParser,
    type_env: crate::type_system::type_environment::TypeEnvironment,
}

    fn default() -> Self {
        Self::new()
    }

    pub fn new() -> Self {
        Self {
            source_code: None,
            annotation_extractor: AnnotationExtractor::new(),
            annotation_parser: AnnotationParser::new(),
            type_env: crate::type_system::type_environment::TypeEnvironment::new(),
        }
    }

    pub fn with_source(mut self, source: String) -> Self {
        self.source_code = Some(source);
        self
    }

    pub fn python_to_hir(
        mut self,
        module: ast::Mod,
    ) -> Result<(HirModule, crate::type_system::type_environment::TypeEnvironment)> {
        let hir = match module {
            ast::Mod::Module(m) => self.convert_module(m)?,
            _ => bail!("Only module-level code is supported"),
        };
        Ok((hir, self.type_env))
    }

pub fn python_to_hir(
    module: ast::Mod,
) -> Result<(HirModule, crate::type_system::type_environment::TypeEnvironment)> {
    AstBridge::new().python_to_hir(module)
}

fn propagate_can_fail_through_calls(functions: &mut [HirFunction]) {
    // Build a map of function names to can_fail status for quick lookup
    let mut can_fail_map: std::collections::HashMap<String, bool> =
        functions.iter().map(|f| (f.name.clone(), f.properties.can_fail)).collect();

    // Fixed-point iteration: keep propagating until no changes occur
    let mut changed = true;
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 100; // Prevent infinite loops

    while changed && iterations < MAX_ITERATIONS {
        changed = false;
        iterations += 1;

        for func in functions.iter_mut() {
            // Skip if already marked as can_fail
            if func.properties.can_fail {
                continue;
            }

            // Check if this function calls any function that can fail
            if calls_failing_function(&func.body, &can_fail_map) {
                func.properties.can_fail = true;
                can_fail_map.insert(func.name.clone(), true);
                changed = true;
            }
        }
    }
}

fn calls_failing_function(
    stmts: &[HirStmt],
    can_fail_map: &std::collections::HashMap<String, bool>,
) -> bool {
    for stmt in stmts {
        if stmt_calls_failing_function(stmt, can_fail_map) {
            return true;
        }
    }
    false
}

    fn parse_python_to_hir(source: &str) -> HirModule {
        let statements = Suite::parse(source, "<test>").unwrap();
        let ast = rustpython_ast::Mod::Module(rustpython_ast::ModModule {
            body: statements,
            type_ignores: vec![],
            range: Default::default(),
        });
        let (hir, _type_env) =
            AstBridge::new().with_source(source.to_string()).python_to_hir(ast).unwrap();
        hir
    }

    fn parse_python_to_hir(source: &str) -> HirModule {
        let statements = Suite::parse(source, "<test>").unwrap();
        let ast = rustpython_ast::Mod::Module(rustpython_ast::ModModule {
            body: statements,
            type_ignores: vec![],
            range: Default::default(),
        });
        let (hir, _type_env) =
            AstBridge::new().with_source(source.to_string()).python_to_hir(ast).unwrap();
        hir
    }
