import { beforeEach, describe, expect, it } from "vitest";

// Import the quality scoring system
interface QualityMetrics {
  transpile_time_ms: number;
  parse_time_ms: number;
  ast_nodes: number;
  complexity_score: number;
  memory_usage_mb: number;
  energy_reduction: {
    joules: number;
    wattsAverage: number;
    co2Grams: number;
    confidence: number;
  };
}

interface PmatScore {
  productivity: number;
  maintainability: number;
  accessibility: number;
  testability: number;
  tdg: number; // Total Depyler Grade
}

class QualityScorer {
  private config = {
    targets: {
      transpile_p95_ms: 50,
      parse_p95_ms: 10,
      complexity_max: 10,
      memory_budget_mb: 100,
    },
    weights: {
      productivity: 0.3,
      maintainability: 0.25,
      accessibility: 0.25,
      testability: 0.2,
    },
    thresholds: {
      excellent: 0.9,
      good: 0.7,
      acceptable: 0.5,
    },
  };

  calculatePmatScore(metrics: QualityMetrics): PmatScore {
    const productivity = this.calculateProductivity(metrics);
    const maintainability = this.calculateMaintainability(metrics);
    const accessibility = this.calculateAccessibility(metrics);
    const testability = this.calculateTestability(metrics);

    const tdg = productivity * this.config.weights.productivity +
      maintainability * this.config.weights.maintainability +
      accessibility * this.config.weights.accessibility +
      testability * this.config.weights.testability;

    return {
      productivity,
      maintainability,
      accessibility,
      testability,
      tdg,
    };
  }

  private calculateProductivity(metrics: QualityMetrics): number {
    // Productivity based on transpilation speed and efficiency
    const speedScore = this.sigmoid(
      this.config.targets.transpile_p95_ms - metrics.transpile_time_ms,
      this.config.targets.transpile_p95_ms,
    );

    const energyScore = Math.min(metrics.energy_reduction.confidence, 1.0);

    return (speedScore * 0.6 + energyScore * 0.4);
  }

  private calculateMaintainability(metrics: QualityMetrics): number {
    // Maintainability based on code complexity and structure
    const complexityScore = this.exponentialDecay(
      metrics.complexity_score,
      this.config.targets.complexity_max,
    );

    const astEfficiencyScore = Math.min(1.0, 100 / Math.max(metrics.ast_nodes, 1));

    return (complexityScore * 0.7 + astEfficiencyScore * 0.3);
  }

  private calculateAccessibility(metrics: QualityMetrics): number {
    // Accessibility based on error handling and user feedback
    const parseTimeScore = this.exponentialDecay(
      metrics.parse_time_ms,
      this.config.targets.parse_p95_ms, // 10ms target
    );

    const memoryScore = this.exponentialDecay(
      metrics.memory_usage_mb,
      this.config.targets.memory_budget_mb,
    );

    return (parseTimeScore * 0.4 + memoryScore * 0.6);
  }

  private calculateTestability(metrics: QualityMetrics): number {
    // Testability based on determinism and reliability
    const consistencyScore = metrics.energy_reduction.confidence;
    const performanceScore = metrics.transpile_time_ms > 0 ? 1.0 : 0.0;

    return (consistencyScore * 0.6 + performanceScore * 0.4);
  }

  private sigmoid(value: number, target: number, steepness = 5): number {
    const x = value / target;
    return 1 / (1 + Math.exp(-steepness * (x - 0.5)));
  }

  private exponentialDecay(actual: number, target: number, rate = 0.1): number {
    if (actual <= target) {
      return 1.0;
    }
    return Math.exp(-rate * (actual - target) / target);
  }

