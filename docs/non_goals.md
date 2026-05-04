# Nao Objetivos

Ficam explicitamente fora do MVP:

- integracao com sistemas contabeis;
- envio de relatorios por e-mail;
- leitura ou importacao de PDF;
- documentacao tipo `README.md`;
- suporte inicial para macOS;
- suporte inicial para Linux;
- instalador `.msi` ou `.exe`;
- funcionamento com dependencia de internet;
- protecao por senha no Excel;
- criptografia local dos dados;
- historico completo de processamentos anteriores;
- persistencia de cache ou preferencias apos fechar o aplicativo;
- criacao de `config.json` local;
- previa dos dados na tela antes de gerar o Excel;
- qualquer funcionalidade fora do escopo definido sem nova confirmacao.

Para a extracao opcional de GTINS, ficam explicitamente fora do escopo:

- extrair GTINS de CT-e;
- extrair GTINS de notas sem CNPJ identificado;
- persistir o estado dos interruptores de GTINS;
- criar colunas alem de Descricao, NCM, CEST e GTIN;
- aplicar limite de palavras na descricao usada nas abas de GTINS;
- gerar um arquivo Excel separado apenas para GTINS.

Para o cache em sessao, ficam explicitamente fora do escopo:

- manter cache apos fechar e abrir novamente o aplicativo;
- salvar XMLs, documentos processados, hash ou preferencias em arquivo persistente;
- armazenar dados dentro do executavel;
- sincronizar cache entre maquinas;
- permitir dois processamentos concorrentes;
- descartar do cache itens ja processados com sucesso quando o usuario cancelar uma operacao.
