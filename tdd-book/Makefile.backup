# Makefile for Depyler TDD Book

.PHONY: help test lint quality build serve clean install

help: ## Show this help message
	@echo "Depyler TDD Book - Makefile Commands"
	@echo "===================================="
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

install: ## Install dependencies
	pip install -r requirements.txt

test: ## Run all tests
	pytest tests/ -v --cov --cov-report=term-missing --cov-report=html

test-fast: ## Run tests in parallel
	pytest tests/ -n auto -v

test-module: ## Test specific module (usage: make test-module MODULE=os)
	pytest tests/test_$(MODULE)/ -v

quality: ## Run quality gates (requires pmat)
	@command -v pmat >/dev/null 2>&1 && { \
		pmat analyze complexity --fail-on-violation; \
		pmat analyze satd --fail-on-violation; \
		pmat quality-gate --strict; \
	} || echo "⚠️  pmat not installed, skipping quality gates"

lint: ## Run linters
	ruff check tests/ scripts/
	black --check tests/ scripts/
	@command -v mypy >/dev/null 2>&1 && mypy tests/ scripts/ || echo "⚠️  mypy not installed, skipping type checking"

format: ## Auto-format code
	black tests/ scripts/
	ruff check --fix tests/ scripts/

extract: ## Generate docs from tests
	@command -v python >/dev/null 2>&1 && python scripts/extract_examples.py --all || echo "⚠️  extract script not yet implemented"

validate: test lint ## Run all validation checks
	@echo "✅ All validation checks passed!"

clean: ## Clean build artifacts
	rm -rf reports/ .coverage htmlcov/ .pytest_cache/
	find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
	find . -type f -name "*.pyc" -delete

setup: install ## Initial project setup
	@echo "🚀 Setting up Depyler TDD Book..."
	mkdir -p tests/ docs/modules docs/edge-cases reports/
	@echo "✅ Setup complete!"

.DEFAULT_GOAL := help
