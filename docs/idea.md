# Ideia do Projeto

## Objetivo

Criar um aplicativo desktop local, portatil para Windows, usando React e Tauri, para importar XMLs de documentos fiscais e gerar um relatorio Excel simples separando documentos de entrada, saida e documentos nos quais o CNPJ informado nao foi identificado na operacao.

## Problema

O usuario precisa analisar grandes volumes de XMLs fiscais de forma local e rapida, classificando documentos em relatorios separados conforme a relacao do CNPJ da empresa com cada documento fiscal.

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
