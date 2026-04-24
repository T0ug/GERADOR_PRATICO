# Plano Inicial de Implementacao

Este plano e macro e serve apenas como orientacao inicial. A arquitetura detalhada deve ser definida em etapa posterior.

## 1. Base do aplicativo

- Criar app desktop com React e Tauri.
- Configurar execucao local no Windows.
- Preparar empacotamento como executavel portatil.

## 2. Interface principal

- Criar tela principal em portugues do Brasil.
- Permitir informar CNPJ.
- Permitir selecionar XMLs individuais ou em lote.
- Permitir selecionar um ou mais arquivos `.zip`.
- Permitir escolher descricao completa ou limitada.
- Permitir informar limite de palavras quando a descricao for limitada.
- Permitir escolher local de salvamento do Excel.
- Exibir progresso durante o processamento.

## 3. Importacao e leitura de arquivos

- Ler XMLs selecionados diretamente.
- Extrair XMLs de arquivos `.zip`, incluindo pastas internas.
- Separar arquivos validos, invalidos, corrompidos e fora do escopo.

## 4. Interpretacao dos documentos fiscais

- Extrair dados de NF-e.
- Extrair dados de NFC-e.
- Extrair dados de CT-e.
- Normalizar os campos necessarios para o relatorio.

## 5. Classificacao

- Validar e sanitizar CNPJ informado.
- Classificar documentos em Entradas, Saidas ou Notas sem CNPJ identificado na operacao.
- Detectar duplicados por chave de acesso e ignora-los.

## 6. Geracao do Excel

- Criar um unico arquivo Excel com tres abas.
- Preencher as colunas definidas no escopo.
- Aplicar formatacao basica.
- Gerar nome sugerido com os meses identificados nas datas das notas.

## 7. Persistencia local

- Salvar ultimo CNPJ usado.
- Salvar ultima pasta de importacao.
- Salvar ultima pasta de exportacao.

## 8. Validacao do MVP

- Validar fluxos com XMLs validos.
- Validar fluxos com XMLs invalidos e fora do escopo.
- Validar importacao por `.zip`.
- Validar grandes volumes com progresso e interface responsiva.
- Validar ausencia total de dados validos.
