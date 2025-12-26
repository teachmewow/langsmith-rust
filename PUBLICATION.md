# Como Usar a Crate em Outro Projeto

## Opção 1: Via Git (Recomendado)

Adicione no `Cargo.toml` do seu outro projeto:

```toml
[dependencies]
langsmith-rust = { git = "https://github.com/teachmewow/langsmith-rust", branch = "main" }
```

Ou use uma versão específica:

```toml
[dependencies]
langsmith-rust = { git = "https://github.com/teachmewow/langsmith-rust", tag = "v0.1.0" }
```

## Opção 2: Via Path (Desenvolvimento Local)

Se os projetos estão no mesmo workspace:

```toml
[dependencies]
langsmith-rust = { path = "../langsmith-rust" }
```

## Opção 3: Publicar no crates.io (Futuro)

Para publicar no crates.io:

1. Criar conta no https://crates.io
2. Obter API token
3. Executar: `cargo publish`

Depois disso, usar:

```toml
[dependencies]
langsmith-rust = "0.1.0"
```

## Uso Básico

```rust
use langsmith_rust;

// Inicializar
langsmith_rust::init();

// Usar tracing
use langsmith_rust::{trace_node, RunType};

let result = trace_node(
    "my_node",
    RunType::Llm,
    input_data,
    |input| async move {
        // sua lógica aqui
        process(input).await
    }
).await?;
```

## Configuração

Crie um arquivo `.env` no seu projeto:

```bash
LANGSMITH_TRACING=true
LANGSMITH_ENDPOINT=https://api.smith.langchain.com
LANGSMITH_API_KEY=<sua-api-key>
LANGSMITH_PROJECT=<nome-do-projeto>
```

