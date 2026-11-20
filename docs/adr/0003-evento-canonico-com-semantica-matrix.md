# Evento canônico próprio com semântica Matrix; Chatwoot é só um adaptador

O formato interno da Ponte é um esquema próprio versionado (`byo.*.v1`) que segue a semântica dos eventos Matrix (mensagem, reação, edição, exclusão, recibo, resposta, membros de grupo). O protocolo do Chatwoot não cobre grupos, reações nem edições, e usá-lo como modelo amarraria o domínio ao destino mais pobre; ele vira um adaptador de saída. WhatsApp (wacli) e Telegram definem a cobertura mínima.

## Considered Options

- Modelo do Chatwoot: rejeitado, cobertura insuficiente.
- Matrix completo (todo Conector como bridge, byo-chat como cliente Matrix): rejeitado, exigiria Conduit sempre ativo e escrever bridges para wacli/Telegram. A semântica compatível mantém essa porta aberta.
