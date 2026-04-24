# Decision Log

## 2026-04-24 - Stack inicial

Decisao: usar React e Tauri.

Justificativa: stack solicitada pelo usuario para criar um aplicativo desktop local.

## 2026-04-24 - Plataforma inicial

Decisao: suportar inicialmente apenas Windows.

Justificativa: o usuario informou que o uso sera apenas em Windows.

## 2026-04-24 - Formato de entrega

Decisao: entregar como executavel portatil.

Justificativa: o usuario prefere executavel portatil em vez de instalador.

## 2026-04-24 - Tipos de XML aceitos

Decisao: aceitar inicialmente NF-e, NFC-e e CT-e.

Justificativa: esses foram os tres tipos solicitados para o MVP.

## 2026-04-24 - Classificacao por CNPJ

Decisao: classificar como saida quando o CNPJ informado for emitente, entrada quando for destinatario ou tomador, e nao identificado quando nao aparecer nos papeis esperados.

Justificativa: regra confirmada pelo usuario.

## 2026-04-24 - Estrutura do Excel

Decisao: gerar um unico arquivo Excel com tres abas.

Justificativa: o usuario solicitou um unico arquivo contendo Entradas, Saidas e Notas sem CNPJ identificado na operacao.

## 2026-04-24 - Tratamento de invalidos

Decisao: continuar o processamento quando houver arquivo invalido, corrompido ou fora do escopo, exibindo aviso final com arquivo e motivo.

Justificativa: o usuario nao quer interromper a importacao por causa desses arquivos.

## 2026-04-24 - Duplicidade

Decisao: detectar duplicados pela chave de acesso e ignora-los silenciosamente.

Justificativa: regra solicitada pelo usuario.

## 2026-04-24 - Persistencia local

Decisao: salvar ultimo CNPJ, ultima pasta de importacao e ultima pasta de exportacao.

Justificativa: melhora o uso recorrente sem salvar escolhas que devem ser feitas a cada processamento.

## 2026-04-24 - Abordagem arquitetural inicial

Decisao: usar React como camada de interface e Tauri/Rust como camada responsavel por leitura de arquivos, extracao de ZIP, parsing de XML, classificacao, deduplicacao e geracao do Excel.

Alternativas consideradas:

- dividir parte do processamento fiscal no frontend React;
- criar desde o inicio um pipeline Rust mais formal e granular.

Justificativa: a abordagem escolhida mantem a interface leve, concentra processamento pesado no backend local, atende melhor ao volume de dezenas de milhares de XMLs e evita complexidade desnecessaria para o MVP.

Impacto: a arquitetura deve separar UI, comandos Tauri, processamento fiscal, persistencia local e exportacao Excel.

## 2026-04-24 - Comunicacao UI/backend

Decisao: usar comandos Tauri para iniciar operacoes e eventos Tauri para comunicar progresso do backend Rust para o frontend React.

Justificativa: o processamento pode envolver dezenas de milhares de XMLs e deve manter a interface responsiva sem enviar grandes volumes de dados brutos ao frontend.

Impacto: o frontend deve receber progresso e resultado resumido; o backend concentra o estado do processamento.

## 2026-04-24 - Persistencia sem banco de dados

Decisao: usar arquivo local de configuracao para ultimo CNPJ, ultima pasta de importacao e ultima pasta de exportacao.

Justificativa: o escopo exige apenas preferencias simples, sem historico, sem dados relacionais e sem necessidade de consulta posterior.

Impacto: nao sera introduzido banco de dados no MVP.

## 2026-04-24 - Transicao para execucao

Decisao: considerar a arquitetura concluida e iniciar a execucao pela task T001, criando a base inicial React + Tauri.

Justificativa: `docs/architecture.md` foi criado, as decisoes tecnicas principais foram registradas e o usuario confirmou que a arquitetura esta correta.

Impacto: `docs/project_status.md`, `docs/tasks.md` e `docs/handoff.md` passam a apontar para o workflow `execute_task.md`, com Executor e skill `implement_task`.

## 2026-04-24 - Continuidade apos T001

Decisao: avancar para a task T002, implementando a interface principal estatica.

Justificativa: T001 foi aprovada com ressalvas leves em `docs/review_report.md`; nao ha bloqueio para continuidade. A proxima entrega segura e incremental e preparar a UI antes das integracoes reais.

Impacto: `docs/project_status.md`, `docs/tasks.md` e `docs/handoff.md` passam a apontar para T002 com Executor e skill `implement_task`.

## 2026-04-24 - Continuidade apos T002

Decisao: avancar para a task T003, implementando validacao e sanitizacao de CNPJ no frontend.

Justificativa: T002 foi aprovada com ressalvas leves em `docs/review_report.md`; nao ha bloqueio para continuidade. Validar CNPJ e um passo pequeno e necessario antes de qualquer processamento fiscal.

