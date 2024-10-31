# Style

## Rust Code

We create two distinct impl blocks for any rust struct — one for internal rust methods and one for python-facing functions.

This will be denoted by the inclusion/exclusion of the `#[pymethods]` decorator
