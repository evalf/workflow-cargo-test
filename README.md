Reusable workflow for running cargo test with coverage
======================================================

Usage:

```yaml
jobs:
  test:
    uses: evalf/workflow-cargo-test/.github/workflows/test.yaml@release/1
    with:
      # Test matrix as JSON. Used keys are `rust-toolchain` and cargo `features`. (optional)
      matrix: '{"rust-toolchain": ["stable", "beta"], "features": [""]}'
      # Check code formatting. (optional)
      check-fmt: true
      # Name of the coverage artifact. (optional)
      artifact: 'coverage'
```
