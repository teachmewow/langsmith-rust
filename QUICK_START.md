# Quick Start: Usando a Crate no Seu Projeto

## Passo a Passo Rápido

### 1. Adicionar Dependência

No `Cargo.toml` do seu projeto (ex: `teachmewowagent`):

```toml
[dependencies]
langsmith-rust = { path = "../langsmith-rust" }
```

### 2. Configurar Variáveis de Ambiente

Crie um arquivo `.env` no seu projeto:

```bash
LANGSMITH_TRACING=true
LANGSMITH_ENDPOINT=https://api.smith.langchain.com
LANGSMITH_API_KEY=<sua-api-key>
LANGSMITH_PROJECT=<nome-do-projeto>
```

### 3. Inicializar no Código

```rust
use langsmith_rust;

fn main() {
    // Inicializa (carrega .env automaticamente)
    langsmith_rust::init();
    
    // Seu código aqui...
}
```

### 4. Usar Tracing

#### Opção A: Automático (Recomendado)

```rust
use langsmith_rust::{trace_node, RunType};

async fn minha_funcao(input: String) -> langsmith_rust::Result<String> {
    // Sua lógica aqui
    Ok(format!("Processado: {}", input))
}

// Usar com tracing automático
let resultado = trace_node(
    "minha_funcao",
    RunType::Runnable,
    "entrada".to_string(),
    minha_funcao
).await?;
```

#### Opção B: Manual

```rust
use langsmith_rust::{Tracer, RunType};
use serde_json::json;

let mut tracer = Tracer::new("meu_node", RunType::Chain, json!({"input": "..."}));
tracer.post().await?;

// ... executar sua função ...

tracer.end(json!({"output": "..."}));
tracer.patch().await?;
```

## Exemplo Completo

```rust
use langsmith_rust::{trace_node, RunType, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Inicializar
    langsmith_rust::init();
    
    // 2. Definir função do node
    async fn llm_node(messages: Vec<String>) -> Result<String> {
        // Simular chamada LLM
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        Ok(format!("Resposta para: {}", messages.join(", ")))
    }
    
    // 3. Executar com tracing
    let messages = vec!["Olá".to_string(), "Como vai?".to_string()];
    let resultado = trace_node(
        "llm_node",
        RunType::Llm,
        messages,
        llm_node
    ).await?;
    
    println!("Resultado: {}", resultado);
    Ok(())
}
```

## Estrutura de Diretórios Recomendada

```
teachmewow/
├── langsmith-rust/          # Esta crate
│   ├── Cargo.toml
│   └── src/
│
└── teachmewowagent/         # Seu projeto
    ├── Cargo.toml           # Adicione: langsmith-rust = { path = "../langsmith-rust" }
    ├── .env                 # Configure variáveis aqui
    └── src/
        └── main.rs          # Use a crate aqui
```

## Verificar se Funcionou

1. Execute seu projeto: `cargo run`
2. Verifique logs no stderr (erros de tracing aparecem lá)
3. Acesse LangSmith dashboard para ver os traces

## Troubleshooting

**Erro: "cannot find crate langsmith-rust"**
- Verifique o caminho no `Cargo.toml`
- Execute `cargo clean && cargo build`

**Tracing não aparece no LangSmith**
- Verifique `LANGSMITH_TRACING=true` no `.env`
- Verifique `LANGSMITH_API_KEY` está correto
- Veja erros no stderr

**Erro de compilação**
- Certifique-se que todas as dependências estão instaladas
- Execute `cargo update`

