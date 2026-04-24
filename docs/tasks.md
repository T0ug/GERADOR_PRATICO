# Tasks

## Marcos concluidos

- Discovery concluido e confirmado pelo usuario.
- Arquitetura definida e confirmada pelo usuario.
- T001 concluida e aprovada com ressalvas.
- T002 concluida e aprovada com ressalvas.
- T003 concluida e aprovada.
- T004 concluida e aprovada com ressalvas.
- T005 concluida e aprovada.
- T006 concluida e aprovada com ressalvas.
- T007 concluida e aprovada.
- T008 concluida e aprovada.
- T009 concluida e aprovada com ressalvas.
- T010 concluida e aprovada.
- T011 concluida e aprovada.
- T012 concluida e aprovada.
- T013 concluida e aprovada.
- T014 concluida como validacao operacional e revelou bloqueio real na importacao via ZIP.
- T015 concluida e aprovada.
- T016 concluida e aprovada.
- T017 concluida e aprovada.

---

# Task

## Identificacao

- ID: T017
- Nome: Atualizar identidade visual e ativos da aplicacao
- Fase: Execution
- Agente responsavel: Executor

---

## Objetivo

Atualizar a identidade visual do aplicativo usando os ativos existentes em `icones_e_logo/`, incluindo a logo na pagina principal, a troca dos icones da aplicacao e a aplicacao de uma paleta baseada em vermelho, cinza claro e branco.

---

## Contexto

Depois da validacao manual da T016, o usuario confirmou que os ajustes funcionaram no uso real e priorizou agora a apresentacao visual do app. Os ativos ja existem localmente e devem servir de base para a identidade visual sem alterar o fluxo funcional.

---

## Entradas

- `docs/scope.md`
- `docs/architecture.md`
- `docs/decision_log.md`
- `docs/project_status.md`
- `src/App.tsx`
- `src/styles.css`
- `src-tauri/tauri.conf.json` se necessario
- `icones_e_logo/LOGO.png`
- demais arquivos em `icones_e_logo/`

---

## Escopo

- Atualizar os icones da aplicacao com base nos arquivos existentes em `icones_e_logo/`.
- Incluir a logo na pagina principal da aplicacao.
- Ajustar a paleta visual para vermelho, cinza claro e branco.
- Buscar a proporcao visual aproximada `60 30 10` entre as cores.
- Manter o layout responsivo e sem crescimento horizontal indevido.
- Remover o texto auxiliar no topo quando ele nao agregar valor visual.
- Simplificar a importacao para um unico botao de selecao que aceite `.xml` e `.zip` no mesmo dialogo.
- Ajustar a area de descricao para exibir ajuda por botao de interrogracao e mostrar o campo de limite apenas no modo limitado.

---

## Fora de escopo (CRITICO)

- Nao alterar parser, classificacao, persistencia ou geracao do Excel.
- Nao redesenhar completamente a estrutura da interface.
- Nao criar novos ativos graficos do zero se os fornecidos forem suficientes.

---

## Saidas esperadas

- Aplicacao com identidade visual atualizada.
- Logo visivel na interface principal.
- Iccones do app alinhados aos ativos fornecidos.
- Importacao mais simples com um unico ponto de entrada para arquivos fiscais.
- Ajuda visual na descricao dos itens sem poluir a tela.

---

## Criterios de aceite

- A logo aparece na interface principal sem quebrar o layout.
- Os icones da aplicacao passam a usar os ativos fornecidos ou derivados diretos deles.
- A paleta visual predominante fica em vermelho, cinza claro e branco.
- O texto "Aplicativo local para Windows" deixa de aparecer no topo.
- O card de importacao usa apenas `Selecionar arquivos` e `Limpar selecao`, aceitando `.xml` e `.zip` no mesmo dialogo.
- O botao de interrogracao na descricao mostra exemplos de formato completo e limitado.
- O campo de limite de palavras aparece apenas quando `Limitada` estiver selecionado.
- `npm run test` passa.
- `npm run build` passa.
- `cargo build` passa se houver alteracao em configuracao/ativos do Tauri.

---

## Dependencias

- T016 aprovada.

---

## Restricoes

- Reaproveitar os arquivos em `icones_e_logo/`.
- Manter o app totalmente offline.
- Preservar o fluxo operacional ja validado pelo usuario.

