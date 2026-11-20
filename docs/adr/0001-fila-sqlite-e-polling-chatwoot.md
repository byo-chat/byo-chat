# Fila local em SQLite e polling do Chatwoot por padrão

O byo-chat roda num PC atrás de NAT e o Chatwoot pode estar na nuvem ou local. Usamos uma fila persistente em SQLite (inbox/outbox com retry) em vez de um broker como NATS, para manter um único binário sem processos extras. Para mensagens de saída (atendente → contato), o padrão é polling da API do Chatwoot, pois o webhook de callback não alcança uma máquina atrás de NAT; webhook via túnel (Cloudflare Tunnel, Tailscale Funnel, ngrok) é opcional para tempo real.

## Considered Options

- NATS embutido: rejeitado — overhead sem ganho numa única máquina.
- Só webhook: rejeitado — exige exposição pública da máquina do usuário.
