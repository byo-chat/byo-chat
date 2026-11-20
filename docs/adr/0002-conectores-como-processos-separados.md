# Conectores rodam como processos separados, não linkados no binário

A maioria dos Conectores é escrita em Go, e os bridges mautrix são AGPL-3.0. Linkar código Go no binário Rust (cgo c-archive) traria um runtime Go por biblioteca, builds frágeis e, no caso dos mautrix, obrigaria o byo-chat inteiro a virar AGPL, o que conflita com a licença MIT. Por isso cada Conector, e o Conduit usado pelo LinkedIn, roda como processo separado ("sidecar"): o byo-chat baixa ou empacota o binário na versão fixada no Manifesto, supervisiona, reinicia e isola cada um. Isso também dá o isolamento que queremos: um Conector que trava ou vaza memória não derruba a Ponte.

## Considered Options

- Linkar via cgo/FFI: rejeitado por licença, tamanho e fragilidade de build.
- Reescrever os Conectores em Rust: rejeitado pelo custo e para não acompanhar mudanças de protocolo das Redes por conta própria.
