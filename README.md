# OpenAlex Retractions & Corrections Analysis

Questo progetto interroga la API di OpenAlex per analizzare, per diverse macroaree disciplinari, l’incidenza di:

- articoli ritrattati;
- articoli che contengono la keyword `correction`;
- articoli che contengono la keyword `erratum`;
- proxy complessiva `correction + erratum`.

L’obiettivo è costruire una base dati utile per un elaborato metodologico sul possibile impatto del periodo Covid sulla qualità della produzione scientifica.

## Obiettivo dell’analisi

Il progetto confronta tre finestre temporali:

| Periodo | Intervallo |
|---|---|
| Pre-Covid | 2016-2017 |
| Covid | 2020-2021 |
| Post-Covid | 2024-2025 |

Per ogni periodo vengono analizzate otto macroaree disciplinari:

1. Medicine & Health Sciences
2. Life Sciences
3. Physical Sciences
4. Engineering
5. Computer Science
6. Social Sciences
7. Economics & Management
8. Humanities

## Indicatori calcolati

Per ogni combinazione disciplina-periodo il programma calcola:

| Indicatore | Descrizione |
|---|---|
| Totale articoli | Numero totale di articoli OpenAlex nella categoria e nel periodo |
| Ritrattati | Numero di articoli con `is_retracted = true` |
| % ritrattati | Ritrattati / totale articoli × 100 |
| Correction count | Numero di articoli che contengono `correction` in titolo o abstract |
| % correction | Correction count / totale articoli × 100 |
| Erratum count | Numero di articoli che contengono `erratum` in titolo o abstract |
| % erratum | Erratum count / totale articoli × 100 |
| Correction + Erratum proxy | Somma di `correction` ed `erratum` |
| % correction + erratum proxy | Proxy complessiva / totale articoli × 100 |

## Nota metodologica

Le ritrattazioni sono misurate tramite il campo strutturato `is_retracted` di OpenAlex, quindi rappresentano l’indicatore più robusto dell’analisi.

Le correzioni, invece, sono approssimate tramite ricerca testuale nei titoli e negli abstract usando le keyword `correction` ed `erratum`. Questo significa che il proxy sulle correzioni è esplorativo e non deve essere interpretato come conteggio esaustivo di tutte le correzioni editoriali.

In particolare:

- la keyword `correction` può intercettare anche articoli che parlano di correzioni statistiche, tecniche o metodologiche;
- `correction + erratum` è una somma non deduplicata;
- i dati OpenAlex rappresentano lo stato attuale del database, non necessariamente lo stato storico disponibile nel momento della pubblicazione.

## Output generati

Il programma crea automaticamente una cartella `output/` contenente:

```text
output/
├── openalex_ritrattazioni_correzioni_2016_2017_2020_2021_2024_2025.csv
└── openalex_ritrattazioni_correzioni_report_2016_2017_2020_2021_2024_2025.html

````md
## Avvio con Docker

Il progetto può essere avviato tramite Docker senza installare Rust localmente.

Build dell’immagine:

```bash
docker build -t openalex-rust .
````

Esecuzione del container:

```bash
docker run --rm -v "${PWD}/output:/app/output" openalex-rust
```


```bash
docker run --rm -v ${PWD}/output:/app/output openalex-rust
```

I risultati verranno salvati nella cartella locale:

```text
output/
```
