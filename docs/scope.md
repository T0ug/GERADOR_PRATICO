# Escopo

## Plataforma

- Aplicativo desktop local.
- Windows como unica plataforma inicial.
- Entrega como executavel portatil.
- Interface em portugues do Brasil.
- Stack definida pelo usuario: React e Tauri.

## Importacao

O aplicativo deve permitir importar:

- um XML individual;
- varios XMLs selecionados pelo explorer, sem limite fixo definido;
- um arquivo `.zip`;
- varios arquivos `.zip`;
- XMLs localizados em pastas internas dos arquivos `.zip`.

## Validacao de CPF/CNPJ

O usuario deve informar o CPF ou CNPJ da pessoa/empresa trabalhada.

O aplicativo deve:

- aceitar CPF com ou sem mascara;
- aceitar CNPJ com ou sem mascara;
- sanitizar o valor informado;
- validar o CPF ou CNPJ antes de processar os XMLs.

## Classificacao dos documentos

A classificacao deve usar o CPF/CNPJ informado pelo usuario.

- Saidas: quando o CPF/CNPJ informado aparecer como emitente.
- Entradas: quando o CPF/CNPJ informado aparecer como destinatario ou tomador.
- Notas sem CNPJ identificado na operacao: quando o CPF/CNPJ informado nao aparecer nos papeis esperados do documento.

## Relatorio Excel

O aplicativo deve gerar um unico arquivo Excel com tres abas:

- Entradas
- Saidas
- Notas sem CNPJ identificado na operacao

Opcionalmente, quando o usuario habilitar a extracao de GTINS, o mesmo arquivo Excel deve incluir aba(s) adicionais de GTINS conforme regras especificas descritas abaixo.

Todas as abas devem possuir as mesmas colunas:

- Data
- Numero da nota
- Valor
- CFOP
- Descricao dos itens
- Tomador
- Destinatario
- Remetente

## Regras das colunas

### Data

- Usar a data de emissao do documento fiscal.
- Se a data estiver ausente ou invalida, manter o campo em branco.

### Numero da nota

- Para NF-e e NFC-e, usar o numero da nota.
- Para CT-e, usar o numero do CT-e.

### Valor

- Usar o valor total do documento.

### CFOP

- Listar todos os CFOPs existentes nos itens.
- Separar os CFOPs por `;`.

### Descricao dos itens

- Juntar as descricoes dos itens em uma unica celula.
- Separar as descricoes por `;`.
- Para CT-e, usar a descricao do servico quando existir.
- A cada geracao de relatorio, o usuario deve escolher entre descricao completa ou limitada.
- Na opcao limitada, o usuario informa ate quantas palavras de cada item devem aparecer.
- O limite de palavras deve ser aplicado por item antes de juntar as descricoes com `;`.

### Tomador, Destinatario e Remetente

- Cada coluna deve conter `Razao Social CNPJ/CPF`.
- Se o XML nao tiver dados para uma dessas colunas, o campo deve ficar em branco.

## Arquivo Excel gerado

O aplicativo deve:

- permitir que o usuario escolha onde salvar o Excel a cada execucao;
- sugerir o nome padrao `Relatorio de notas [meses identificados nas datas das notas].xlsx`;
- listar todos os meses identificados no nome quando houver documentos de mais de um mes;
- aplicar formatacao basica no Excel:
  - cabecalho em negrito;
  - largura ajustada;
  - valores monetarios formatados.

## Extracao opcional de GTINS

A interface deve incluir um interruptor acima do botao de gerar relatorio com o texto:

- `Extrair GTINS tambem?`

Esse interruptor deve sempre iniciar desligado e nao deve ser persistido entre execucoes do aplicativo.

Quando `Extrair GTINS tambem?` estiver ligado, a interface deve exibir um segundo interruptor abaixo com o texto:

- `Separar GTINS de entrada e saida em abas diferentes?`

Esse segundo interruptor tambem deve sempre iniciar desligado e nao deve ser persistido entre execucoes.

Quando a extracao de GTINS estiver ligada e a separacao por tipo de operacao estiver desligada, o Excel deve incluir uma aba adicional chamada:

- `GTINS`

