# 🧠 Desafio 01 — Transposição de Matriz 3x3 em Rust

Este desafio consiste em implementar uma função em Rust que recebe uma matriz 3x3 e retorna sua transposta.

---

## 🎯 Objetivo

Dada uma matriz quadrada de 3x3, a proposta é construir a função:

```rust
fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3]
```

Essa função deve retornar a **transposta** da matriz, ou seja, trocar as **linhas pelas colunas**.

---

## Exemplo

### Entrada:
```rust
[
 [101, 102, 103],
 [201, 202, 203],
 [301, 302, 303],
]
```

### Saída esperada:
```rust
[
 [101, 201, 301],
 [102, 202, 302],
 [103, 203, 303],
]
```

---

## Execução

```bash
cargo run     # Executa a transposição e imprime no terminal
cargo test    # Valida a função com teste automatizado
```

---

📁 Desafio resolvido como parte de uma série de exercícios práticos em Rust.
