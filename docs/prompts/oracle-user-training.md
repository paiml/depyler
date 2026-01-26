# OPERATION ORACLE FEEDBACK — Wire User Corpus Training

**Ticket**: DEPYLER-ORACLE-TRAIN
**Priority**: P1
**Est**: 200-400 lines

## Problem

Oracle is read-only. Users cannot train on their own corpus. The value prop of "learn your codebase patterns" doesn't work.

## Gap Analysis

| Component | Exists | Wired |
|-----------|--------|-------|
| `NgramFixPredictor::learn_pattern()` | ✅ | ❌ |
| `NgramFixPredictor::record_feedback()` | ✅ | ❌ |
| `TrainingDataset::add()` | ✅ | ❌ |
| CLI command for training | ❌ | — |

## Implementation

### 1. Add CLI Command (lib.rs)

```rust
#[derive(Parser)]
pub enum Commands {
    // ... existing commands ...

    /// Train Oracle on user corpus
    Train {
        /// Path to corpus directory
        #[arg(short, long)]
        corpus: PathBuf,

        /// Output model path (default: ~/.depyler/oracle_user.bin)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}
```

### 2. Wire Feedback in Converge (converge/mod.rs ~line 179)

After a fix is successfully applied:

```rust
// Existing: track fix
state.fixes_applied.push(fix.clone());

// NEW: feed back to Oracle
if let Some(oracle) = &mut self.oracle {
    oracle.record_feedback(
        &error.code,
        &error.message,
        &fix.pattern,
        true, // success
    );
}
```

### 3. Persist User Model (ngram.rs)

Add save/load for user-local patterns:

```rust
impl NgramFixPredictor {
    pub fn save_user_model(&self, path: &Path) -> Result<()> {
        let data = bincode::serialize(&self.patterns)?;
        std::fs::write(path, data)?;
        Ok(())
    }

    pub fn load_user_model(&mut self, path: &Path) -> Result<()> {
        if path.exists() {
            let data = std::fs::read(path)?;
            let patterns: HashMap<String, FixPattern> = bincode::deserialize(&data)?;
            self.patterns.extend(patterns);
        }
        Ok(())
    }
}
```

### 4. Train Command Handler

```rust
async fn handle_train(corpus: PathBuf, output: Option<PathBuf>) -> Result<()> {
    let output = output.unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap()
            .join(".depyler")
            .join("oracle_user.bin")
    });

    // 1. Run converge on corpus with feedback enabled
    let mut oracle = Oracle::load_default()?;
    oracle.load_user_model(&output)?;

    for file in glob(&corpus.join("**/*.py"))? {
        let result = converge_file(&file, &mut oracle)?;
        // Feedback is recorded during converge
    }

    // 2. Save updated model
    oracle.save_user_model(&output)?;

    println!("Trained on {} files, saved to {}", count, output.display());
    Ok(())
}
```

## Files to Modify

1. `crates/depyler/src/lib.rs` — Add Train command
2. `crates/depyler/src/converge/mod.rs` — Wire record_feedback (~line 179)
3. `crates/depyler-oracle/src/ngram.rs` — Add save/load_user_model
4. `crates/depyler-oracle/src/lib.rs` — Expose user model loading in Oracle struct

## Validation

```bash
# Train on user corpus
depyler train --corpus ~/myproject/

# Verify model created
ls ~/.depyler/oracle_user.bin

# Subsequent transpiles should use learned patterns
depyler compile ~/myproject/new_file.py
```

## Success Criteria

- [ ] `depyler train --corpus PATH` works
- [ ] Fixes applied during converge feed back to Oracle
- [ ] User model persists to ~/.depyler/oracle_user.bin
- [ ] Subsequent transpiles load and use user model
- [ ] No regression on existing tests
