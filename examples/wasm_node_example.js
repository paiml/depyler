#!/usr/bin/env node

/**
 * Depyler WASM Node.js Example
 * 
 * This example demonstrates how to use Depyler's WebAssembly bindings
 * in a Node.js environment for server-side Python to Rust transpilation.
 * 
 * Prerequisites:
 * 1. Build the WASM package: cd crates/depyler-wasm && wasm-pack build --target nodejs
 * 2. Install the package: npm link ./pkg
 * 3. Run this example: node wasm_node_example.js
 */

const { DepylerWasm, WasmTranspileOptions } = require('depyler-wasm');

// Example Python code snippets
const examples = {
    fibonacci: `
def fibonacci(n: int) -> int:
    """Calculate the nth Fibonacci number using recursion."""
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
`,

    dataProcessing: `
def process_data(data: list) -> dict:
    """Process a list of numbers and return statistics."""
    if not data:
        return {"count": 0, "sum": 0, "average": 0}
    
    total = sum(data)
    count = len(data)
    average = total / count
    
    return {
        "count": count,
        "sum": total,
        "average": average,
        "min": min(data),
        "max": max(data)
    }
`,

    stringManipulation: `
def clean_text(text: str) -> str:
    """Clean and normalize text input."""
    # Remove leading/trailing whitespace
    text = text.strip()
    
    # Convert to lowercase
    text = text.lower()
    
    # Replace multiple spaces with single space
    import re
    text = re.sub(r'\\s+', ' ', text)
    
    return text
`,

    classExample: `
class Calculator:
    """A simple calculator class."""
    
    def __init__(self):
        self.result = 0
    
    def add(self, x: int) -> None:
        self.result += x
    
    def subtract(self, x: int) -> None:
        self.result -= x
    
    def multiply(self, x: int) -> None:
        self.result *= x
    
    def get_result(self) -> int:
        return self.result
    
    def reset(self) -> None:
        self.result = 0
`
};

async function main() {
    console.log('Depyler WASM Node.js Example\n');
    console.log('=============================\n');

    // Initialize Depyler WASM
    const depyler = new DepylerWasm();
    console.log(`Depyler version: ${depyler.get_version()}\n`);

    // Example 1: Basic transpilation
    console.log('Example 1: Basic Transpilation');
    console.log('------------------------------');
    await transpileExample(depyler, 'fibonacci', examples.fibonacci);

    // Example 2: Transpilation with different options
    console.log('\nExample 2: Transpilation with Custom Options');
    console.log('--------------------------------------------');
    const customOptions = new WasmTranspileOptions();
    customOptions.set_verify(false);
    customOptions.set_optimize(true);
    customOptions.set_emit_docs(true);
    await transpileWithOptions(depyler, 'dataProcessing', examples.dataProcessing, customOptions);

    // Example 3: Code analysis
    console.log('\nExample 3: Static Code Analysis');
    console.log('--------------------------------');
    await analyzeCode(depyler, 'stringManipulation', examples.stringManipulation);

    // Example 4: Performance benchmarking
    console.log('\nExample 4: Performance Benchmarking');
    console.log('-----------------------------------');
    await benchmarkTranspilation(depyler, 'fibonacci', examples.fibonacci);

    // Example 5: Error handling
    console.log('\nExample 5: Error Handling');
    console.log('-------------------------');
    const invalidCode = `
def broken_function(
    # Missing closing parenthesis and body
`;
    await demonstrateErrorHandling(depyler, invalidCode);

    // Example 6: Batch processing
    console.log('\nExample 6: Batch Processing');
    console.log('---------------------------');
    await batchProcess(depyler, examples);
}

async function transpileExample(depyler, name, code) {
    const options = new WasmTranspileOptions();
    
    try {
        const result = depyler.transpile(code, options);
        
        if (result.success()) {
            console.log(`✓ ${name} transpilation successful!`);
            console.log(`  Time: ${result.transpile_time_ms().toFixed(2)}ms`);
            console.log(`  Energy: ${(result.energy_estimate().joules() * 1000).toFixed(3)}mJ`);
            console.log(`  PMAT Score: ${result.quality_metrics().pmat_score().toFixed(2)}`);
            console.log(`  Complexity: ${result.quality_metrics().code_complexity()}`);
            console.log('\n  Generated Rust (first 200 chars):');
            console.log('  ' + result.rust_code().substring(0, 200) + '...\n');
        } else {
            console.log(`✗ ${name} transpilation failed:`);
            result.errors().forEach(err => console.log(`  - ${err}`));
        }
    } catch (e) {
        console.error(`Error transpiling ${name}: ${e.message}`);
    }
}