---

## Impacto no sistema

- Afeta principalmente frontend e ativos visuais da aplicacao.
- Pode tocar configuracao de icones do app no Tauri.

---

## Estrategia de implementacao

- Mapear os ativos existentes em `icones_e_logo/`.
- Integrar a logo na pagina principal respeitando o layout atual.
- Atualizar estilos globais para a nova paleta.
- Ajustar os icones da aplicacao no ponto minimo necessario.

---

## Plano de validacao

- Verificar a renderizacao da logo e da nova paleta na interface.
- Validar que o layout continua sem overflow horizontal.
- Executar builds e testes relevantes.

---

## Artefatos a atualizar

- `src/App.tsx`
- `src/styles.css`
- `src-tauri/tauri.conf.json` se necessario
- ativos/copias em `src-tauri/icons/` ou pasta equivalente
- `docs/handoff.md`
- `docs/project_status.md`
- `docs/tasks.md`

---

## Observacoes

- Os ativos base estao na pasta `icones_e_logo/`.
- Durante a execucao, o usuario pediu pequenos refinamentos adicionais de UI ainda dentro da mesma fronteira frontend/ativos.

---

## Status

- [ ] Nao iniciada
- [x] Em andamento
- [x] Concluida
- [ ] Bloqueada

---

# Task

## Identificacao

- ID: T018
- Nome: Refinar consolidacao de CFOP e formatacao do Excel
- Fase: Execution
- Agente responsavel: Executor

---

## Objetivo

Refinar o relatorio Excel para consolidar CFOPs repetidos dentro da mesma nota, aplicar bordas mais adequadas na planilha, formatar a coluna de valor em estilo contabil e incluir a logo acima do cabecalho.

---

## Contexto

Depois de validar o fluxo principal e definir a identidade visual como prioridade imediata, o usuario pediu um segundo refinamento concentrado no Excel. Os ajustes sao pontuais e pertencem ao modulo de relatorio, sem alterar regras fiscais centrais.

---

## Entradas

- `docs/scope.md`
- `docs/architecture.md`
- `docs/decision_log.md`
- `src-tauri/src/report.rs`
- exemplos de saida do relatorio observados pelo usuario
- `icones_e_logo/LOGO.png`

---

## Escopo

- Aglutinar CFOPs iguais na mesma nota antes de escrever a celula final.
- Aplicar bordas externas grossas no cabecalho.
- Aplicar todas as bordas nas linhas de dados do relatorio.
- Formatar a coluna de valor no estilo contabil.
- Incluir a logo na planilha, centralizada em relacao as colunas, acima do cabecalho.

---

## Fora de escopo (CRITICO)

- Nao alterar parser, classificacao ou importacao.
- Nao mudar estrutura de abas nem colunas alem do necessario para a formatacao.
- Nao alterar a interface React nesta task.

---

## Saidas esperadas

- CFOPs repetidos deixam de aparecer duplicados dentro da mesma nota.
- Cabecalho com borda externa grossa.
- Linhas de dados com todas as bordas.
- Coluna de valor em formato contabil.
- Logo visivel acima do cabecalho do relatorio.

---

## Criterios de aceite

- Uma nota com CFOP repetido nao repete o mesmo codigo na celula final.
- O Excel gerado aplica borda externa grossa no cabecalho.
- O Excel gerado aplica todas as bordas nos itens.
- A coluna de valor aparece em formato contabil.
- A logo aparece acima do cabecalho, centralizada em relacao as colunas da planilha.
- `cargo test` passa.
- `cargo build` passa.
- `npm run test` passa.
- `npm run build` passa.

---

## Dependencias

- T017 concluida e aprovada.
- T016 aprovada.

---

## Restricoes

- Manter o processamento pesado no backend Rust.
- Corrigir apenas o necessario no modulo de relatorio.
- Preservar o nome e a estrutura funcional atual das abas.

---

## Impacto no sistema

- Afeta o modulo `report.rs` e seus testes.
- Nao deve afetar fluxo de importacao, parser ou UX do app.

---

## Estrategia de implementacao

- Ajustar a normalizacao de CFOPs no momento de montar a celula do relatorio.
- Revisar os estilos do workbook no `report.rs`.
- Cobrir as mudancas com testes automatizados focados.