Impacto: `docs/project_status.md`, `docs/tasks.md` e `docs/handoff.md` passam a apontar para T003 com Executor e skill `implement_task`.

## 2026-04-24 - Continuidade apos T003

Decisao: avancar para a task T004, integrando selecao nativa de XMLs, ZIPs e destino de exportacao via Tauri.

Justificativa: T003 foi aprovada em `docs/review_report.md`. A selecao nativa de caminhos e o proximo passo incremental antes de qualquer leitura ou processamento de arquivos.

Impacto: `docs/project_status.md`, `docs/tasks.md` e `docs/handoff.md` passam a apontar para T004 com Executor e skill `implement_task`.

## 2026-04-24 - Continuidade apos T004

Decisao: avancar para a task T005, implementando o importador backend de candidatos XML e ZIP.

Justificativa: T004 foi aprovada com ressalvas leves em `docs/review_report.md`; nao ha bloqueio para continuidade. Depois de selecionar caminhos, o proximo passo incremental e descobrir candidatos XML no backend sem parsing fiscal.

Impacto: `docs/project_status.md`, `docs/tasks.md` e `docs/handoff.md` passam a apontar para T005 com Executor e skill `implement_task`.

## 2026-04-24 - Continuidade apos T005

Decisao: avancar para a task T006, implementando o parser fiscal inicial para NF-e, NFC-e e CT-e.

Justificativa: T005 foi aprovada em `docs/review_report.md`. Depois de descobrir candidatos XML, o proximo passo incremental e identificar os tres tipos fiscais do MVP e extrair os campos normalizados necessarios, sem classificar, deduplicar ou gerar Excel.

Impacto: `docs/project_status.md`, `docs/tasks.md` e `docs/handoff.md` passam a apontar para T006 com Executor e skill `implement_task`.

## 2026-04-24 - Continuidade apos T006

Decisao: avancar para a task T007, implementando deduplicacao por chave de acesso.

Justificativa: T006 foi aprovada com ressalvas em `docs/review_report.md`; a ressalva sobre tomador em CT-e nao bloqueia continuidade. Depois de normalizar documentos fiscais, o proximo passo incremental e remover duplicados silenciosamente antes da classificacao e do relatorio.

Impacto: `docs/project_status.md`, `docs/tasks.md` e `docs/handoff.md` passam a apontar para T007 com Executor e skill `implement_task`.

## 2026-04-24 - Continuidade apos T007

Decisao: avancar para a task T008, implementando classificacao por CNPJ.

Justificativa: T007 foi aprovada em `docs/review_report.md`. Depois de normalizar e deduplicar documentos, o proximo passo incremental e aplicar a regra de classificacao confirmada pelo usuario antes da geracao do Excel.

Impacto: `docs/project_status.md`, `docs/tasks.md` e `docs/handoff.md` passam a apontar para T008 com Executor e skill `implement_task`.

## 2026-04-24 - Continuidade apos T008

Decisao: avancar para a task T009, implementando geracao de Excel.

Justificativa: T008 foi aprovada em `docs/review_report.md`. Depois de classificar documentos, o proximo passo incremental e gerar o relatorio Excel com as tres abas antes da integracao final.

Impacto: `docs/project_status.md`, `docs/tasks.md` e `docs/handoff.md` passam a apontar para T009 com Executor e skill `implement_task`.

## 2026-04-24 - Nome da aba do Excel reduzido

Decisao: renomear a terceira aba de `Notas sem CNPJ identificado na operacao` para `Notas sem CNPJ identificado`.

Justificativa: o nome original tinha 40 caracteres, excedendo o limite de 31 caracteres imposto pelo formato `.xlsx` (Excel). O nome reduzido (27 caracteres) permanece descritivo e respeita a limitacao tecnica.

Impacto: `src-tauri/src/report.rs` usa o nome reduzido. O escopo funcional nao e afetado.

## 2026-04-24 - Continuidade apos T009

Decisao: avancar para a task T010, integrando o fluxo completo no backend.

Justificativa: T009 foi aprovada com ressalvas em `docs/review_report.md`. Com todos os modulos individuais prontos, o proximo passo e conectar tudo em um unico comando Tauri.

Impacto: `src-tauri/src/commands.rs`, `src-tauri/src/lib.rs` e `src/App.tsx` foram alterados na T010.

## 2026-04-24 - Regra de exibicao de partes por aba no relatorio

Decisao: o relatorio Excel deve exibir a contraparte relevante em cada aba, nao todas as partes indiscriminadamente.

Regras confirmadas pelo usuario:

