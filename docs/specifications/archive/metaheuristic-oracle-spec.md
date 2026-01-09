# Metaheuristic Oracle: Self-Supervised Corpus Generation (Toyota Way)

**Version**: 1.1.0 (Toyota Way Revision)
**Status**: Draft
**Authors**: Depyler Team
**Date**: 2025-11-29

## Abstract
This specification defines a metaheuristic-optimized Oracle system for compile error classification. By adhering to **Toyota Way** principles—specifically **Jidoka** (intelligent automation) and **Muda** elimination—we replace wasteful manual template generation with a self-supervised learning loop. This system mines the Python standard library, generates "living" test cases, and uses evolutionary algorithms to maximize corpus diversity and training signal quality.

## 1. Executive Summary

### 1.1 Problem: The "Muda" of Synthetic Templates
The previous Oracle relied on static templates (approx. 30k combinations). This approach suffers from **Diminishing Returns**:
-   **Saturation**: New templates yield minimal accuracy gains.
-   **Artificiality**: Synthetic errors lack the entropy of real code.
-   **Maintenance Waste**: Manual template updates are slow and error-prone.

> **Annotation [1]:** **Entropy in Software**: As noted by Devanbu et al., software code is "natural" and repetitive, but bugs often introduce "unnaturalness" or high entropy. Synthetic templates often fail to capture this subtle distribution, leading to training on data that is "too clean" (overfitting to the template structure rather than the error semantics).

### 1.2 Solution: Metaheuristic Self-Supervision
We propose a dynamic generation system that:
1.  **Mines** real patterns from the Python stdlib (Genchi Genbutsu).
2.  **Evolves** test cases using Differential Evolution to maximize error coverage (Kaizen).
3.  **Learns** from `rustc` feedback directly (CITL).

## 2. Architecture (Toyota Principles)

```
┌─────────────────────────────────────────────────────────────────────────┐
│                    Self-Supervised Corpus Pipeline                       │
└─────────────────────────────────────────────────────────────────────────┘

┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   Python     │    │   Example    │    │   Depyler    │    │    rustc     │
│   Stdlib     │ →  │  Generator   │ →  │  Transpile   │ →  │   Compile    │
│   (Raw Material)  │  (Jidoka)    │    │ (Processing) │    │ (Quality Check)│
└──────────────┘    └──────────────┘    └──────────────┘    └──────────────┘
       │                   │                   │                   │
       ▼                   ▼                   ▼                   ▼
  Signatures         Python Code          Rust Code           Errors
  + Docstrings       (Evolved)            (may fail)          (Labeled Data)
                          │                                        │
                          │     ┌──────────────────────────┐       │
                          └────→│  Metaheuristic Optimizer │←──────┘
                                │  (Differential Evolution) │
                                └──────────────────────────┘
                                           │
                                           ▼
                                ┌──────────────────────────┐
                                │   Oracle Classifier      │
                                │   (Random Forest + TF-IDF)│
                                └──────────────────────────┘
```

## 3. Components

### 3.1 Stdlib Parser (Mining Raw Materials)
Instead of creating fake functions, we parse real Python stdlib signatures. This ensures our training data respects the "Naturalness of Software" hypothesis.

> **Annotation [2]:** **Naturalness Hypothesis**: Hindle et al. (2012) demonstrated that code is repetitive and predictable (low entropy). Training on "real" code ensures the model learns the statistical properties of actual developer code, not just the idiosyncrasies of a generator script.

### 3.2 Metaheuristic Optimizer (aprender::metaheuristics)
We use **Differential Evolution (DE)** to optimize the generation parameters. The objective function is not just "compile failure" (which is easy), but "compile failure *diversity*".

> **Annotation [3]:** **Search Based Software Engineering (SBSE)**: Harman and Jones (2001) defined SBSE as using search techniques (like Genetic Algorithms) to solve SE problems. Here, we search the space of "program inputs" to maximize the "fault revealing" capability of the corpus.

**Optimization Parameters:**
-   `mutation_rate`: Probability of introducing a type error.
-   `complexity`: Depth of nested calls.
-   `diversity_penalty`: Penalty for generating errors we already have (preventing mode collapse).