---

## Plano de validacao

- Gerar planilha de teste com CFOP repetido e verificar consolidacao.
- Validar estilos do workbook por teste e por inspecao de saida gerada.
- Reexecutar a bateria de build e testes.

---

## Artefatos a atualizar

- `src-tauri/src/report.rs`
- testes Rust relacionados ao relatorio
- ativo de imagem utilizado pelo relatorio, se necessario
- `docs/handoff.md`
- `docs/project_status.md`
- `docs/tasks.md`

---

## Observacoes

- O usuario mostrou exemplos visuais para borda externa grossa, todas as bordas e formato contabil.

---

## Status

- [ ] Nao iniciada
- [x] Em andamento
- [x] Concluida
- [ ] Bloqueada

---

# Task

## Identificacao

- ID: T011
- Nome: Ajustar colunas do relatorio por aba
- Fase: Execution
- Agente responsavel: Executor

---

## Objetivo

Ajustar o modulo `report.rs` para que cada aba do relatorio Excel exiba apenas a contraparte relevante conforme decisao registrada no `decision_log.md`.

---

## Contexto

O usuario identificou que o relatorio exibe todas as colunas de partes (Tomador, Destinatario, Remetente) em todas as abas, o que nao faz sentido contabil. A regra de negocio confirmada:

- Entradas: exibir emitente/remetente (quem enviou).
- Saidas: exibir destinatario (quem recebeu).
- Sem CNPJ identificado: manter todas as colunas para identificacao manual.

---

## Escopo

- Ajustar `report.rs` para que colunas variem conforme a aba.
- Aba Entradas: colunas Data, Numero da nota, Valor, CFOP, Descricao dos itens, Emitente, Remetente.
- Aba Saidas: colunas Data, Numero da nota, Valor, CFOP, Descricao dos itens, Destinatario.
- Aba Sem CNPJ identificado: manter colunas completas (Tomador, Destinatario, Remetente).
- Atualizar o teste existente se necessario.

---

## Fora de escopo (CRITICO)

- Nao alterar o pipeline de integracao (commands.rs).
- Nao alterar o frontend.
- Nao implementar persistencia ou progresso.

---

## Saidas esperadas

- Relatorio com colunas diferenciadas por aba.
- Testes atualizados.

---

## Criterios de aceite

- `cargo test` passa.
- `cargo build` passa.
- `npm run build` passa.
- Aba Entradas mostra emitente/remetente, nao destinatario.
- Aba Saidas mostra destinatario, nao emitente.
- Aba Sem CNPJ mostra todas as partes.

---

## Status

- [ ] Nao iniciada
- [x] Em andamento
- [x] Concluida
- [ ] Bloqueada

---

# Task

## Identificacao

- ID: T012
- Nome: Implementar persistencia local
- Fase: Execution
- Agente responsavel: Executor

---

## Objetivo

Implementar a persistencia do ultimo CNPJ usado, ultima pasta de importacao e ultima pasta de exportacao em um arquivo de configuracao local (`config.json`), permitindo que esses dados sejam carregados ao iniciar o app.

---

## Contexto

Conforme o `scope.md`, o app deve lembrar certas escolhas do usuario para melhorar a UX em execucoes repetitivas. O modulo `config.rs` ja possui a struct base.

---

## Escopo

- Implementar funcoes de `load_config` e `save_config` no `src-tauri/src/config.rs`.
- Salvar o arquivo no diretório de dados do aplicativo (ou pasta local provisoria).
- Criar comandos Tauri `get_config` e `update_config`.
- Frontend: carregar configuracoes ao montar o componente principal.
- Frontend: disparar atualizacoes de config ao selecionar pastas ou validar CNPJ.

---

## Fora de escopo (CRÍTICO)

- Nao persistir limite de palavras ou modo de descricao (proibido pelo scope.md).
- Nao implementar configuracoes de seguranca.

---

## Saídas esperadas

- Arquivo `config.json` gerado e lido corretamente.
- Interface inicia com valores da ultima execucao.

---

## Criterios de aceite

- Ao fechar e abrir o app, o CNPJ e caminhos selecionados anteriormente (as pastas pai) devem ser mantidos.
- Nao ha erros ao tentar carregar config inexistente (deve retornar default).
- `cargo test` e `npm build` continuam passando.