  validateScore(score: PmatScore): { valid: boolean; issues: string[] } {
    const issues: string[] = [];

    // Check bounds
    const metrics = [
      score.productivity,
      score.maintainability,
      score.accessibility,
      score.testability,
      score.tdg,
    ];
    const names = ["productivity", "maintainability", "accessibility", "testability", "tdg"];

    metrics.forEach((metric, index) => {
      if (metric < 0 || metric > 1) {
        issues.push(`${names[index]} score ${metric} is out of bounds [0, 1]`);
      }
    });

    // Check TDG calculation
    const expectedTdg = score.productivity * this.config.weights.productivity +
      score.maintainability * this.config.weights.maintainability +
      score.accessibility * this.config.weights.accessibility +
      score.testability * this.config.weights.testability;

    if (Math.abs(score.tdg - expectedTdg) > 0.001) {
      issues.push(`TDG score ${score.tdg} does not match calculated value ${expectedTdg}`);
    }

    // Check for NaN values
    if (metrics.some((m) => isNaN(m))) {
      issues.push("Score contains NaN values");
    }

    return {
      valid: issues.length === 0,
      issues,
    };
  }

  getQualityGrade(tdgScore: number): string {
    if (tdgScore >= this.config.thresholds.excellent) {
      return "Excellent";
    } else if (tdgScore >= this.config.thresholds.good) {
      return "Good";
    } else if (tdgScore >= this.config.thresholds.acceptable) {
      return "Acceptable";
    } else {
      return "Needs Improvement";
    }
  }
}

