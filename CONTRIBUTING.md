# Contributing

## Conventions

### Import Spacing
When using imports, break up the spacing so that all imports from crates or libraries are together at the top, and then the `crate` and `super` imports are below, with one line break. An example:
```rs
use std::{fmt::Display, sync::Arc};
use num_complex::Complex64;

use crate::var::Var;
use super::{ArcExpr, Expr};
```
All of the "normal" imports are separated from the "special imports" (`crate`, `super`). Otherwise, there are no more linebreaks between the imports.
