# algonds
Command line application for offline practice of competitive programming tasks. All contributions are welcome!

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
  cargo run -- --db=./src/data/db.yaml
```

## How to add new problems
This app is still in early stages of it's development and there aren't many interesting problems for now. You can add new problem by:
  1. Adding new problem and test cases in `src/data/db.yaml`
  2. Adding stress tests in `src/data.rs` (see function `generate_stress_tests_for`)

