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

## Validacao de CNPJ

O usuario deve informar o CNPJ da empresa trabalhada.

O aplicativo deve:

- aceitar CNPJ com ou sem mascara;
- sanitizar o valor informado;
- validar o CNPJ antes de processar os XMLs.

## Classificacao dos documentos

A classificacao deve usar o CNPJ informado pelo usuario.

- Saidas: quando o CNPJ informado aparecer como emitente.
- Entradas: quando o CNPJ informado aparecer como destinatario ou tomador.
- Notas sem CNPJ identificado na operacao: quando o CNPJ informado nao aparecer nos papeis esperados do documento.

## Relatorio Excel

O aplicativo deve gerar um unico arquivo Excel com tres abas:

- Entradas
- Saidas
- Notas sem CNPJ identificado na operacao

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

## Persistencia local

O aplicativo deve salvar localmente:

- ultimo CNPJ usado;
- ultima pasta de importacao;
- ultima pasta de exportacao.

O aplicativo nao deve salvar:

- ultima escolha de descricao completa ou limitada;
- ultimo limite de palavras usado.

## Processamento

O aplicativo deve:

- suportar importacoes com dezenas de milhares de XMLs;
- manter a interface responsiva;
- mostrar progresso durante processamentos grandes;
- cancelar simplesmente o processamento se o app for fechado.

## Seguranca

Nao ha exigencia de senha, criptografia ou protecao especifica.

O app e de uso pessoal, interno, local e sem acesso externo.
