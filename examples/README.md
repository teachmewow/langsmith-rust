# Exemplos de Uso

## test_llm_tracing.rs

Exemplo completo que demonstra como fazer tracing de chamadas LLM para o LangSmith.

### Pré-requisitos

Certifique-se de ter configurado seu `.env` com:

```bash
LANGSMITH_TRACING=true
LANGSMITH_ENDPOINT=https://api.smith.langchain.com
LANGSMITH_API_KEY=<sua-api-key>
LANGSMITH_PROJECT=<nome-do-projeto>
OPENAI_API_KEY=<sua-openai-api-key>
```

### Como executar

```bash
# Opção 1: Usando cargo diretamente
cargo run --example test_llm_tracing

# Opção 2: Usando o script
./run_test.sh
```

### O que o exemplo faz

1. **Exemplo 1 - Tracing Manual:**
   - Cria um tracer principal (Chain)
   - Cria um child run para a chamada LLM
   - Faz chamada real para OpenAI com "Oi, tudo bem?"
   - Salva inputs e outputs no LangSmith

2. **Exemplo 2 - Tracing Automático:**
   - Usa a função helper `trace_node`
   - Faz chamada para OpenAI com "Como você está?"
   - O tracing é feito automaticamente

### Verificando os resultados

Após executar, acesse o dashboard do LangSmith:
- Vá para o projeto configurado em `LANGSMITH_PROJECT`
- Você verá os runs criados com:
  - Inputs: mensagem enviada
  - Outputs: resposta da LLM
  - Metadados: timestamps, duração, etc.

