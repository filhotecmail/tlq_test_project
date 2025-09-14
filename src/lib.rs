use tlq_client::TlqClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Venda {
    pub id: u32,
    pub cliente_id: u32,
    pub valor_total: f64,
    pub data: String,
    pub itens: Vec<ItemVenda>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemVenda {
    pub produto_id: u32,
    pub quantidade: u32,
    pub preco_unitario: f64,
    pub subtotal: f64,
}

pub struct TlqTester {
    client: TlqClient,
}

impl TlqTester {
    pub fn new(host: &str, port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let client = TlqClient::new(host, port)?;
        Ok(TlqTester { client })
    }

    pub async fn add_venda_message(&self, venda: &Venda) -> Result<tlq_client::Message, Box<dyn std::error::Error>> {
        let venda_json = serde_json::to_string(venda)?;
        let message = self.client.add_message(&venda_json).await?;
        Ok(message)
    }

    pub async fn get_and_process_vendas(&self, count: u32) -> Result<Vec<Venda>, Box<dyn std::error::Error>> {
        let mut processed_vendas = Vec::new();
        let mut retrieved_count = 0;
        
        while retrieved_count < count {
            let messages = self.client.get_messages(10).await?;
            if messages.is_empty() {
                break;
            }
            
            for msg in messages {
                retrieved_count += 1;
                match serde_json::from_str::<Venda>(&msg.body) {
                    Ok(venda) => {
                        processed_vendas.push(venda);
                        self.client.delete_message(msg.id).await?;
                    }
                    Err(e) => {
                        eprintln!("Erro ao parsear venda {}: {}", msg.id, e);
                        self.client.delete_message(msg.id).await?;
                    }
                }
                
                if retrieved_count >= count {
                    break;
                }
            }
        }
        
        Ok(processed_vendas)
    }
    
    pub async fn clear_queue(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let messages = self.client.get_messages(10).await?;
            if messages.is_empty() {
                break;
            }
            
            for msg in messages {
                self.client.delete_message(msg.id).await?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;