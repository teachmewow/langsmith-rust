# Guia de Publicação da Crate

## Opções para Usar a Crate

### Opção 1: Dependência Local (Path) - ⭐ Recomendado para Desenvolvimento

Para usar a crate localmente no projeto `teachmewowagent`:

**No `Cargo.toml` do `teachmewowagent`:**

```toml
[dependencies]
langsmith-rust = { path = "../langsmith-rust" }
```

**Vantagens:**
- ✅ Mudanças são refletidas imediatamente
- ✅ Não precisa publicar
- ✅ Ideal para desenvolvimento

**Desvantagens:**
- ❌ Só funciona localmente
- ❌ Não funciona em CI/CD sem ajustes

---

### Opção 2: Dependência Git - Para Repositório Remoto

Se você tem a crate em um repositório Git:

**No `Cargo.toml` do `teachmewowagent`:**

```toml
[dependencies]
langsmith-rust = { git = "https://github.com/seu-usuario/langsmith-rust", branch = "main" }
# ou para uma versão específica:
# langsmith-rust = { git = "https://github.com/seu-usuario/langsmith-rust", tag = "v0.1.0" }
```

**Vantagens:**
- ✅ Funciona em qualquer máquina com acesso ao repo
- ✅ Funciona em CI/CD
- ✅ Versionamento via tags

**Desvantagens:**
- ❌ Precisa de repositório Git público ou acesso configurado

---

### Opção 3: Publicar no crates.io - Para Uso Público

Para publicar no registro oficial do Rust:

#### Pré-requisitos

1. **Conta no crates.io**: https://crates.io
2. **API Token**: Gere em https://crates.io/me
3. **Configurar cargo**:

```bash
cargo login <seu-api-token>
```

#### Preparar Cargo.toml

Atualize o `Cargo.toml` com informações completas:

```toml
[package]
name = "langsmith-rust"
version = "0.1.0"
edition = "2021"
description = "Rust crate for manual tracing to LangSmith"
license = "MIT"
authors = ["Seu Nome <seu-email@example.com>"]
repository = "https://github.com/seu-usuario/langsmith-rust"
homepage = "https://github.com/seu-usuario/langsmith-rust"
documentation = "https://docs.rs/langsmith-rust"
keywords = ["langsmith", "tracing", "observability", "langchain"]
categories = ["development-tools", "web-programming"]

[dependencies]
# ... suas dependências
```

#### Publicar

```bash
# Verificar antes de publicar
cargo publish --dry-run

# Publicar
cargo publish
```

**Depois de publicar, usar no projeto:**

```toml
[dependencies]
langsmith-rust = "0.1.0"
```

**Vantagens:**
- ✅ Disponível publicamente
- ✅ Versionamento automático
- ✅ Fácil de usar em qualquer projeto

**Desvantagens:**
- ❌ Nome deve ser único no crates.io
- ❌ Versões publicadas não podem ser deletadas
- ❌ Precisa seguir convenções do crates.io

---

## Recomendação para Seu Caso

Para desenvolvimento local, use **Opção 1 (Path)**:

1. No projeto `teachmewowagent`, adicione ao `Cargo.toml`:

```toml
[dependencies]
langsmith-rust = { path = "../langsmith-rust" }
```

2. Use normalmente:

```rust
use langsmith_rust::{Tracer, RunType, trace_node};
```

3. Quando quiser compartilhar ou usar em produção, publique no Git ou crates.io.