> **Annotation [4]:** **Mode Collapse Mitigation**: In Generative Adversarial Networks (GANs), mode collapse occurs when the generator produces only one type of output. By explicitly penalizing low diversity in the fitness function, we ensure the corpus covers the long tail of error types (Feldman's Long Tail distribution).

### 3.3 CITL Integration (The Feedback Loop)
The compiler acts as the **Andon Cord**. When an error is detected, it is not just discarded; it is labeled and fed back into the system.

> **Annotation [5]:** **Active Learning**: Settles (2009) describes Active Learning as querying the oracle (here, the compiler) only for the most informative samples. We prioritize generating samples that trigger *unseen* or *ambiguous* error states, maximizing information gain per compute cycle.

## 4. Methodology (Academic Rigor)

### 4.1 Diversity Monitoring
To prevent the "Muda" of redundant data, we employ a Diversity Monitor.

```rust
impl SyntheticGenerator {
    fn diversity_score(&self, sample: &Self::Output) -> f64 {
        // Calculate distance from existing cluster centroids
        // High distance = High Value (keep)
        // Low distance = Low Value (discard/waste)
    }
}
```

> **Annotation [6]:** **Curiosity-Driven Exploration**: Pathak et al. (2017) introduced curiosity as an intrinsic reward for exploring unseen states. Our diversity score acts as this "curiosity" signal, driving the generator to explore the boundaries of the transpiler's capability.

### 4.2 Weak Supervision for Labeling
Since manual labeling is "Muri" (Overburden), we use Weak Supervision (Snorkel) to programmatically label errors based on heuristics, then train a robust model on the noisy labels.

> **Annotation [7]:** **Weak Supervision**: Ratner et al. (2017) showed that large volumes of weakly labeled data (programmatically generated) often outperform small volumes of hand-labeled data. This allows us to scale the corpus to 100k+ examples without human bottleneck.

## 5. Roadmap (Kaizen)

### Phase 1: Foundation (Week 1)
-   **Goal:** Parse stdlib and generate basic calls.
-   **Metric:** 1000 valid Python examples generated.

### Phase 2: Optimization (Week 2)
-   **Goal:** Implement Differential Evolution.
-   **Metric:** Increase Error Diversity Score from 0.3 to 0.8.
-   **Annotation [8]:** **Exploration vs. Exploitation**: We start with high exploration (random generation) and anneal towards exploitation (focusing on known weak spots in Depyler), simulating a Simulated Annealing process.

### Phase 3: Integration (Week 3)
-   **Goal:** Full loop with `entrenar`.
-   **Metric:** 95% Classification Accuracy on hold-out set.

## 6. References & Annotations

> **Annotation [9]:** **Automated Test Generation**: This system is conceptually similar to EvoSuite (Fraser and Arcuri, 2011) for Java, which evolves unit tests to maximize coverage. We evolve *inputs* to maximize *error coverage*.

> **Annotation [10]:** **Self-Correction**: The ultimate goal is a system that can "heal" itself. If the Oracle detects a Type Mismatch, and the Fixer resolves it, and the Doctest passes, we have a closed-loop autopoietic system (Maturana & Varela, 1980) that maintains its own structural integrity.

### Bibliography
1.  Hindle, A., et al. (2012). On the naturalness of software. *ICSE*.
2.  Harman, M., & Jones, B. F. (2001). Search-based software engineering. *Information and Software Technology*.
3.  Feldman, V. (2020). Does learning require memorization? A short tale about a long tail. *STOC*.
4.  Settles, B. (2009). Active learning literature survey.
5.  Pathak, D., et al. (2017). Curiosity-driven exploration by self-supervised prediction. *CVPR*.
6.  Ratner, A., et al. (2017). Snorkel: Rapid training data creation with weak supervision. *VLDB*.
7.  Fraser, G., & Arcuri, A. (2011). EvoSuite: automatic test suite generation for object-oriented software. *FSE*.
8.  Maturana, H. R., & Varela, F. J. (1980). Autopoiesis and cognition: The realization of the living.