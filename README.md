# Learning vanilla Rust

<https://doc.rust-lang.org/book/ch01-03-hello-cargo.html>

## Commandes

### Nouveau projet

```shell
cargo new project_name
```

### Compilation

```shell
# DEV
cargo build
# Output in ./target/debug

# PROD
cargo build --release
# Output in ./target/release
```

### Mise à jour des dépendances (en théorie)

```shell
cargo update
```

### Exécution

```shell
cargo run
```

### Test compilation

```shell
cargo check
```

### Générer la documentation locale

```shell
cargo doc --open
```

### Débugger à partir d'un code erreur

```shell
# … error[E0384]
rustc --explain E0384
```
