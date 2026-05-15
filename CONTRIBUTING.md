# Contributing to RepoFlow

## Fork-and-Pull Git Workflow

### 1. Fork the Repository

Click the "Fork" button on the repository page. This creates your own copy of the repository under your GitHub account.

### 2. Clone Your Fork

```bash
git clone https://github.com/YOUR_USERNAME/repoflow-contract.git
cd repoflow-contract
git remote add upstream https://github.com/repoflow-labs/repoflow-contract.git
```

### 3. Create a Feature Branch

```bash
git fetch upstream
git checkout -b feature/your-feature-name
# Or: fix/bug-description, docs/description, chore/task-description
```

### 4. Make Changes

Implement your feature or bug fix. Ensure you:
- Write tests for new functionality
- Follow existing code style and conventions
- Keep commits atomic and focused

### 5. Keep Your Branch Updated

```bash
git fetch upstream
git rebase upstream/main
# Resolve any conflicts if they arise
```

### 6. Push and Create PR

```bash
git push origin feature/your-feature-name
# Create PR via GitHub UI
```

### 7. Address Review Feedback

Make additional commits if needed. Use `git push --force-with-lease` to update your PR branch.

### 8. PR Merge

Once approved and CI passes, maintainers will merge your PR.

## Local Commit Message Conventions

### Format

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

| Type | Description |
|------|-------------|
| `feat` | New feature for the user |
| `fix` | Bug fix for the user |
| `docs` | Documentation changes only |
| `style` | Code style change (formatting, no logic) |
| `refactor` | Code change that neither fixes nor adds |
| `perf` | Code change that improves performance |
| `test` | Adding or correcting tests |
| `chore` | Build process, dependencies, tooling |
| `revert` | Reverting a previous commit |

### Scope

Use the affected subsystem:
- `contract` - Soroban smart contract
- `backend` - Rust Axum backend
- `frontend` - Next.js/React frontend
- `indexer` - Event indexer service
- `ci` - GitHub Actions workflows

### Subject Rules

- Use imperative mood: "add" not "added" or "adds"
- No trailing period
- Max 50 characters
- Lowercase first letter

### Examples

```
feat(contract): add set_dependency_split function with 50 dep limit

fix(backend): resolve nonce expiration race condition in verification worker

docs(readme): update architecture diagram with indexer component

perf(frontend): optimize D3 force graph rendering for 100+ nodes

test(contract): add test for circular dependency detection in split logic
```

### Body

- Wrap at 72 characters
- Explain *what* and *why*, not *how*
- Include motivation and context

### Footer

- Reference issues: `Closes #123`, `Fixes #456`
- Breaking changes: `BREAKING CHANGE: description`

## PR Submission Requirements

### Test Coverage Rules

| Component | Minimum Coverage |
|-----------|------------------|
| Smart Contract | 90% line coverage |
| Backend | 80% line coverage |
| Frontend | 70% line coverage |

Run coverage before submitting:

```bash
# Contract
cargo tarpaulin --manifest-path contracts/repoflow/Cargo.toml --output html

# Backend
cd backend && cargo tarpaulin --output html

# Frontend
cd frontend && npm run test -- --coverage
```

### CI Validation Checklist

Before submitting your PR, ensure:

- [ ] `cargo build --workspace` succeeds
- [ ] `cargo test --workspace` passes
- [ ] `cargo fmt --check` passes (run `cargo fmt` to fix)
- [ ] `cargo clippy -- -D warnings` passes (address all warnings)
- [ ] `npm run lint` passes (frontend)
- [ ] `npm run typecheck` passes (frontend)
- [ ] New tests added for changed functionality
- [ ] Documentation updated for API changes
- [ ] Commit messages follow conventions
- [ ] PR description links related issues

### PR Description Template

```markdown
## Summary
Brief description of changes.

## Type of Change
- [ ] Feature
- [ ] Bug Fix
- [ ] Refactor
- [ ] Documentation
- [ ] Tests

## Testing
Describe testing performed.

## Checklist
- [ ] Tests pass locally
- [ ] Code follows style guidelines
- [ ] Documentation updated
- [ ] Related issues linked
```

### Review Process

1. **Automated Checks** — CI runs build, tests, linting
2. **Code Review** — Maintainers review for correctness, security, performance
3. **Security Review** — Critical changes require security audit
4. **Approval** — At least one maintainer approval required

### Code Style

**Rust:**
- Follow `rustfmt` defaults
- Use `clippy` recommendations
- Prefer explicit error handling over `unwrap()`
- Document public APIs with doc comments

**TypeScript/React:**
- Follow ESLint configuration
- Use functional components with hooks
- Prefer explicit types over `any`
- Use Tailwind CSS for styling

## Development Environment

### Required Tools

- Rust 1.77+
- Node.js 20 LTS
- Docker
- PostgreSQL 15+
- Redis 7+

### Running Tests

```bash
# All tests
make test

# Contract only
cargo test --manifest-path contracts/repoflow/Cargo.toml

# Backend only
cd backend && cargo test

# Frontend only
cd frontend && npm test
```

### Local Development

```bash
# Start all services
make dev

# Contract development
make contract-build
make contract-test

# Backend development
make backend-up

# Frontend development
make frontend-dev
```

## Communication

- **Issues** — Use GitHub Issues for bugs and feature requests
- **Discussions** — Use GitHub Discussions for Q&A
- **Discord** — Join the Drips Network Discord for real-time help

## Recognition

Contributors are recognized in the README and project's CONTRIBUTORS file.