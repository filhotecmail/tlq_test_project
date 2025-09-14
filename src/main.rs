use tlq_client::TlqClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Venda {
    id: u32,
    cliente_id: u32,
    valor_total: f64,
    data: String,
    itens: Vec<ItemVenda>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemVenda {
    produto_id: u32,
    quantidade: u32,
    preco_unitario: f64,
    subtotal: f64,
}

fn gerar_xml_nota_fiscal(venda: &Venda) -> String {
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<nfe>\n");
    xml.push_str(&format!("  <id>{}</id>\n", venda.id));
    xml.push_str(&format!("  <cliente_id>{}</cliente_id>\n", venda.cliente_id));
    xml.push_str(&format!("  <valor_total>{:.2}</valor_total>\n", venda.valor_total));
    xml.push_str(&format!("  <data>{}</data>\n", venda.data));
    xml.push_str("  <itens>\n");
    
    for item in &venda.itens {
        xml.push_str("    <item>\n");
        xml.push_str(&format!("      <produto_id>{}</produto_id>\n", item.produto_id));
        xml.push_str(&format!("      <quantidade>{}</quantidade>\n", item.quantidade));
        xml.push_str(&format!("      <preco_unitario>{:.2}</preco_unitario>\n", item.preco_unitario));
        xml.push_str(&format!("      <subtotal>{:.2}</subtotal>\n", item.subtotal));
        xml.push_str("    </item>\n");
    }
    
    xml.push_str("  </itens>\n");
    xml.push_str("</nfe>");
    xml
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testando TLQ com mensagens de Vendas e Nota Fiscal Eletrônica");
    
    let client = TlqClient::new("localhost", 1337)?;
    println!("Cliente TLQ criado com sucesso");
    
    // Criar vendas de exemplo
    let venda1 = Venda {
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
            },
            ItemVenda {
                produto_id: 102,
                quantidade: 1,
                preco_unitario: 99.75,
                subtotal: 99.75,
            }
        ],
    };
    
    let venda2 = Venda {
        id: 1002,
        cliente_id: 502,
        valor_total: 89.90,
        data: "2025-09-14".to_string(),
        itens: vec![
            ItemVenda {
                produto_id: 103,
                quantidade: 3,
                preco_unitario: 29.97,
                subtotal: 89.90,
            }
        ],
    };
    
    // Converter vendas para XML
    let xml_venda1 = gerar_xml_nota_fiscal(&venda1);
    let xml_venda2 = gerar_xml_nota_fiscal(&venda2);
    
    println!("XML da Venda 1:\n{}\n", xml_venda1);
    println!("XML da Venda 2:\n{}\n", xml_venda2);
    
    // Adicionar mensagens à fila (em formato JSON para facilitar o processamento)
    let venda1_json = serde_json::to_string(&venda1)?;
    let venda2_json = serde_json::to_string(&venda2)?;
    
    let message1 = client.add_message(&venda1_json).await?;
    let message2 = client.add_message(&venda2_json).await?;
    
    println!("Venda 1 adicionada com ID: {}", message1.id);
    println!("Venda 2 adicionada com ID: {}", message2.id);
    
    // Recuperar mensagens da fila
    let messages = client.get_messages(5).await?;
    println!("Recuperadas {} mensagens da fila", messages.len());
    
    for msg in messages {
        match serde_json::from_str::<Venda>(&msg.body) {
            Ok(venda) => {
                println!("Venda recebida - ID: {}, Cliente: {}, Valor: R$ {:.2}", 
                         venda.id, venda.cliente_id, venda.valor_total);
                
                // Gerar XML da nota fiscal
                let xml_nfe = gerar_xml_nota_fiscal(&venda);
                println!("XML gerado para a venda {}:\n{}\n", venda.id, xml_nfe);
                
                // Deletar a mensagem após processamento
                client.delete_message(msg.id).await?;
                println!("Venda {} deletada com sucesso\n", venda.id);
            }
            Err(e) => {
                println!("Erro ao parsear venda: {}", e);
                client.delete_message(msg.id).await?;
            }
        }
    }
    
    println!("Teste concluído com sucesso!");
    Ok(())
}