describe("Quality Scoring System Integration", () => {
  let scorer: QualityScorer;

  beforeEach(() => {
    scorer = new QualityScorer();
  });

  describe("PMAT Score Calculation", () => {
    it("calculates scores for high-quality metrics", () => {
      const excellentMetrics: QualityMetrics = {
        transpile_time_ms: 15, // Under 50ms target
        parse_time_ms: 3, // Under 10ms target
        ast_nodes: 25, // Reasonable complexity
        complexity_score: 2, // Low complexity
        memory_usage_mb: 5, // Well under 100MB
        energy_reduction: {
          joules: 0.001,
          wattsAverage: 1.0,
          co2Grams: 0.000475,
          confidence: 0.95, // High confidence
        },
      };

      const score = scorer.calculatePmatScore(excellentMetrics);

      expect(score.productivity).toBeGreaterThan(0.8);
      expect(score.maintainability).toBeGreaterThan(0.8);
      expect(score.accessibility).toBeGreaterThan(0.8);
      expect(score.testability).toBeGreaterThan(0.8);
      expect(score.tdg).toBeGreaterThan(0.8);

      const validation = scorer.validateScore(score);
      expect(validation.valid).toBe(true);
      expect(validation.issues).toHaveLength(0);
    });

    it("calculates scores for poor-quality metrics", () => {
      const poorMetrics: QualityMetrics = {
        transpile_time_ms: 150, // Over 50ms target
        parse_time_ms: 25, // Over 10ms target
        ast_nodes: 500, // High complexity
        complexity_score: 15, // High complexity
        memory_usage_mb: 200, // Over 100MB
        energy_reduction: {
          joules: 0.1,
          wattsAverage: 50.0,
          co2Grams: 0.0475,
          confidence: 0.3, // Low confidence
        },
      };

      const score = scorer.calculatePmatScore(poorMetrics);

      expect(score.productivity).toBeLessThan(0.5);
      expect(score.maintainability).toBeLessThan(0.5);
      expect(score.accessibility).toBeLessThan(0.5);
      expect(score.testability).toBeLessThan(0.7);
      expect(score.tdg).toBeLessThan(0.5);

      const validation = scorer.validateScore(score);
      expect(validation.valid).toBe(true);
    });

    it("handles edge case metrics correctly", () => {
      const edgeCaseMetrics: QualityMetrics = {
        transpile_time_ms: 0, // Zero time
        parse_time_ms: 0, // Zero time
        ast_nodes: 1, // Minimal AST
        complexity_score: 0, // No complexity
        memory_usage_mb: 0, // No memory
        energy_reduction: {
          joules: 0,
          wattsAverage: 0,
          co2Grams: 0,
          confidence: 1.0, // Perfect confidence
        },
      };

      const score = scorer.calculatePmatScore(edgeCaseMetrics);

      expect(score.productivity).toBeLessThan(1.0); // Can't be perfect with 0 time
      expect(score.maintainability).toBeGreaterThan(0.8); // Should be high
      expect(score.accessibility).toBeGreaterThan(0.8); // Should be high
      expect(score.testability).toBeLessThan(1.0); // Can't be perfect with 0 performance

      const validation = scorer.validateScore(score);
      expect(validation.valid).toBe(true);
    });
  });

  describe("Score Validation", () => {
    it("validates correct scores", () => {
      const validScore: PmatScore = {
        productivity: 0.85,
        maintainability: 0.78,
        accessibility: 0.92,
        testability: 0.88,
        tdg: 0.85 * 0.3 + 0.78 * 0.25 + 0.92 * 0.25 + 0.88 * 0.2, // Correct calculation
      };

      const validation = scorer.validateScore(validScore);
      expect(validation.valid).toBe(true);
      expect(validation.issues).toHaveLength(0);
    });

    it("detects out-of-bounds scores", () => {
      const invalidScore: PmatScore = {
        productivity: 1.5, // Over 1.0
        maintainability: -0.1, // Under 0.0
        accessibility: 0.8,
        testability: 0.7,
        tdg: 0.8,
      };

      const validation = scorer.validateScore(invalidScore);
      expect(validation.valid).toBe(false);
      expect(validation.issues.length).toBeGreaterThan(0);
      expect(validation.issues.some(issue => issue.includes("productivity score 1.5"))).toBe(true);
      expect(validation.issues.some(issue => issue.includes("maintainability score -0.1"))).toBe(true);
    });

    it("detects incorrect TDG calculation", () => {
      const incorrectScore: PmatScore = {
        productivity: 0.8,
        maintainability: 0.7,
        accessibility: 0.9,
        testability: 0.6,
        tdg: 0.5, // Incorrect TDG
      };

      const validation = scorer.validateScore(incorrectScore);
      expect(validation.valid).toBe(false);
      expect(validation.issues).toHaveLength(1);
      expect(validation.issues[0]).toContain("TDG score");
    });

    it("detects NaN values", () => {
      const nanScore: PmatScore = {
        productivity: NaN,
        maintainability: 0.8,
        accessibility: 0.7,
        testability: 0.9,
        tdg: 0.8,
      };

      const validation = scorer.validateScore(nanScore);
      expect(validation.valid).toBe(false);
      expect(validation.issues).toContain("Score contains NaN values");
    });
  });

  describe("Quality Grading", () => {
    it("assigns correct grades based on TDG scores", () => {
      expect(scorer.getQualityGrade(0.95)).toBe("Excellent");
      expect(scorer.getQualityGrade(0.90)).toBe("Excellent");
      expect(scorer.getQualityGrade(0.85)).toBe("Good");
      expect(scorer.getQualityGrade(0.70)).toBe("Good");
      expect(scorer.getQualityGrade(0.65)).toBe("Acceptable");
      expect(scorer.getQualityGrade(0.50)).toBe("Acceptable");
      expect(scorer.getQualityGrade(0.45)).toBe("Needs Improvement");
      expect(scorer.getQualityGrade(0.10)).toBe("Needs Improvement");
    });
  });

  describe("Mathematical Functions", () => {
    it("sigmoid function behaves correctly", () => {
      const sigmoid = (value: number, target: number, steepness = 5) => {
        const x = value / target;
        return 1 / (1 + Math.exp(-steepness * (x - 0.5)));
      };

      // Test sigmoid properties
      expect(sigmoid(0, 10)).toBeLessThan(0.5);
      expect(sigmoid(5, 10)).toBeCloseTo(0.5, 1);
      expect(sigmoid(10, 10)).toBeGreaterThan(0.5);
      expect(sigmoid(20, 10)).toBeGreaterThan(0.9);
    });

    it("exponential decay function behaves correctly", () => {
      const exponentialDecay = (actual: number, target: number, rate = 0.1) => {
        if (actual <= target) {
          return 1.0;
        }
        return Math.exp(-rate * (actual - target) / target);
      };

      // Test decay properties
      expect(exponentialDecay(5, 10)).toBe(1.0); // Under target
      expect(exponentialDecay(10, 10)).toBe(1.0); // At target
      expect(exponentialDecay(15, 10)).toBeLessThan(1.0); // Over target
      expect(exponentialDecay(20, 10)).toBeLessThan(exponentialDecay(15, 10)); // Further decay
    });
  });

  describe("Performance Targets", () => {
    it("meets transpilation speed targets", () => {
      const fastMetrics: QualityMetrics = {
        transpile_time_ms: 25, // Under 50ms target
        parse_time_ms: 5,
        ast_nodes: 10,
        complexity_score: 1,
        memory_usage_mb: 10,
        energy_reduction: {
          joules: 0.001,
          wattsAverage: 1.0,
          co2Grams: 0.000475,
          confidence: 0.8,
        },
      };

      const score = scorer.calculatePmatScore(fastMetrics);
      expect(score.productivity).toBeGreaterThan(0.7);

      const slowMetrics: QualityMetrics = {
        ...fastMetrics,
        transpile_time_ms: 100, // Over 50ms target
      };

      const slowScore = scorer.calculatePmatScore(slowMetrics);
      expect(slowScore.productivity).toBeLessThan(score.productivity);
    });

    it("penalizes high complexity appropriately", () => {
      const simpleMetrics: QualityMetrics = {
        transpile_time_ms: 30,
        parse_time_ms: 5,
        ast_nodes: 10,
        complexity_score: 2, // Low complexity
        memory_usage_mb: 10,
        energy_reduction: {
          joules: 0.001,
          wattsAverage: 1.0,
          co2Grams: 0.000475,
          confidence: 0.8,
        },
      };

      const complexMetrics: QualityMetrics = {
        ...simpleMetrics,
        complexity_score: 12, // High complexity
      };

      const simpleScore = scorer.calculatePmatScore(simpleMetrics);
      const complexScore = scorer.calculatePmatScore(complexMetrics);

      expect(complexScore.maintainability).toBeLessThan(simpleScore.maintainability);
      expect(complexScore.tdg).toBeLessThan(simpleScore.tdg);
    });
  });

  describe("Scoring Consistency", () => {
    it("produces consistent scores for identical metrics", () => {
      const metrics: QualityMetrics = {
        transpile_time_ms: 30,
        parse_time_ms: 5,
        ast_nodes: 15,
        complexity_score: 3,
        memory_usage_mb: 12,
        energy_reduction: {
          joules: 0.002,
          wattsAverage: 2.0,
          co2Grams: 0.00095,
          confidence: 0.85,
        },
      };

      const score1 = scorer.calculatePmatScore(metrics);
      const score2 = scorer.calculatePmatScore(metrics);

      expect(score1.productivity).toEqual(score2.productivity);
      expect(score1.maintainability).toEqual(score2.maintainability);
      expect(score1.accessibility).toEqual(score2.accessibility);
      expect(score1.testability).toEqual(score2.testability);
      expect(score1.tdg).toEqual(score2.tdg);
    });

    it("shows monotonic behavior for key metrics", () => {
      const baseMetrics: QualityMetrics = {
        transpile_time_ms: 50,
        parse_time_ms: 10,
        ast_nodes: 20,
        complexity_score: 5,
        memory_usage_mb: 25,
        energy_reduction: {
          joules: 0.003,
          wattsAverage: 3.0,
          co2Grams: 0.001425,
          confidence: 0.7,
        },
      };

      // Improving transpilation time should improve productivity
      const fasterMetrics = { ...baseMetrics, transpile_time_ms: 25 };
      const baseScore = scorer.calculatePmatScore(baseMetrics);
      const fasterScore = scorer.calculatePmatScore(fasterMetrics);

      expect(fasterScore.productivity).toBeGreaterThan(baseScore.productivity);
      expect(fasterScore.tdg).toBeGreaterThan(baseScore.tdg);
    });
  });
});