---

## Status

- [ ] Nao iniciada
- [ ] Em andamento
- [x] Concluida
- [ ] Bloqueada

---

# Task

## Identificacao

- ID: T016
- Nome: Ajustar UX final de processamento e formato do relatorio
- Fase: Execution
- Agente responsavel: Executor

---

## Objetivo

Melhorar o fluxo final de processamento para ignorar XMLs de evento sem aviso final, formatar a data da planilha em `dd/mm/aaaa`, mover a escolha do local de salvamento para o fim do processamento e apresentar mensagens em modal popup coerente com o visual do app.

---

## Contexto

Depois da validacao operacional e da correcao de CT-e na T015, o usuario pediu ajustes de UX e apresentacao para deixar o fluxo mais aderente ao uso real. Os pontos principais envolvem aviso final, formato de data no Excel e interacao de salvamento do relatorio.

---

## Entradas

- `docs/scope.md`
- `docs/architecture.md`
- `docs/decision_log.md`
- `src-tauri/src/parser.rs`
- `src-tauri/src/commands.rs`
- `src-tauri/src/report.rs`
- `src/App.tsx`
- `src/components/ResultDialog.tsx`
- `src/styles.css`
- `exemplos_xml/evento.xml`

---

## Escopo

- Ignorar XMLs de evento no processamento sem listar esses arquivos no aviso final.
- Ajustar a coluna de data do Excel para exibir apenas `dia/mes/ano`.
- Alterar o fluxo para escolher onde salvar o relatorio apenas ao final do processamento.
- Trocar a mensagem estatica da pagina por modal popup com o estilo da aplicacao.
- Ajustar o layout para evitar crescimento horizontal da pagina.

---

## Fora de escopo (CRITICO)

- Nao alterar regras fiscais fora do necessario para ignorar evento.
- Nao mudar classificacao contabil.
- Nao reintroduzir card de exportacao antes do processamento.
- Nao mexer em persistencia alem do necessario para continuar lembrando a ultima pasta de exportacao.

---

## Saidas esperadas

- Eventos ignorados sem poluir o aviso final.
- Data do Excel no formato brasileiro.
- Save dialog aberto somente depois do processamento.
- Mensagens exibidas em modal.
- Layout sem estourar horizontalmente.

---

## Criterios de aceite

- `evento.xml` e ignorado sem aparecer no aviso final.
- Datas do Excel saem em `dd/mm/aaaa`.
- O usuario nao precisa escolher o destino antes de processar.
- O save dialog aparece no final com sugestao de nome adequada.
- A mensagem final aparece em modal popup.
- `cargo test`, `cargo build`, `npm run test` e `npm run build` passam.

---

## Dependencias

- T015 aprovada.

---

## Restricoes

- Corrigir apenas o necessario para o fluxo pedido pelo usuario.
- Manter o processamento pesado no backend Rust.
- Manter a interface responsiva e sem crescimento horizontal.

---

## Impacto no sistema

- Afeta parser, commands, report e frontend principal.
- Ajusta o contrato de exportacao entre backend e UI.

---

## Estrategia de implementacao

- Silenciar XMLs de evento no parser/commands.
- Formatar datas no report.
- Gerar relatorio primeiro e abrir o save dialog ao final.
- Substituir o painel de mensagem por modal.
- Endurecer o CSS contra overflow horizontal.

---

## Plano de validacao

- Executar testes e builds de Rust e frontend.
- Validar `evento.xml` como ignorado.
- Validar o formato de data por teste automatizado.
- Validar o novo fluxo de salvamento pelo codigo e build da UI.

---

## Artefatos a atualizar

- `src-tauri/src/parser.rs`
- `src-tauri/src/commands.rs`
- `src-tauri/src/report.rs`
- `src-tauri/src/lib.rs`
- `src/App.tsx`
- `src/components/ResultDialog.tsx`
- `src/styles.css`
- `docs/handoff.md`
- `docs/project_status.md`
- `docs/tasks.md`

---

## Observacoes

- O usuario forneceu `exemplos_xml/evento.xml` como referencia de documento a ignorar.

---

## Status

- [ ] Nao iniciada
- [ ] Em andamento
- [x] Concluida
- [ ] Bloqueada

---

# Task

## Identificacao

