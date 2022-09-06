<div align="center">
  
# Algonds

Command line application for offline practice of competitive programming tasks with correctness and performance analysis. 

Every feedback/contribution is welcome ❤️

[![Rust](https://github.com/MaciejWas/algonds/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/MaciejWas/algonds/actions/workflows/rust.yml)
[![Clippy check](https://github.com/MaciejWas/algonds/actions/workflows/clippy_check.yaml/badge.svg?branch=main)](https://github.com/MaciejWas/algonds/actions/workflows/clippy_check.yaml)  
 
</div>


## Demo
<img src="./assets/demo1.gif">

## How to run
### For the first time
Assuming Rust is installed:
```
  git clone https://github.com/MaciejWas/algonds
  cd algonds
  cargo run -- help
```

### Offline mode
```
  cargo run -- --db=./src/data/db.yaml run
```

## How to add new problems
This app is still in early stages of it's development and there aren't many interesting problems for now. You can add new problem by:
  1. Adding new problem and test cases in `src/data/db.yaml`
  2. Adding stress tests in `src/data.rs` (see function `generate_stress_tests_for`)

