# Arquitetura Tecnica

## Visao geral

O aplicativo sera um desktop local para Windows usando React no frontend e Tauri/Rust no backend.

A arquitetura escolhida mantem o React responsavel pela interface e concentra no Rust todo processamento pesado: leitura de arquivos, extracao de ZIP, parsing de XML, classificacao, deduplicacao, geracao do Excel e persistencia local simples.

A evolucao de extracao opcional de GTINS deve seguir a mesma diretriz: o React apenas coleta as opcoes do usuario e o Rust concentra a extracao, deduplicacao e escrita das abas adicionais no Excel.

## Camadas

### Frontend React

Responsavel por:

- exibir a interface em portugues do Brasil;
- receber CNPJ informado pelo usuario;
- permitir selecao de XMLs e ZIPs;
- permitir escolha entre descricao completa ou limitada;
- permitir informar limite de palavras quando aplicavel;
- permitir escolher local de salvamento do Excel;
- exibir progresso durante processamento;
- exibir resultado final, erros bloqueantes e avisos.
- permitir habilitar ou desabilitar a extracao opcional de GTINS;
- permitir separar GTINS de entrada e saida em abas diferentes somente quando a extracao de GTINS estiver habilitada.

O frontend nao deve processar XML bruto nem manter grandes massas de dados fiscais em memoria.

### Backend Tauri/Rust

Responsavel por:

- sanitizar e validar CNPJ;
- ler XMLs selecionados diretamente;
- ler arquivos `.zip` e localizar XMLs em pastas internas;
- identificar NF-e, NFC-e e CT-e;
- extrair dados fiscais;
- normalizar os documentos;
- detectar duplicados pela chave de acesso;
- classificar documentos em entrada, saida ou sem CNPJ identificado;
- gerar Excel com tres abas;
- gerar abas adicionais de GTINS quando a opcao estiver habilitada;
- aplicar formatacao basica no Excel;
- persistir configuracoes locais;
- emitir eventos de progresso para o frontend.

## Componentes

### Frontend

- `App`: coordena a tela principal e os estados da operacao.
- `CnpjInput`: recebe CNPJ com ou sem mascara e mostra validacao visual.
- `ImportSelector`: permite selecionar multiplos XMLs e multiplos ZIPs.
- `DescriptionOptions`: controla descricao completa ou limitada e limite de palavras.
- `GtinsOptions`: controla os interruptores locais de extracao de GTINS e separacao por entrada/saida.
- `ExportSelector`: permite escolher o local do Excel.
- `ProgressPanel`: mostra progresso de leitura, processamento e exportacao.
- `ResultDialog`: mostra sucesso, ausencia de dados validos ou lista de avisos.

### Backend

- `commands`: comandos expostos ao React via Tauri.
- `config`: leitura e gravacao das preferencias locais.
- `importer`: leitura de XMLs diretos e extracao de XMLs dentro de ZIPs.
- `parser`: deteccao e extracao de NF-e, NFC-e e CT-e.
- `classifier`: classificacao por comparacao com o CNPJ informado.
- `deduplicator`: controle de chaves de acesso ja processadas.
- `report`: geracao do Excel, incluindo abas opcionais de GTINS.
- `progress`: emissao de eventos de progresso.
- `errors`: padronizacao de erros bloqueantes e avisos.

## Fluxo de dados

1. O app carrega as configuracoes locais.
2. O usuario informa CNPJ, seleciona XMLs/ZIPs, escolhe opcoes de descricao e pode habilitar a extracao opcional de GTINS.
3. O React chama um comando Tauri com caminhos, CNPJ, opcoes de descricao e opcoes de GTINS.
4. O Rust sanitiza e valida o CNPJ.
5. O `importer` monta a lista de XMLs candidatos, incluindo XMLs dentro de ZIPs.
6. O `parser` tenta identificar e normalizar cada documento fiscal. Para NF-e e NFC-e, tambem extrai itens de produto para GTINS; para CT-e, a lista de produtos fica vazia.
7. O `deduplicator` ignora documentos com chave de acesso ja processada.
8. O `classifier` classifica documentos validos em Entradas, Saidas ou Notas sem CNPJ identificado na operacao.
9. O backend acumula avisos nao bloqueantes com nome do arquivo e motivo.
10. Se nao houver dados validos, o backend retorna aviso e nao gera Excel.
11. Se houver dados validos, o `report` gera um unico Excel com tres abas principais.
12. Se a extracao de GTINS estiver habilitada, o `report` filtra produtos de NF-e/NFC-e classificados como entrada ou saida, deduplica por Descricao + NCM + CEST + GTIN e cria a aba `GTINS` ou as abas `GTINS Entradas` e `GTINS Saidas`.
13. O backend salva ultimo CNPJ, ultima pasta de importacao e ultima pasta de exportacao.
14. Durante o processo, o backend envia eventos de progresso ao React.

