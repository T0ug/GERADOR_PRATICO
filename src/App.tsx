import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open, save } from "@tauri-apps/plugin-dialog";
import { CnpjInput } from "./components/CnpjInput";
import { DescriptionMode, DescriptionOptions } from "./components/DescriptionOptions";
import { ImportSelector } from "./components/ImportSelector";
import { ProgressPanel } from "./components/ProgressPanel";
import { ResultDialog } from "./components/ResultDialog";
import logo from "./assets/logo.png";
import { getCnpjValidationState } from "./utils/cnpj";

type AppStatus = {
  message: string;
};

type AppConfig = {
  last_cnpj: string | null;
  last_import_dir: string | null;
  last_export_dir: string | null;
};

type ProgressStage = "Leitura" | "Processamento" | "Exportacao";

type ProgressUpdate = {
  stage: ProgressStage;
  current: number;
  total: number;
  percent: number;
  message: string;
};

function App() {
  const [backendStatus, setBackendStatus] = useState("Preparando aplicativo local...");
  const [cnpj, setCnpj] = useState("");
  const [xmlPaths, setXmlPaths] = useState<string[]>([]);
  const [zipPaths, setZipPaths] = useState<string[]>([]);
  const [descriptionMode, setDescriptionMode] = useState<DescriptionMode>("complete");
  const [wordLimit, setWordLimit] = useState(8);
  const [lastImportDir, setLastImportDir] = useState("");
  const [lastExportDir, setLastExportDir] = useState("");
  const [configLoaded, setConfigLoaded] = useState(false);
  const [modalTitle, setModalTitle] = useState("Resultado");
  const [message, setMessage] = useState("");
  const [isResultOpen, setIsResultOpen] = useState(false);
  const [progress, setProgress] = useState<ProgressUpdate>({
    stage: "Leitura",
    current: 0,
    total: 0,
    percent: 0,
    message: "Aguardando processamento.",
  });
  const cnpjValidationState = getCnpjValidationState(cnpj);
  const canGenerateReport = cnpjValidationState === "valid";
  const openResultDialog = openResultDialogFactory(setModalTitle, setMessage, setIsResultOpen);

  useEffect(() => {
    invoke<AppStatus>("app_status")
      .then((result) => setBackendStatus(result.message))
      .catch(() => setBackendStatus("Aplicativo iniciado. Backend indisponivel no navegador."));
  }, []);

  useEffect(() => {
    invoke<AppConfig>("get_config")
      .then((config) => {
        if (config.last_cnpj) {
          setCnpj(config.last_cnpj);
        }
        if (config.last_import_dir) {
          setLastImportDir(config.last_import_dir);
        }
        if (config.last_export_dir) {
          setLastExportDir(config.last_export_dir);
        }
      })
      .finally(() => setConfigLoaded(true));
  }, []);

  useEffect(() => {
    let disposed = false;

    const unlistenPromise = listen<ProgressUpdate>("report-progress", (event) => {
      if (!disposed) {
        setProgress(event.payload);
      }
    });

    return () => {
      disposed = true;
      void unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  useEffect(() => {
    if (!configLoaded || cnpjValidationState !== "valid") {
      return;
    }

    void persistConfig({
      last_cnpj: cnpj,
      last_import_dir: lastImportDir || null,
      last_export_dir: lastExportDir || null,
    });
  }, [cnpj, cnpjValidationState, configLoaded, lastExportDir, lastImportDir]);

  async function persistImportDir(selectedPath: string) {
    const parentDirectory = getParentDirectory(selectedPath);

    if (!parentDirectory) {
      return;
    }

    setLastImportDir(parentDirectory);
    await persistConfig({
      last_cnpj: cnpj || null,
      last_import_dir: parentDirectory,
      last_export_dir: lastExportDir || null,
    });
  }

  async function persistExportDir(selectedPath: string) {
    const parentDirectory = getParentDirectory(selectedPath);

    if (!parentDirectory) {
      return;
    }

    setLastExportDir(parentDirectory);
    await persistConfig({
      last_cnpj: cnpj || null,
      last_import_dir: lastImportDir || null,
      last_export_dir: parentDirectory,
    });
  }

  async function handleSelectFiles() {
    const selected = await open({
      multiple: true,
      directory: false,
      defaultPath: lastImportDir || undefined,
      filters: [{ name: "Arquivos fiscais", extensions: ["xml", "zip"] }],
    });

    if (!selected) {
      return;
    }

    const paths = Array.isArray(selected) ? selected : [selected];
    setXmlPaths(paths.filter((path) => path.toLowerCase().endsWith(".xml")));
    setZipPaths(paths.filter((path) => path.toLowerCase().endsWith(".zip")));
    await persistImportDir(paths[0]);
  }

  async function handleGenerateReportClick() {
    if (!canGenerateReport) {
      openResultDialog("Atenção", "Informe um CNPJ valido antes de gerar o relatorio.");
      return;
    }

    if (xmlPaths.length === 0 && zipPaths.length === 0) {
      openResultDialog("Atenção", "Selecione ao menos um arquivo XML ou ZIP.");
      return;
    }

    setProgress({
      stage: "Leitura",
      current: 0,
      total: 0,
      percent: 0,
      message: "Preparando processamento...",
    });

    try {
      type ReportResponse = {
        success: boolean;
        file_path: string;
        suggested_file_name: string;
        entradas_count: number;
        saidas_count: number;
        sem_cnpj_count: number;
        warnings: string[];
      };

      const result = await invoke<ReportResponse>("generate_report", {
        request: {
          cnpj,
          xml_paths: xmlPaths,
          zip_paths: zipPaths,
          export_path: "",
          description_mode: descriptionMode === "complete" ? "complete" : "limited",
          word_limit: descriptionMode === "limited" ? wordLimit : null,
        },
      });

      const selected = await save({
        defaultPath: buildSuggestedExportPath(lastExportDir, result.suggested_file_name),
        filters: [{ name: "Arquivo Excel", extensions: ["xlsx"] }],
      });

      if (!selected) {
        openResultDialog(
          "Salvamento cancelado",
          `O processamento foi concluido, mas o salvamento foi cancelado. O arquivo temporario ficou em: ${result.file_path}`,
        );
        return;
      }

      const savedPath = await invoke<string>("save_generated_report", {
        request: {
          temp_file_path: result.file_path,
          destination_path: selected,
        },
      });

      await persistExportDir(savedPath);

      let msg = `Relatorio gerado com sucesso em: ${result.file_path}. `;
      msg += `Entradas: ${result.entradas_count}, Saidas: ${result.saidas_count}, Sem CNPJ: ${result.sem_cnpj_count}.`;

      if (result.warnings.length > 0) {
        msg += ` Avisos: ${result.warnings.join("; ")}`;
      }
      msg = msg.replace(result.file_path, savedPath);

      setProgress({
        stage: "Exportacao",
        current: 1,
        total: 1,
        percent: 100,
        message: "Processamento finalizado.",
      });
      openResultDialog("Processamento concluido", msg);
    } catch (error) {
      setProgress((currentProgress) => ({
        ...currentProgress,
        message: "Processamento interrompido.",
      }));
      openResultDialog("Erro", `Erro ao gerar relatorio: ${error}`);
    }
  }

  return (
    <main className="app-shell">
        <header className="app-header">
        <div className="brand-block">
          <img className="brand-logo" src={logo} alt="Logo da aplicacao" />
          <div className="brand-copy">
            <h1>Gerador de relatorio de notas</h1>
          </div>
        </div>
        <div className="backend-status" role="status">
          {backendStatus}
        </div>
      </header>

      <section className="workspace" aria-label="Configuracao do relatorio">
        <div className="primary-column">
          <CnpjInput
            value={cnpj}
            validationState={cnpjValidationState}
            onChange={setCnpj}
          />

          <ImportSelector
            selectedPaths={[...xmlPaths, ...zipPaths]}
            onSelectFiles={handleSelectFiles}
            onClear={() => {
              setXmlPaths([]);
              setZipPaths([]);
            }}
          />

          <DescriptionOptions
            mode={descriptionMode}
            wordLimit={wordLimit}
            onModeChange={setDescriptionMode}
            onWordLimitChange={setWordLimit}
          />
        </div>

        <aside className="side-column">
          <ProgressPanel
            xmlCount={xmlPaths.length}
            zipCount={zipPaths.length}
            stageLabel={getProgressStageLabel(progress.stage)}
            message={progress.message}
            percent={progress.percent}
            current={progress.current}
            progressTotal={progress.total}
          />

          <button
            className="primary-action"
            type="button"
            disabled={!canGenerateReport}
            onClick={handleGenerateReportClick}
          >
            Gerar relatorio
          </button>
        </aside>
      </section>

      <ResultDialog
        open={isResultOpen}
        title={modalTitle}
        message={message}
        onClose={() => setIsResultOpen(false)}
      />
    </main>
  );
}

export default App;

function getParentDirectory(path: string): string {
  const normalized = path.replace(/[\\/]+$/, "");
  const lastSeparatorIndex = Math.max(normalized.lastIndexOf("\\"), normalized.lastIndexOf("/"));

  return lastSeparatorIndex >= 0 ? normalized.slice(0, lastSeparatorIndex) : "";
}

function buildSuggestedExportPath(lastExportDir: string, suggestedFileName: string): string {
  return lastExportDir ? `${lastExportDir}\\${suggestedFileName}` : suggestedFileName;
}

async function persistConfig(config: AppConfig): Promise<void> {
  try {
    await invoke("update_config", { config });
  } catch {
    // Persistencia falhou, mas a operacao principal deve continuar.
  }
}

function getProgressStageLabel(stage: ProgressStage): string {
  switch (stage) {
    case "Leitura":
      return "Leitura de arquivos";
    case "Processamento":
      return "Processamento fiscal";
    case "Exportacao":
      return "Exportacao do Excel";
    default:
      return "Processamento";
  }
}

function openResultDialogFactory(
  setModalTitle: (value: string) => void,
  setMessage: (value: string) => void,
  setIsResultOpen: (value: boolean) => void,
) {
  return (title: string, message: string) => {
    setModalTitle(title);
    setMessage(message);
    setIsResultOpen(true);
  };
}
