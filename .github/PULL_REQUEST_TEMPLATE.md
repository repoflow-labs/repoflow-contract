## Summary

<!-- Provide a brief description of what this PR changes and why -->

## Type of Change

- [ ] **Feature** — New functionality added
- [ ] **Bug Fix** — Existing functionality corrected
- [ ] **Refactor** — Code restructuring without behavior change
- [ ] **Documentation** — Documentation updates only
- [ ] **Tests** — Test additions or corrections

## Breaking Change

- [ ] **Yes** — This PR introduces a breaking change
- [ ] **No** — This PR does not introduce a breaking change

If yes, describe the breaking change:

## Related Issues

<!-- Link related issues using keywords: Closes, Fixes, Resolves, Addresses -->

- Closes #
- Fixes #
- Related to #

## Testing Documentation

### Test Plan

<!-- Describe the testing approach -->

### Unit Tests Added/Modified

<!-- List unit tests added or modified -->

### Integration Tests Added/Modified

<!-- List integration tests added or modified -->

### Manual Testing Steps

<!-- Steps to manually verify the change -->

| Step | Action | Expected Result |
|------|--------|-----------------|
| 1    |        |                 |
| 2    |        |                 |
| 3    |        |                 |

## Code Review Checklist

### General

- [ ] Code follows project style guidelines
- [ ] No debug code or commented-out functionality
- [ ] No security vulnerabilities introduced
- [ ] No performance regressions introduced
- [ ] No unnecessary dependencies added

### Smart Contract (if applicable)

- [ ] Proper error handling with meaningful error messages
- [ ] Integer overflow/underflow handled
- [ ] Access control properly enforced
- [ ] Storage usage optimized (key size, data structure)
- [ ] Unit tests cover critical paths
- [ ] Edge cases handled (zero values, boundary conditions)

### Backend (if applicable)

- [ ] Database queries optimized (index usage, query plans)
- [ ] API endpoints follow REST conventions
- [ ] Authentication/authorization properly implemented
- [ ] Rate limiting in place where needed
- [ ] Proper logging and monitoring
- [ ] Error responses consistent and informative

### Frontend (if applicable)

- [ ] Proper TypeScript usage (no `any` where possible)
- [ ] Accessible (ARIA labels, keyboard navigation)
- [ ] Responsive design works on mobile/desktop
- [ ] No memory leaks (event listeners, subscriptions)
- [ ] Loading and error states handled

### Documentation

- [ ] Code comments added for complex logic
- [ ] API documentation updated if applicable
- [ ] README updated if needed
- [ ] Changelog entry added (if required)

### CI/CD

- [ ] GitHub Actions workflow changes tested
- [ ] Build pipeline passes
- [ ] All tests pass
- [ ] Linting passes
- [ ] Type checking passes

## Screenshots (if UI changes)

<!-- Add screenshots or GIFs of UI changes -->

## Additional Context

<!-- Any other relevant context for reviewers -->

## Reviewers Requested

<!-- Tag specific reviewers if needed -->

@reviewer1 @reviewer2

## Self-Review Before Submitting

- [ ] I have run all tests locally and they pass
- [ ] I have run linting and fixed all issues
- [ ] I have run type checking and fixed all type errors
- [ ] I have verified the build succeeds
- [ ] I have checked my branch is up-to-date with main
- [ ] I have removed any debug code or commented-out sections
- [ ] I have added tests for new functionality
- [ ] I have updated documentation for API changes