## Modelo normalizado

Cada documento fiscal valido deve ser convertido para uma estrutura comum contendo:

- chave de acesso;
- tipo do documento: NF-e, NFC-e ou CT-e;
- data de emissao opcional;
- numero do documento;
- valor total;
- CFOPs;
- descricoes;
- emitente;
- tomador;
- destinatario;
- remetente;
- itens de produto para GTINS.

Pessoas ou empresas relacionadas ao documento devem ser representadas com:

- razao social ou nome;
- CNPJ ou CPF.

No Excel, `Tomador`, `Destinatario` e `Remetente` devem usar o formato `Razao Social CNPJ/CPF` e ficar em branco quando ausentes.

### Itens de produto para GTINS

Itens de produto devem ser representados por uma estrutura normalizada contendo apenas:

- descricao;
- NCM;
- CEST;
- GTIN.

Para NF-e e NFC-e, cada item `<det><prod>` deve gerar um item de produto quando houver descricao. CEST, NCM e GTIN ausentes devem ser normalizados como campo em branco.

Para CT-e, a lista de itens de produto deve ficar vazia, porque CT-e esta fora da extracao de GTINS.

A descricao de itens de GTINS deve ser sempre completa e nao deve sofrer o limite de palavras usado na aba principal do relatorio.

## Contrato UI/backend para GTINS

O request do comando de geracao de relatorio deve incluir:

- `extract_gtins: bool`;
- `split_gtins_by_operation: bool`.

Regras do contrato:

- se `extract_gtins` for `false`, o backend deve ignorar `split_gtins_by_operation`;
- se `extract_gtins` for `true` e `split_gtins_by_operation` for `false`, o Excel deve receber a aba `GTINS`;
- se `extract_gtins` for `true` e `split_gtins_by_operation` for `true`, o Excel deve receber as abas `GTINS Entradas` e `GTINS Saidas`;
- o frontend nao envia produtos ao backend;
- o backend nao retorna listas de produtos ao frontend.

O formato principal da resposta do comando pode permanecer inalterado. Contagens de GTINS nao sao obrigatorias no escopo atual.

## Persistencia local

A persistencia sera feita em arquivo local de configuracao do app, sem banco de dados.

Dados persistidos:

- ultimo CNPJ usado;
- ultima pasta de importacao;
- ultima pasta de exportacao.

Dados nao persistidos:

- escolha de descricao completa ou limitada;
- limite de palavras;
- escolha de extrair GTINS;
- escolha de separar GTINS por entrada e saida;
- historico de processamentos;
- XMLs importados;
- relatorios gerados;
- avisos de processamento.
- produtos ou GTINS extraidos.

## Integracoes

O app nao tera integracoes externas.

Integracoes locais permitidas:

- dialogos nativos de arquivos e pastas via Tauri;
- leitura do filesystem local;
- leitura de arquivos `.xml`;
- leitura de arquivos `.zip`;
- gravacao de arquivo `.xlsx`;
- arquivo local de configuracao.

Nao ha integracao externa nova para GTINS. A extracao usa apenas XMLs ja importados, parsing local, comando Tauri existente e geracao local do Excel.

## Tratamento de erro

### Erros bloqueantes

- CNPJ invalido.
- Caminho de exportacao ausente ou invalido.
- Falha ao gravar o Excel.
- Falha inesperada que impeça continuar com seguranca.

Comportamento: interromper o fluxo atual e mostrar mensagem clara ao usuario.

### Avisos nao bloqueantes

- XML invalido.
- XML corrompido.
- XML de tipo diferente de NF-e, NFC-e ou CT-e.
- Data ausente ou invalida.
- Campos opcionais ausentes.

Comportamento: continuar processando os demais arquivos e mostrar ao final a lista de arquivos com problema e motivo, quando aplicavel.

### Duplicados

Documentos com chave de acesso ja processada devem ser ignorados silenciosamente e nao entram nos avisos finais.

