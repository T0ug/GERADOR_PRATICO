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

- Validar e sanitizar CPF/CNPJ informado.
- Classificar documentos em Entradas, Saidas ou Notas sem CNPJ identificado na operacao.
- Detectar duplicados por chave de acesso e ignora-los.

## 6. Geracao do Excel

- Criar um unico arquivo Excel com tres abas.
- Preencher as colunas definidas no escopo.
- Aplicar formatacao basica.
- Gerar nome sugerido com os meses identificados nas datas das notas.

## 7. Cache em sessao e preferencias

- Manter cache em memoria apenas enquanto o app estiver aberto.
- Identificar XMLs por hash do conteudo, inclusive XMLs internos de ZIP.
- Reaproveitar XMLs ja processados para gerar novos relatorios sem reprocessamento.
- Processar apenas XMLs novos ou com conteudo diferente.
- Reclassificar documentos quando o CPF/CNPJ informado mudar.
- Manter preferencias simples, como ultimo CPF/CNPJ e pastas, apenas em memoria.
- Remover persistencia em `config.json`.

## 8. Validacao do MVP

- Validar fluxos com XMLs validos.
- Validar fluxos com XMLs invalidos e fora do escopo.
- Validar importacao por `.zip`.
- Validar grandes volumes com progresso e interface responsiva.
- Validar ausencia total de dados validos.

## 9. Evolucao: extracao opcional de GTINS

- Atualizar a arquitetura para incluir a extracao opcional de produtos de NF-e/NFC-e.
- Incluir na interface os interruptores de GTINS, sempre iniciando desligados.
- Extrair Descricao, NCM, CEST e GTIN dos itens de entrada e saida.
- Deduplicar produtos pelo conjunto Descricao + NCM + CEST + GTIN.
- Gerar aba unica `GTINS` ou abas `GTINS Entradas` e `GTINS Saidas` no mesmo Excel.
- Validar desempenho com grandes volumes, incluindo mais de 30 mil XMLs e mais de 100 mil produtos.

## 10. Evolucao: cancelamento e protecao de processamento

- Desabilitar o botao de gerar relatorio enquanto houver processamento em andamento.
- Impedir processamento duplicado por duplo clique.
- Oferecer botao de cancelar processamento.
- Preservar no cache os XMLs ja processados com sucesso quando o usuario cancelar.
- Exibir modal de confirmacao ao fechar o app, avisando que o cache em memoria sera perdido.
