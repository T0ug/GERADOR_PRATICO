# Gerador de Relatorio de Notas

Aplicativo desktop local para Windows, feito com React + Tauri, que le XMLs fiscais e gera um relatorio Excel para apoio contabil.

O processamento fiscal fica no backend Rust. A interface React coordena a selecao dos arquivos, as opcoes do relatorio, o progresso e a exibicao do resultado.

## Estado atual

O projeto esta em desenvolvimento ativo.

Implementado no codigo atual:

- importacao de arquivos `.xml` e `.zip`;
- leitura de XMLs dentro de pastas internas de arquivos ZIP;
- suporte a NF-e, NFC-e e CT-e;
- validacao de CNPJ no frontend;
- classificacao por CNPJ informado:
  - `Entradas`;
  - `Saidas`;
  - `Notas sem CNPJ identificado`;
- deduplicacao de documentos pela chave de acesso;
- geracao de um unico arquivo `.xlsx`;
- escolha do local de salvamento ao final do processamento;
- progresso por eventos Tauri durante leitura, processamento e exportacao;
- avisos finais para arquivos invalidos, corrompidos ou fora do escopo;
- ignorar XMLs de evento sem poluir o aviso final;
- descricao completa ou limitada por quantidade de palavras;
- abas opcionais de GTINS:
  - `GTINS`;
  - `GTINS Entradas` e `GTINS Saidas`;
- deduplicacao de GTINS por `Descricao + NCM + CEST + GTIN`;
- formatacao do Excel com logo, cabecalhos, bordas, valores monetarios e datas em formato brasileiro;
- persistencia atual em `config.json` do ultimo CNPJ e pastas usadas.

Definido no escopo/documentacao para a proxima evolucao, mas ainda nao refletido no codigo atual:

- aceitar CPF alem de CNPJ no campo principal;
- remover a persistencia em `config.json`;
- manter ultimo CPF/CNPJ e pastas apenas em memoria;
- cache de XMLs em memoria durante a sessao, identificado por hash do conteudo;
- reaproveitar parsing/importacao ja processados ao gerar novos relatorios;
- reclassificar os documentos quando o CPF/CNPJ informado mudar;
- bloquear processamento concorrente por duplo clique;
- permitir cancelamento preservando no cache os XMLs ja processados com sucesso;
- avisar ao fechar o app que o cache em memoria sera perdido.

## Stack

- React 19
- TypeScript
- Vite
- Tauri 2
- Rust 2021
- `roxmltree` para leitura XML
- `zip` para leitura de arquivos compactados
- `rust_xlsxwriter` para geracao do Excel
- Vitest para testes do frontend
- testes nativos Rust para backend

## Estrutura principal

```text
src/
  App.tsx                  Interface principal
  components/              Componentes de formulario, progresso, GTINS e modal
  utils/cnpj.ts            Sanitizacao e validacao de CNPJ

src-tauri/src/
  commands.rs              Comandos Tauri e orquestracao do fluxo
  importer.rs              Coleta de XMLs diretos e internos de ZIP
  parser.rs                Parsing fiscal de NF-e, NFC-e e CT-e
  classifier.rs            Classificacao por documento informado
  deduplicator.rs          Deduplicacao por chave de acesso
  report.rs                Geracao e formatacao do Excel
  progress.rs              Eventos de progresso
  config.rs                Persistencia local atual

docs/
  Fonte de verdade do projeto, pipeline, escopo, arquitetura, tasks e handoffs
```

## Requisitos

- Windows
- Node.js e npm
- Rust stable
- dependencias de build exigidas pelo Tauri 2 no Windows

## Comandos

Instalar dependencias:

```bash
npm install
```

Rodar a interface Vite:

```bash
npm run dev
```

Rodar o app desktop em modo desenvolvimento:

```bash
npm run tauri:dev
```

Testes do frontend:

```bash
npm run test
```

Build do frontend:

```bash
npm run build
```

Testes do backend Rust:

```bash
cd src-tauri
cargo test
```

Build do backend Rust:

```bash
cd src-tauri
cargo build
```

Build desktop Tauri:

```bash
npm run tauri:build
```

## Fluxo de uso

1. Informe o CNPJ da empresa.
2. Selecione arquivos `.xml` e/ou `.zip`.
3. Escolha descricao completa ou limitada.
4. Opcionalmente, habilite a extracao de GTINS.
5. Clique em `Gerar relatorio`.
6. Ao final do processamento, escolha onde salvar o Excel.

## Observacoes de escopo

- O app e local e offline.
- A plataforma inicial e Windows.
- A entrega desejada e executavel portatil.
- Nao ha integracao com sistemas contabeis, e-mail, PDF, banco de dados ou internet.
- `docs/` e a fonte de verdade do projeto e deve ser consultado antes de novas alteracoes.