### Ausencia de dados validos

Se nenhum documento valido for identificado, o app nao gera Excel e mostra aviso informando que nao identificou nenhum dado valido nos arquivos.

### Extracao opcional de GTINS

Campos ausentes em produtos nao devem criar erro bloqueante.

- Produto sem GTIN deve aparecer com GTIN em branco.
- Produto sem CEST deve aparecer com CEST em branco.
- Produto sem NCM deve aparecer com NCM em branco.
- Produto sem descricao deve ser ignorado na lista de GTINS, pois a descricao faz parte da linha e da chave de deduplicacao.
- CT-e nao deve gerar erro nem aviso por nao possuir produtos de GTINS.
- Nota sem CNPJ identificado nao deve gerar erro nem aviso por ficar fora das abas de GTINS.

Se a extracao de GTINS estiver habilitada e nenhum produto elegivel for encontrado, o Excel ainda deve ser gerado quando houver documentos validos para o relatorio principal. A(s) aba(s) de GTINS podem ser geradas apenas com cabecalho.

### Fechamento do app

Se o app for fechado durante o processamento, a operacao e cancelada sem retomada automatica.

## Escalabilidade e desempenho

A arquitetura deve suportar dezenas de milhares de XMLs por importacao.

Diretrizes:

- processamento pesado sempre no Rust;
- frontend recebe apenas progresso, estado e resultado resumido;
- evitar enviar XML bruto para o React;
- usar deduplicacao em memoria por chave de acesso;
- quando GTINS estiver habilitado, usar deduplicacao em memoria por chave composta de produto;
- acumular avisos em formato leve;
- gerar o Excel ao final a partir dos dados normalizados.
- nao reprocessar XML apenas para extrair GTINS.

Para GTINS:

- extrair itens de produto durante o parsing;
- manter os itens dentro do documento normalizado para aproveitar a deduplicacao por chave de acesso;
- deduplicar produtos no momento da geracao do Excel com chave composta por Descricao + NCM + CEST + GTIN;
- em aba unica `GTINS`, usar um unico conjunto de deduplicacao;
- em abas separadas, usar conjuntos independentes para entradas e saidas;
- nao enviar produtos ao React;
- nao criar arquivo intermediario.

Limites aceitos no MVP:

- uso de memoria proporcional ao volume de documentos validos e avisos;
- uso de memoria proporcional ao volume de produtos extraidos quando GTINS estiver habilitado;
- sem processamento distribuido;
- sem retomada apos fechamento;
- sem banco de dados intermediario.

## Dependencias esperadas

As dependencias especificas devem ser escolhidas na implementacao, respeitando estes papeis:

- biblioteca Rust para parsing XML;
- biblioteca Rust para leitura de ZIP;
- biblioteca Rust para geracao de Excel;
- APIs do Tauri para comandos, dialogos e eventos.

## Riscos tecnicos

- Variacoes reais nos XMLs de NF-e, NFC-e e CT-e podem exigir ajustes no parser.
- Grandes volumes podem pressionar memoria caso muitos documentos validos sejam acumulados antes da geracao do Excel.
- Arquivos ZIP muito grandes ou corrompidos precisam de tratamento cuidadoso para manter o app responsivo.
- A geracao do Excel pode ser um ponto de custo de tempo em importacoes muito grandes.
- A extracao de GTINS aumenta memoria e tempo de geracao do Excel quando houver mais de 100 mil produtos.
- XMLs reais podem variar nas tags de GTIN, NCM e CEST; a implementacao deve ser validada com fixtures representativas.

## Decisoes arquiteturais

- React sera camada de interface, sem processamento fiscal pesado.
- Rust sera a camada de processamento local.
- A comunicacao entre UI e backend sera feita por comandos Tauri e eventos de progresso.
- Persistencia sera feita em arquivo local de configuracao, sem banco de dados.
- O Excel sera gerado apenas quando houver ao menos um documento valido.
- Duplicados por chave de acesso serao ignorados silenciosamente.
- A extracao opcional de GTINS sera implementada estendendo o modelo normalizado existente com itens de produto.
- GTINS serao extraidos no parser para NF-e/NFC-e e escritos pelo modulo `report` apos classificacao.
- CT-e e notas sem CNPJ identificado ficarao fora da extracao de GTINS.
- Os interruptores de GTINS nao serao persistidos.