- ID: T014
- Nome: Executar validacao real do MVP
- Fase: Execution
- Agente responsavel: Executor

---

## Objetivo

Executar um teste real do MVP com arquivos representativos para validar o fluxo ponta a ponta do aplicativo, incluindo importacao, processamento, geracao do Excel, avisos e progresso.

---

## Contexto

As tasks de implementacao centrais do MVP ja foram concluídas e aprovadas ate a T013. O proximo passo logico e validar o comportamento real do aplicativo contra o escopo definido, antes de considerar empacotamento final ou ajustes de refinamento.

---

## Entradas

- `docs/scope.md`
- `docs/architecture.md`
- `docs/implementation_plan.md`
- `docs/decision_log.md`
- `docs/review_report.md`
- build atual do app em ambiente local
- conjunto de arquivos XML/ZIP representativos do uso real

---

## Escopo

- Executar teste ponta a ponta com XMLs validos de NF-e, NFC-e e CT-e quando disponiveis.
- Executar teste com importacao por ZIP, incluindo XML em pasta interna.
- Executar teste com arquivos invalidos ou fora do escopo para verificar aviso final.
- Executar teste de ausencia total de dados validos para confirmar que nao gera Excel e mostra aviso.
- Executar teste com volume representativo para observar responsividade e progresso.
- Verificar se o Excel gerado atende as regras funcionais principais do escopo.
- Registrar evidencias objetivas do teste e qualquer desvio encontrado.

---

## Fora de escopo (CRITICO)

- Nao alterar escopo do produto.
- Nao iniciar empacotamento final do executavel portatil nesta task.
- Nao fazer refactors amplos "aproveitando o teste".
- Nao corrigir automaticamente bugs fora de uma task formal derivada dos achados.

---

## Saidas esperadas

- Evidencia documentada de teste real do MVP.
- Lista clara de comportamentos aprovados e problemas encontrados, se houver.
- Confirmacao se o MVP esta apto ou nao para seguir para empacotamento/refino final.

---

## Criterios de aceite

- Existe evidencia reproduzivel do teste real do fluxo ponta a ponta.
- O comportamento observado e comparado com o `scope.md`.
- O resultado do teste deixa claro se o MVP esta apto ou se existem bloqueios reais.
- Achados relevantes ficam documentados para gerar novas tasks, se necessario.

---

## Dependencias

- T010 aprovada.
- T011 aprovada.
- T012 aprovada.
- T013 aprovada.

---

## Restricoes

- Usar `docs/` como fonte de verdade.
- Tratar esta task como validacao operacional, nao como nova rodada de implementacao.
- Se houver bug, documentar com precisao em vez de expandir escopo sem autorizacao.

---

## Impacto no sistema

- Pode revelar bugs reais ainda nao cobertos por testes automatizados.
- Pode gerar novas tasks pequenas de correcao antes do empacotamento final.

---

## Estrategia de implementacao

- Preparar um roteiro de teste baseado no escopo.
- Executar os cenarios principais com arquivos reais ou representativos.
- Registrar o resultado de cada cenario e sintetizar conclusao operacional.

---

## Plano de validacao

- Validar importacao direta de XML.
- Validar importacao via ZIP.
- Validar geracao do Excel.
- Validar avisos finais.
- Validar ausencia total de dados validos.
- Validar exibicao de progresso em execucao maior.

---

## Artefatos a atualizar

- `docs/handoff.md`
- `docs/project_status.md`
- `docs/tasks.md`
- `docs/review_report.md` se aplicavel ao fechamento
- artefato de evidencias do teste, se criado durante a execucao

---

## Observacoes

- Esta e a primeira task explicitamente voltada a teste real do MVP.
- Caso os arquivos reais de validacao nao estejam disponiveis, a task deve registrar bloqueio com clareza.

---

## Status

- [ ] Nao iniciada
- [ ] Em andamento
- [x] Concluida
- [ ] Bloqueada

---

# Task

## Identificacao

- ID: T015
- Nome: Investigar e corrigir importacao/classificacao de CT-e em ZIP
- Fase: Execution
- Agente responsavel: Executor

---

## Objetivo

Investigar por que a importacao de XMLs em lote via ZIP esta deixando de contabilizar documentos validos, com foco especial em CT-e, e ajustar o parser/classificacao conforme os campos reais presentes nos XMLs de exemplo.

