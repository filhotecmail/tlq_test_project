#[cfg(test)]
mod tests {
    use crate::{Venda, ItemVenda};
    
    #[tokio::test]
    async fn test_venda_structure() {
        let venda = Venda {
            id: 1001,
            cliente_id: 501,
            valor_total: 150.75,
            data: "2025-09-14".to_string(),
            itens: vec![
                ItemVenda {
                    produto_id: 101,
                    quantidade: 2,
                    preco_unitario: 25.50,
                    subtotal: 51.00,
                }
            ],
        };
        
        let json = serde_json::to_string(&venda).expect("Falha ao serializar venda");
        let parsed_venda: Venda = serde_json::from_str(&json).expect("Falha ao desserializar venda");
        
        assert_eq!(venda.id, parsed_venda.id);
        assert_eq!(venda.cliente_id, parsed_venda.cliente_id);
        assert_eq!(venda.valor_total, parsed_venda.valor_total);
        assert_eq!(venda.itens.len(), parsed_venda.itens.len());
        
        let item_original = &venda.itens[0];
        let item_parsed = &parsed_venda.itens[0];
        assert_eq!(item_original.produto_id, item_parsed.produto_id);
        assert_eq!(item_original.quantidade, item_parsed.quantidade);
        assert_eq!(item_original.preco_unitario, item_parsed.preco_unitario);
        assert_eq!(item_original.subtotal, item_parsed.subtotal);
    }
}