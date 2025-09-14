// Testes de integração para o TLQ com Vendas e NF-e
// Estes testes devem ser executados manualmente quando o servidor TLQ estiver em execução

use crate::{Venda, ItemVenda, TlqTester};

#[tokio::test]
async fn test_single_venda() {
    let tester = TlqTester::new("localhost", 1337).expect("Falha ao criar cliente TLQ");
    
    // Limpar qualquer mensagem existente na fila
    tester.clear_queue().await.expect("Falha ao limpar a fila");
    
    // Criar uma venda de teste
    let venda = Venda {
        id: 2001,
        cliente_id: 601,
        valor_total: 250.50,
        data: "2025-09-14".to_string(),
        itens: vec![
            ItemVenda {
                produto_id: 201,
                quantidade: 1,
                preco_unitario: 100.25,
                subtotal: 100.25,
            },
            ItemVenda {
                produto_id: 202,
                quantidade: 2,
                preco_unitario: 75.12,
                subtotal: 150.25,
            }
        ],
    };
    
    // Adicionar venda à fila
    let _message = tester.add_venda_message(&venda).await.expect("Falha ao adicionar venda");
    
    // Recuperar e processar vendas
    let processed_vendas = tester.get_and_process_vendas(5).await.expect("Falha ao processar vendas");
    
    // Verificar se a venda foi processada corretamente
    assert!(!processed_vendas.is_empty(), "Nenhuma venda foi recuperada da fila");
    let processed_venda = &processed_vendas[0];
    assert_eq!(processed_venda.id, venda.id);
    assert_eq!(processed_venda.cliente_id, venda.cliente_id);
    assert_eq!(processed_venda.valor_total, venda.valor_total);
    assert_eq!(processed_venda.itens.len(), venda.itens.len());
}

#[tokio::test]
async fn test_multiple_vendas() {
    let tester = TlqTester::new("localhost", 1337).expect("Falha ao criar cliente TLQ");
    
    // Limpar qualquer mensagem existente na fila
    tester.clear_queue().await.expect("Falha ao limpar a fila");
    
    // Adicionar múltiplas vendas
    let vendas_to_add = vec![
        Venda {
            id: 3001,
            cliente_id: 701,
            valor_total: 120.00,
            data: "2025-09-14".to_string(),
            itens: vec![
                ItemVenda {
                    produto_id: 301,
                    quantidade: 3,
                    preco_unitario: 40.00,
                    subtotal: 120.00,
                }
            ],
        },
        Venda {
            id: 3002,
            cliente_id: 702,
            valor_total: 85.75,
            data: "2025-09-14".to_string(),
            itens: vec![
                ItemVenda {
                    produto_id: 302,
                    quantidade: 1,
                    preco_unitario: 85.75,
                    subtotal: 85.75,
                }
            ],
        },
        Venda {
            id: 3003,
            cliente_id: 703,
            valor_total: 210.30,
            data: "2025-09-14".to_string(),
            itens: vec![
                ItemVenda {
                    produto_id: 303,
                    quantidade: 2,
                    preco_unitario: 105.15,
                    subtotal: 210.30,
                }
            ],
        },
    ];
    
    for venda in &vendas_to_add {
        tester.add_venda_message(venda).await.expect("Falha ao adicionar venda");
    }
    
    // Recuperar e processar vendas
    let processed_vendas = tester.get_and_process_vendas(10).await.expect("Falha ao processar vendas");
    
    // Verificar se todas as vendas foram processadas
    assert_eq!(processed_vendas.len(), vendas_to_add.len(), 
              "Número de vendas processadas ({}) não corresponde ao número de vendas adicionadas ({})", 
              processed_vendas.len(), vendas_to_add.len());
    
    // Verificar conteúdo das vendas (a ordem pode variar)
    let mut found_vendas = vec![false; vendas_to_add.len()];
    for processed_venda in &processed_vendas {
        let index = vendas_to_add.iter().position(|venda| {
            venda.id == processed_venda.id && 
            venda.cliente_id == processed_venda.cliente_id && 
            venda.valor_total == processed_venda.valor_total
        });
        
        if let Some(i) = index {
            found_vendas[i] = true;
        }
    }
    
    // Verificar se todas as vendas foram encontradas
    for (i, found) in found_vendas.iter().enumerate() {
        assert!(*found, "Venda {} não foi encontrada na fila processada", i);
    }
}