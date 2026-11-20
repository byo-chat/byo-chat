# byo-chat

Ponte local que mantém contas de redes de mensagem conectadas via CLIs/bridges de terceiros e as expõe ao Chatwoot como canais de atendimento.

## Language

**Rede**:
Serviço de mensagens externo (WhatsApp, Telegram, Instagram, LinkedIn, X, Teams).
_Avoid_: plataforma, provider

**Conta**:
Uma identidade autenticada do usuário numa Rede. Uma Rede pode ter várias Contas.
_Avoid_: sessão, perfil

**Conector**:
Software de terceiros (CLI ou bridge) que o byo-chat supervisiona para operar Contas de uma Rede.
_Avoid_: driver, plugin

**Estratégia**:
Forma como o byo-chat conversa com um Conector: Stream, Poll ou Matrix.

**Manifesto de Conector**:
Descrição declarativa de um Conector: Rede, Estratégia, binário e versão, comandos de evento/envio/saúde e mapeamento para a Mensagem canônica.

**Evento canônico**:
Unidade comum de entrada e saída da Ponte (mensagem, mídia, reação, edição, exclusão, recibo, resposta, entrada/saída de membro), com semântica dos eventos Matrix e esquema próprio versionado (`byo.*.v1`).
_Avoid_: payload, mensagem do Chatwoot

**Conversa**:
Diálogo numa Conta: direta (1:1) ou grupo. Grupos são suportados mesmo quando o destino não os suporta.

**Canal**:
Inbox do Chatwoot vinculado a exatamente uma Conta.
_Avoid_: inbox (no código de domínio)

**Ponte**:
O papel do byo-chat: transportar mensagens entre Conta e Canal nos dois sentidos.

## Relationships

- Uma **Rede** tem um ou mais **Conectores** possíveis; cada **Conta** usa exatamente um **Conector**
- Cada **Conector** é descrito por um **Manifesto** e usa uma **Estratégia**
- Cada **Conta** mapeia para exatamente um **Canal** (1:1)

## Flagged ambiguities

- "nat" — resolvido: não é NATS nem NAT traversal embutido; fila é local (ver ADR-0001).
- Referência de cobertura: WhatsApp (wacli) e Telegram; demais Redes se adaptam.
- Licença: MIT (mantida; Conectores AGPL ficam isolados como processos).
- Risco de banimento: responsabilidade do usuário, exibido como disclaimer; o byo-chat não tenta mitigar.