---

## Contexto

Na validacao real do MVP, o usuario informou que o app passou nos demais cenarios, mas falhou ao importar 3 arquivos ZIP: o resultado mostrou apenas 38 linhas de entradas, enquanto o conjunto esperado teria 71 documentos validos entre entradas e saidas. Os avisos exibidos incluem varios `MissingRequiredField` e arquivos `evento` fora do escopo.

O usuario indicou suspeita de problema com CT-e e disponibilizou exemplos reais em `exemplos_xml/entrada.xml` e `exemplos_xml/CTE SAÍDA.xml`.

---

## Entradas

- `docs/scope.md`
- `docs/architecture.md`
- `docs/decision_log.md`
- `docs/review_report.md`
- `src-tauri/src/importer.rs`
- `src-tauri/src/parser.rs`
- `src-tauri/src/classifier.rs`
- `src-tauri/src/commands.rs`
- `exemplos_xml/entrada.xml`
- `exemplos_xml/CTE SAÍDA.xml`

---

## Escopo

- Inspecionar os XMLs reais em `exemplos_xml/`.
- Comparar os campos reais desses XMLs com as regras hoje implementadas no parser e na classificacao.
- Identificar a causa dos avisos `MissingRequiredField` nos documentos que deveriam ser validos.
- Ajustar o codigo para aceitar corretamente os XMLs reais dentro do escopo do MVP, com foco em CT-e.
- Garantir que documentos `evento` continuem fora do escopo e apenas gerem aviso.
- Adicionar ou atualizar testes cobrindo os formatos reais analisados.

---

## Fora de escopo (CRITICO)

- Nao alterar o escopo fiscal do produto para documentos alem de NF-e, NFC-e e CT-e.
- Nao mexer em persistencia local ou progresso.
- Nao iniciar empacotamento final.
- Nao redesenhar a interface.

---

## Saidas esperadas

- Parser/importacao ajustados para os XMLs reais fornecidos.
- Testes automatizados cobrindo os casos identificados.
- Reducao ou eliminacao dos falsos `MissingRequiredField` nos documentos validos analisados.

---

## Criterios de aceite

- Os XMLs de exemplo validos em `exemplos_xml/` sao interpretados corretamente.
- CT-e de exemplo entra na classificacao esperada quando compativel com o CNPJ informado.
- Arquivos `evento` continuam sendo ignorados com aviso, sem quebrar o fluxo.
- `cargo test` passa.
- `cargo build` passa.
- `npm run test` passa.
- `npm run build` passa.

---

## Dependencias

- T010 aprovada.
- T013 aprovada.
- Evidencia operacional levantada pelo usuario na validacao real do MVP.

---

## Restricoes

- Tratar `docs/` como fonte de verdade.
- Corrigir apenas o necessario para alinhar parser/importacao aos XMLs reais fornecidos.
- Se surgirem variacoes adicionais fora dos exemplos e sem cobertura clara, registrar em handoff em vez de expandir demais a task.

---

## Impacto no sistema

- Afeta principalmente parser, classificacao e possivelmente a forma como avisos sao gerados no fluxo principal.
- Pode desbloquear a validacao real completa do MVP.

---

## Estrategia de implementacao

- Ler os XMLs reais fornecidos.
- Mapear os campos usados hoje versus os campos presentes nesses arquivos.
- Corrigir extracao/normalizacao/classificacao no ponto minimo necessario.
- Cobrir os casos com testes automatizados antes de encerrar.

---

## Plano de validacao

- Validar por testes automatizados os XMLs de exemplo.
- Reexecutar `cargo test`, `cargo build`, `npm run test` e `npm run build`.
- Registrar em handoff como os XMLs reais passaram a ser interpretados.

---

## Artefatos a atualizar

- `src-tauri/src/parser.rs`
- `src-tauri/src/classifier.rs` se necessario
- `src-tauri/src/commands.rs` se necessario
- testes Rust relacionados
- `docs/handoff.md`
- `docs/project_status.md`
- `docs/tasks.md`

---

## Observacoes

- O diretorio `exemplos_xml/` foi criado pelo usuario especificamente para esta investigacao.
- O objetivo imediato e alinhar o codigo aos XMLs reais do usuario, nao generalizar todas as variacoes possiveis de CT-e.

