# 🦀 JSON Parser Manual em Rust

Um parser de JSON **manual**, feito **do zero em Rust**, sem uso de `String`, `.clone()` ou bibliotecas externas — totalmente baseado em `&str` e **lifetimes**, com suporte a navegação por caminhos no estilo `"chave.subchave.0.valor"`.

---

## 💡 Sobre o desafio

O desafio consistia em:

- Criar um **parser completo de JSON** sem usar `serde_json`, `json`, ou qualquer crate auxiliar.
- Modelar os dados como uma **estrutura recursiva em tempo real** (um grafo, não uma AST formal).
- Acessar qualquer valor interno usando uma string como caminho:
  ```
  json.query("interesses.0.topicos.0.idk")
  ```
- Garantir:
  - 100% do código usando `&str`
  - Sem `.clone()`
  - Sem alocações desnecessárias (`String`, `Box`, etc.)

---

## ✨ O que foi implementado

- ✅ Parser completo de `String`, `Number`, `Boolean`, `Null`, `Array`, `Object`
- ✅ Suporte a objetos e arrays aninhados recursivamente
- ✅ Enum `JsonValue` modelando a estrutura JSON como um **grafo**
- ✅ Impressão formatada (`pretty_print_json`)
- ✅ Método `.query("a.b.0.c")` para navegar qualquer valor via caminho
- ✅ Sem dependências externas
- ✅ 100% baseado em (`&str`) com lifetimes

---

## 📦 Como rodar o projeto

### 1. Clone o repositório
```bash
git clone https://github.com/mairinkdev/rust-challenges/tree/main/desafio02-parser-json
cd desafio02-parser-json
```

### 2. Compile e execute
```bash
cargo run
```

### 3. Rodar testes (opcional)
```bash
cargo test
```

---

## 😼 Exemplo de uso

```rust
if let Some(JsonValue::Boolean(idk)) = parsed.query("interesses.0.topicos.0.idk") {
    println!("IDK: {}", idk);
}
```

---

## 📂 Estrutura

```
src/
├── lib.rs      // parser, estrutura JSON, query system
└── main.rs     // exemplo de uso do parser e consulta de dados
```

---

## 🧠 Por que isso importa?

Esse projeto mostra:

- Proficiência em Rust com lifetimes e parsing manual
- Controle total sobre alocação e uso de memória
- Capacidade de modelar dados como grafos
- Design de API leve, intuitiva e extensível

---

## Autor

Feito por [@mairinkdev](https://github.com/mairinkdev)
Desafio proposto por [@YuriRDev](https://github.com/YuriRDev)
