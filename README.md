# Projeto de Teste para TLQ (Tiny Little Queue) - Vendas e NF-e

Este projeto contém testes para o TLQ (Tiny Little Queue), um sistema de filas de mensagens minimalista construído em Rust, adaptado para trabalhar com mensagens de vendas e geração de Nota Fiscal Eletrônica em formato XML.

## Estrutura do Projeto

- [src/main.rs](file:///home/cads/projects/rust/tlq_test_project/src/main.rs): Aplicação de demonstração que mostra como usar o TLQ com vendas e NF-e
- [src/lib.rs](file:///home/cads/projects/rust/tlq_test_project/src/lib.rs): Biblioteca com utilitários para testar o TLQ
- [src/tests.rs](file:///home/cads/projects/rust/tlq_test_project/src/tests.rs): Testes unitários (executados automaticamente)
- [src/integration_tests.rs](file:///home/cads/projects/rust/tlq_test_project/src/integration_tests.rs): Testes de integração (executados manualmente)

## Requisitos

- Rust (cargo)
- TLQ servidor em execução na porta 1337

## Instalação

```bash
# Instalar o TLQ
cargo install tlq

# Iniciar o servidor TLQ
tlq &

# Navegar até o diretório do projeto
cd tlq_test_project
```

## Execução

### Executar a aplicação de demonstração

```bash
cargo run
```

### Executar testes unitários

```bash
cargo test
```

### Executar testes de integração

```bash
cargo test --features integration-tests
```

## Funcionalidades Testadas

1. **Adição de mensagens de venda**: Verifica se é possível adicionar mensagens de venda à fila
2. **Recuperação de mensagens de venda**: Verifica se é possível recuperar mensagens de venda da fila
3. **Processamento de vendas**: Verifica se é possível processar e deletar mensagens de venda
4. **Geração de XML NF-e**: Converte estruturas de venda em formato XML de Nota Fiscal Eletrônica
5. **Múltiplas vendas**: Verifica o processamento de múltiplas vendas em sequência
6. **Limpeza de fila**: Verifica se é possível limpar completamente a fila

## Estrutura das Mensagens de Venda

```rust
struct Venda {
    id: u32,
    cliente_id: u32,
    valor_total: f64,
    data: String,
    itens: Vec<ItemVenda>,
}

struct ItemVenda {
    produto_id: u32,
    quantidade: u32,
    preco_unitario: f64,
    subtotal: f64,
}
```

## Geração de XML NF-e

O projeto converte estruturas de venda no seguinte formato XML:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<nfe>
  <id>1001</id>
  <cliente_id>501</cliente_id>
  <valor_total>150.75</valor_total>
  <data>2025-09-14</data>
  <itens>
    <item>
      <produto_id>101</produto_id>
      <quantidade>2</quantidade>
      <preco_unitario>25.50</preco_unitario>
      <subtotal>51.00</subtotal>
    </item>
  </itens>
</nfe>
```

## Cliente TLQ

O projeto utiliza o cliente oficial `tlq-client` para se comunicar com o servidor TLQ.