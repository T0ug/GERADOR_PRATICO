# Ideia do Projeto

## Objetivo

Criar um aplicativo desktop local, portatil para Windows, usando React e Tauri, para importar XMLs de documentos fiscais e gerar um relatorio Excel simples separando documentos de entrada, saida e documentos nos quais o CPF/CNPJ informado nao foi identificado na operacao.

## Problema

O usuario precisa analisar grandes volumes de XMLs fiscais de forma local e rapida, classificando documentos em relatorios separados conforme a relacao do CPF/CNPJ informado com cada documento fiscal.

## Publico-alvo

Uso pessoal, interno e local pelo proprio usuario.

## Tipos de documentos aceitos inicialmente

- NF-e
- NFC-e
- CT-e

## Resultado esperado

Um unico arquivo Excel gerado a partir dos XMLs importados, contendo tres abas:

- Entradas
- Saidas
- Notas sem CNPJ identificado na operacao

O aplicativo deve funcionar totalmente offline, sem dependencia de internet apos instalado.

## Evolucao prevista: extracao de GTINS

Adicionar ao fluxo atual de geracao do Excel uma opcao para extrair tambem uma lista unica de produtos com GTIN.

Quando habilitada pelo usuario, a extracao deve incluir produtos de NF-e e NFC-e classificados como entradas ou saidas, gerando aba unica `GTINS` ou abas separadas `GTINS Entradas` e `GTINS Saidas`, conforme escolha feita na interface.

## Evolucao prevista: cache em sessao e cancelamento

Adicionar cache em memoria enquanto o app estiver aberto para reaproveitar XMLs ja processados, identificados por hash do conteudo. O usuario deve poder gerar novos relatorios alterando apenas opcoes de saida sem reprocessar XMLs ja carregados.

O app deve impedir processamento concorrente, desabilitando o botao de gerar relatorio durante o processamento e oferecendo botao de cancelar. Ao cancelar, os XMLs processados com sucesso devem permanecer no cache da sessao.
