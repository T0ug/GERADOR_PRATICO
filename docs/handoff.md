# Handoff Operacional

## Origem

Discovery

## Destino

Architect

## Workflow

`start_project.md` / etapa de arquitetura aplicada como evolucao de escopo

## Skill a utilizar

design_architecture

## Task a executar

- ID: T024
- Nome: Atualizar arquitetura para cache em sessao, CPF/CNPJ e cancelamento

## Contexto

O usuario confirmou a nova evolucao apos o fechamento operacional da funcionalidade de GTINS.

Resumo confirmado:

- cache apenas enquanto o app estiver aberto;
- cache acumulativo por hash do conteudo de cada XML;
- XML interno de ZIP tambem deve ser identificado pelo hash do conteudo do XML;
- arquivos ja processados devem ser reaproveitados mesmo em novos relatorios;
- arquivos novos ou diferentes devem ser os unicos processados novamente;
- ao trocar CPF/CNPJ, o app pode reaproveitar parsing/importacao, mas deve reclassificar para o novo documento informado;
- gerar novo Excel mudando opcoes de descricao, limite e GTINS nao deve reprocessar XML;
- deve haver suporte a CPF alem de CNPJ no campo atual;
- botao de gerar relatorio deve ficar desabilitado durante processamento;
- deve haver botao de cancelar processamento;
- cancelamento preserva no cache os XMLs ja processados com sucesso;
- ao tentar fechar o app, exibir modal avisando que o cache sera perdido e sera necessario processar os arquivos novamente;
- remover persistencia em `config.json`;
- ultimo CPF/CNPJ e pastas devem ficar apenas em memoria.

## Objetivo

Atualizar a arquitetura para orientar a implementacao segura dessa evolucao, sem codificar ainda.

## Artefatos obrigatorios para leitura

- `docs/idea.md`
- `docs/scope.md`
- `docs/non_goals.md`
- `docs/decision_log.md`
- `docs/implementation_plan.md`
- `docs/tasks.md`
- `docs/architecture.md`
- codigo atual em `src-tauri/src/` e `src/` quando necessario para entender limites.

## Fora de escopo

- Nao implementar codigo.
- Nao criar cache persistente.
- Nao armazenar dados dentro do executavel.
- Nao criar banco de dados.
- Nao alterar regras fiscais alem do suporte CPF/CNPJ ja confirmado.

## Saidas esperadas

- `docs/architecture.md` atualizado.
- `docs/decision_log.md` atualizado se houver decisoes arquiteturais adicionais.
- `docs/tasks.md` atualizado com tasks de execucao derivadas.
- `docs/project_status.md` e `docs/handoff.md` preparados para a proxima etapa.
