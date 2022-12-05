use std::collections::HashMap;
use std::thread;
use std::time::Duration;

const MAX_REQUESTS_PER_IP: usize = 10;
const BLOCK_DURATION_SECONDS: u64 = 60;

fn main() {
    // Monitora o tráfego da rede e contabiliza as solicitações de cada fonte IP.
    let mut requests_per_ip = HashMap::new();
    loop {
        // Recebe uma solicitação e obtém o endereço IP da fonte.
        let source_ip = receive_request();

        // Incrementa o contador de solicitações para esta fonte IP.
        *requests_per_ip.entry(source_ip).or_insert(0) += 1;

        if requests_per_ip[&source_ip] > MAX_REQUESTS_PER_IP {
            // A fonte IP está emprestando um ataque DDoS, então a bloqueia.
            block_ip(source_ip);

            // Inicia um temporizador para desbloquear a fonte IP após um tempo especificado.
            thread::spawn(move || {
                thread::sleep(Duration::from_secs(BLOCK_DURATION_SECONDS));
                unblock_ip(source_ip);
            });
        }
    }
}