async function transpileWithOptions(depyler, name, code, options) {
    try {
        const result = depyler.transpile(code, options);
        
        if (result.success()) {
            console.log(`✓ ${name} transpiled with custom options`);
            console.log(`  Verify: ${options.verify()}`);
            console.log(`  Optimize: ${options.optimize()}`);
            console.log(`  Emit Docs: ${options.emit_docs()}`);
            console.log(`  Warnings: ${result.warnings().length}`);
            
            if (result.warnings().length > 0) {
                console.log('  Warnings:');
                result.warnings().forEach(warn => console.log(`    - ${warn}`));
            }
        }
    } catch (e) {
        console.error(`Error: ${e.message}`);
    }
}

async function analyzeCode(depyler, name, code) {
    try {
        const analysis = depyler.analyze_code(code);
        
        console.log(`Analysis of ${name}:`);
        console.log(`  Complexity: ${analysis.complexity}`);
        console.log(`  Cyclomatic Complexity: ${analysis.cyclomatic_complexity}`);
        console.log(`  Functions: ${analysis.functions.length}`);
        console.log(`  Imports: ${analysis.imports.length}`);
        
        if (analysis.anti_patterns.length > 0) {
            console.log('  Anti-patterns detected:');
            analysis.anti_patterns.forEach(pattern => {
                console.log(`    - Line ${pattern.line}: ${pattern.pattern} - ${pattern.description}`);
            });
        }
        
        if (analysis.suggestions.length > 0) {
            console.log('  Optimization suggestions:');
            analysis.suggestions.forEach(suggestion => {
                console.log(`    - Line ${suggestion.line}: ${suggestion.message}`);
            });
        }
    } catch (e) {
        console.error(`Analysis error: ${e.message}`);
    }
}

async function benchmarkTranspilation(depyler, name, code) {
    try {
        console.log(`Benchmarking ${name} (10 iterations)...`);
        const benchmark = depyler.benchmark(code, 10);
        
        console.log('  Results:');
        console.log(`    Min: ${benchmark.min_ms.toFixed(2)}ms`);
        console.log(`    Max: ${benchmark.max_ms.toFixed(2)}ms`);
        console.log(`    Mean: ${benchmark.mean_ms.toFixed(2)}ms`);
        console.log(`    Median: ${benchmark.median_ms.toFixed(2)}ms`);
        console.log(`    Std Dev: ${benchmark.std_dev_ms.toFixed(2)}ms`);
        
        // Performance assessment
        if (benchmark.mean_ms < 50) {
            console.log('  ✓ Excellent performance (<50ms)');
        } else if (benchmark.mean_ms < 100) {
            console.log('  ✓ Good performance (<100ms)');
        } else {
            console.log('  ⚠ Performance could be improved');
        }
    } catch (e) {
        console.error(`Benchmark error: ${e.message}`);
    }
}

async function demonstrateErrorHandling(depyler, invalidCode) {
    const options = new WasmTranspileOptions();
    
    try {
        const result = depyler.transpile(invalidCode, options);
        
        if (!result.success()) {
            console.log('✓ Error handling working correctly:');
            result.errors().forEach(err => console.log(`  - ${err}`));
        }
    } catch (e) {
        console.log('✓ Exception caught correctly:');
        console.log(`  - ${e.message}`);
    }
}

async function batchProcess(depyler, codeExamples) {
    const options = new WasmTranspileOptions();
    let successCount = 0;
    let totalTime = 0;
    let totalEnergy = 0;
    
    console.log(`Processing ${Object.keys(codeExamples).length} examples...`);
    
    for (const [name, code] of Object.entries(codeExamples)) {
        try {
            const result = depyler.transpile(code, options);
            if (result.success()) {
                successCount++;
                totalTime += result.transpile_time_ms();
                totalEnergy += result.energy_estimate().joules();
            }
        } catch (e) {
            console.error(`  ✗ ${name}: ${e.message}`);
        }
    }
    
    console.log(`\nBatch Results:`);
    console.log(`  Success rate: ${successCount}/${Object.keys(codeExamples).length}`);
    console.log(`  Total time: ${totalTime.toFixed(2)}ms`);
    console.log(`  Average time: ${(totalTime / successCount).toFixed(2)}ms`);
    console.log(`  Total energy: ${(totalEnergy * 1000).toFixed(3)}mJ`);
    console.log(`  CO2 emissions: ${(totalEnergy * 0.475).toFixed(6)}g`);
}

// Run the examples
main().catch(console.error);