Quando a extracao de GTINS estiver ligada e a separacao por tipo de operacao estiver ligada, o Excel deve incluir duas abas adicionais chamadas:

- `GTINS Entradas`
- `GTINS Saidas`

### Documentos considerados para GTINS

A extracao de GTINS deve considerar apenas:

- produtos de NF-e;
- produtos de NFC-e;
- documentos classificados como entradas;
- documentos classificados como saidas.

A extracao de GTINS nao deve considerar:

- CT-e;
- notas sem CNPJ identificado.

### Colunas da aba de GTINS

As abas de GTINS devem possuir apenas as seguintes colunas:

- Descricao
- NCM
- CEST
- GTIN

Produtos sem CEST ou sem GTIN devem aparecer mesmo assim, deixando o campo correspondente em branco.

A descricao usada nas abas de GTINS deve ser sempre completa, sem aplicar o limite de palavras da descricao do relatorio principal.

### Deduplicacao de produtos nas abas de GTINS

A deduplicacao deve usar o conjunto completo:

- Descricao
- NCM
- CEST
- GTIN

Se todos os campos forem iguais, o produto deve aparecer apenas uma vez na aba correspondente.

Se ao menos um desses campos for diferente, o produto pode aparecer em uma nova linha quantas vezes forem necessarias.

## Duplicidade

O aplicativo deve:

- detectar documentos duplicados pela chave de acesso;
- ignorar duplicados silenciosamente;
- nao incluir duplicados no aviso final.

## Arquivos invalidos ou fora do escopo

Quando houver XML invalido, corrompido ou de tipo diferente de NF-e, NFC-e ou CT-e, o aplicativo deve:

- continuar o processamento;
- nao interromper a importacao;
- mostrar ao final uma lista com nome do arquivo e motivo.

## Ausencia de dados validos

Se nenhum dado valido for identificado nos arquivos importados, o aplicativo nao deve gerar Excel e deve mostrar um aviso informando que nao identificou nenhum dado valido nos arquivos.

## Cache em sessao e persistencia local

O aplicativo deve manter cache apenas enquanto estiver aberto.

O cache deve:

- ficar em memoria;
- ser perdido ao fechar o aplicativo;
- acumular XMLs processados com sucesso durante a sessao;
- identificar XMLs pelo hash do conteudo;
- identificar XMLs internos de arquivos `.zip` pelo hash do conteudo do XML;
- reaproveitar XMLs ja processados mesmo que venham de outro caminho ou outro ZIP;
- processar apenas XMLs novos ou com conteudo diferente;
- preservar no cache os XMLs processados com sucesso quando o usuario cancelar o processamento;
- permitir gerar novo Excel alterando apenas opcoes de relatorio, como descricao completa/limitada, limite de palavras e GTINS, sem reprocessar XMLs ja em cache;
- reaproveitar dados de parsing/importacao quando o CPF/CNPJ mudar, mas nao reaproveitar classificacoes incompatíveis com o novo CPF/CNPJ.

O aplicativo nao deve criar `config.json` nem persistir preferencias entre execucoes.

Devem ficar apenas em memoria e ser perdidos ao fechar:

- ultimo CPF/CNPJ usado;
- ultima pasta de importacao;
- ultima pasta de exportacao;
- ultima escolha de descricao completa ou limitada;
- ultimo limite de palavras usado.

## Processamento

O aplicativo deve:

- suportar importacoes com dezenas de milhares de XMLs;
- suportar a extracao opcional de GTINS em volumes grandes, incluindo mais de 30 mil XMLs e mais de 100 mil produtos;
- manter a interface responsiva;
- mostrar progresso durante processamentos grandes;
- desabilitar o botao de gerar relatorio enquanto houver processamento em andamento;
- nao permitir iniciar dois processamentos ao mesmo tempo por duplo clique;
- oferecer botao de cancelar durante o processamento;
- ao cancelar, manter no cache da sessao os XMLs ja processados com sucesso;
- ao tentar fechar o app, exibir modal no estilo visual da aplicacao avisando que os dados processados em cache serao perdidos e que, se quiser gerar relatorio novamente, sera necessario processar os arquivos novamente.

## Seguranca

Nao ha exigencia de senha, criptografia ou protecao especifica.

O app e de uso pessoal, interno, local e sem acesso externo.