---

## Status

- [ ] Nao iniciada
- [ ] Em andamento
- [x] Concluida
- [ ] Bloqueada

---

# Task

## Identificacao

- ID: T014
- Nome: Executar validacao real do MVP
- Fase: Execution
- Agente responsavel: Executor

---

## Objetivo

Executar um teste real do MVP com arquivos representativos para validar o fluxo ponta a ponta do aplicativo, incluindo importacao, processamento, geracao do Excel, avisos e progresso.

---

## Contexto

As tasks de implementacao centrais do MVP ja foram concluidas e aprovadas ate a T013. O proximo passo logico e validar o comportamento real do aplicativo contra o escopo definido, antes de considerar empacotamento final ou ajustes de refinamento.

---

## Entradas

- `docs/scope.md`
- `docs/architecture.md`
- `docs/implementation_plan.md`
- `docs/decision_log.md`
- `docs/review_report.md`
- build atual do app em ambiente local
- conjunto de arquivos XML/ZIP representativos do uso real

---

## Escopo

- Executar teste ponta a ponta com XMLs validos de NF-e, NFC-e e CT-e quando disponiveis.
- Executar teste com importacao por ZIP, incluindo XML em pasta interna.
- Executar teste com arquivos invalidos ou fora do escopo para verificar aviso final.
- Executar teste de ausencia total de dados validos para confirmar que nao gera Excel e mostra aviso.
- Executar teste com volume representativo para observar responsividade e progresso.
- Verificar se o Excel gerado atende as regras funcionais principais do escopo.
- Registrar evidencias objetivas do teste e qualquer desvio encontrado.

---

## Fora de escopo (CRITICO)

- Nao alterar escopo do produto.
- Nao iniciar empacotamento final do executavel portatil nesta task.
- Nao fazer refactors amplos "aproveitando o teste".
- Nao corrigir automaticamente bugs fora de uma task formal derivada dos achados.

---

## Saidas esperadas

- Evidencia documentada de teste real do MVP.
- Lista clara de comportamentos aprovados e problemas encontrados, se houver.
- Confirmacao se o MVP esta apto ou nao para seguir para empacotamento/refino final.

---

## Criterios de aceite

- Existe evidencia reproduzivel do teste real do fluxo ponta a ponta.
- O comportamento observado e comparado com o `scope.md`.
- O resultado do teste deixa claro se o MVP esta apto ou se existem bloqueios reais.
- Achados relevantes ficam documentados para gerar novas tasks, se necessario.

---

## Dependencias

- T010 aprovada.
- T011 aprovada.
- T012 aprovada.
- T013 aprovada.

---

## Restricoes

- Usar `docs/` como fonte de verdade.
- Tratar esta task como validacao operacional, nao como nova rodada de implementacao.
- Se houver bug, documentar com precisao em vez de expandir escopo sem autorizacao.

---

## Impacto no sistema

- Pode revelar bugs reais ainda nao cobertos por testes automatizados.
- Pode gerar novas tasks pequenas de correcao antes do empacotamento final.

---

## Estrategia de implementacao

- Preparar um roteiro de teste baseado no escopo.
- Executar os cenarios principais com arquivos reais ou representativos.
- Registrar o resultado de cada cenario e sintetizar conclusao operacional.

---

## Plano de validacao

- Validar importacao direta de XML.
- Validar importacao via ZIP.
- Validar geracao do Excel.
- Validar avisos finais.
- Validar ausencia total de dados validos.
- Validar exibicao de progresso em execucao maior.

---

## Artefatos a atualizar

- `docs/handoff.md`
- `docs/project_status.md`
- `docs/tasks.md`
- `docs/review_report.md` se aplicavel ao fechamento
- artefato de evidencias do teste, se criado durante a execucao

---

## Observacoes

- Esta e a primeira task explicitamente voltada a teste real do MVP.
- O usuario executou a validacao operacional e informou um bloqueio real na importacao via 3 ZIPs, com suspeita concentrada em CT-e.
- Caso os arquivos reais de validacao nao estejam disponiveis, a task deve registrar bloqueio com clareza.

---

## Status

- [ ] Nao iniciada
- [ ] Em andamento
- [x] Concluida
- [ ] Bloqueada