- Aba **Entradas** (CNPJ do usuario e destinatario/tomador): exibir o **emitente/remetente** (quem enviou).
- Aba **Saidas** (CNPJ do usuario e emitente): exibir o **destinatario** (quem recebeu).
- Aba **Notas sem CNPJ identificado**: manter todas as colunas de partes (tomador, destinatario, remetente) para permitir identificacao manual.

Justificativa: na pratica contabil, ao registrar uma entrada o usuario precisa saber de quem veio a nota; ao registrar uma saida, precisa saber para quem foi. Exibir todas as partes em todas as abas gera confusao.

Impacto: `src-tauri/src/report.rs` deve ser ajustado para escrever colunas diferentes conforme a aba. Sera tratado em uma proxima task.

## 2026-04-24 - Reexecucao controlada da T012

Decisao: manter a T012 como nao iniciada no fluxo oficial e reescrever a persistencia local do zero.

Justificativa: a revisao identificou codigo de persistencia parcialmente implementado sem handoff, sem evidencia formal e fora da sequencia documental da pipeline. Para evitar herdar comportamento ambiguo, a T012 deve ser executada novamente a partir do handoff do Orchestrator, alinhada integralmente ao `scope.md`, `tasks.md` e `project_status.md`.

Impacto: o Executor deve tratar o codigo atual de persistencia como provisório e pode substitui-lo completamente durante a implementacao formal da T012.
## 2026-04-24 - Continuidade apos T012

Decisao: avancar para a task T013, implementando progresso real do processamento.

Justificativa: T012 foi aprovada pelo Reviewer. O requisito de progresso durante processamentos grandes continua pendente no `scope.md` e ja esta previsto na `architecture.md` e no proprio `decision_log.md` como comunicacao via eventos Tauri.

Impacto: `docs/tasks.md`, `docs/project_status.md` e `docs/handoff.md` passam a apontar para T013 com Executor e skill `implement_task`.

## 2026-04-24 - Continuidade apos T013

Decisao: avancar para a task T014, executando validacao real do MVP.

Justificativa: as implementacoes centrais do MVP foram concluidas e aprovadas ate a T013. O proximo passo mais seguro e validar o fluxo ponta a ponta com arquivos reais ou representativos antes de considerar empacotamento final ou novas melhorias.

Impacto: `docs/tasks.md`, `docs/project_status.md` e `docs/handoff.md` passam a apontar para T014 com Executor e skill `implement_task`.

## 2026-04-24 - Bloqueio identificado na validacao real do MVP

Decisao: priorizar uma task corretiva focada em importacao/classificacao de CT-e em ZIP antes de qualquer empacotamento final.

Justificativa: na validacao operacional realizada pelo usuario, o app passou nos demais cenarios, mas ao importar 3 arquivos ZIP contabilizou apenas 38 entradas, enquanto o conjunto esperado teria 71 documentos validos entre entradas e saidas. Os avisos mostraram varios `MissingRequiredField`, e o usuario separou XMLs reais em `exemplos_xml/` para orientar a investigacao.

Impacto: a proxima task passa a ser a T015, focada em alinhar parser/classificacao aos XMLs reais fornecidos, especialmente CT-e.

## 2026-04-24 - Ajustes de UX final e relatorio

Decisao: ajustar o fluxo para salvar o relatorio apenas ao final do processamento, mostrar mensagens em modal popup, ignorar XMLs de evento no aviso final e formatar datas do Excel em `dd/mm/aaaa`.

Justificativa: apos repetir os testes reais, o usuario confirmou que a base funcional estava boa, mas pediu refinamentos de uso e apresentacao para alinhar o app ao fluxo pratico de trabalho.

Impacto: a T016 passa a concentrar ajustes em `parser`, `commands`, `report` e frontend principal, sem alterar regras fiscais centrais.

## 2026-04-24 - Proximas prioridades apos validacao da T016

Decisao: priorizar primeiro uma task de identidade visual da aplicacao e, em seguida, uma task focada em refinamentos do Excel.

Justificativa: o usuario confirmou em teste manual que os ajustes da T016 funcionaram e pediu como proximos passos a aplicacao da identidade visual com ativos em `icones_e_logo/`, seguida de melhorias pontuais no relatorio Excel. Separar em duas tasks reduz risco e facilita validacao incremental.

Impacto: a proxima task passa a ser a T017 para atualizar icones, logo e paleta visual. A T018 fica reservada para consolidacao de CFOP por nota e formatacao do Excel.

## 2026-04-24 - Logo no relatorio Excel

Decisao: incluir a logo da empresa na planilha Excel, centralizada acima do cabecalho das colunas.

Justificativa: apos aprovar a T017, o usuario pediu que a identidade visual tambem apareca no relatorio final, sem alterar as colunas nem o fluxo fiscal.

Impacto: a T018 passa a incluir insercao de imagem no workbook alem dos ajustes de CFOP, bordas e formato contabil.
