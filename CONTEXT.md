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

**Canal**:
Inbox do Chatwoot vinculado a exatamente uma Conta.
_Avoid_: inbox (no código de domínio)

**Ponte**:
O papel do byo-chat: transportar mensagens entre Conta e Canal nos dois sentidos.

## Relationships

- Uma **Rede** tem um ou mais **Conectores** possíveis; cada **Conta** usa exatamente um **Conector**
- Cada **Conta** mapeia para exatamente um **Canal** (1:1)

## Flagged ambiguities

- "nat" — resolvido: não é NATS nem NAT traversal embutido; fila é local (ver ADR-0001).
- Risco de banimento: responsabilidade do usuário, exibido como disclaimer; o byo-chat não tenta mitigar.
