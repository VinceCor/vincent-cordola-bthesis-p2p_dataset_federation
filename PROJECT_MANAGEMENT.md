# Project Management - Peer-to-Peer (P2P) Dataset Federation

**Vincent Cordola** | ISC-ID-26-6 | HES-SO Valais-Wallis x McGill Montreal  
**Supervisors**: Prof. Oscar Esteban | Prof. Jean-Baptiste Poline | Dr. Nikhil Bhagwat  
**Period**: May 11, 2026 - August 12, 2026

This document provides an overview of the project. It includes the project plan, potential project risks, meeting schedules, and a work journal for daily tracking of the project.

This document will evolve as the project progresses, and the initial information was taken directly from the document: [Donnée du travail de bachelor](https://isc-hei.github.io/isc-typst-tb-descriptions/)

## Table of Contents
1. [Planning](#1-Planning)
2. [Risk Register](#2-risk-register)
3. [Reviews and Meetings](#3-reviews-and-meetings)
4. [Work Journal](#4-work-journal)

## 1. Planning

### 1.1 Deliverables

| Weeks | Deliverable | Target date | Status |
|---|-------------|-------------|--------|
| W1-W2 | **1. Onboarding & Technical Design** | 11.05 - 24.05 |  |
|  | Report setup, GitHub page, planning | 24.05 | Done |
|  | Dataset layout conventions defined | 24.05 | In progress |
|  | logical topology (to get an overview of the project) | 24.05 | In progress |
|  | Rust & iroh crash-course completed | 24.05 | In progress |
| W3-W6 | **2. Core P2P sharing prototype** | 25.05 - 21.06 | |
|  | Rust MVP: (i) join an ad-hoc iroh network, (ii) advertise Parquet files, and (iii) fetch Parquet files from peers. | 21.06 | Upcoming |
|  | Validation on a Docker Compose network of 3–5 containers. | 21.06 | Upcoming |
| W7-W10 | **3. Python/Jupyter integration and connectivity validation** | 22.06 - 19.07 | |
|  | Python client layer: a unified Jupyter interface ensuring required files are present locally via on-demand retrieval. | 19.07 | Upcoming |
|  | Migrate the testbed to virtual machines with separate network stacks to exercise NAT traversal and relay fallback for the first time. | 19.07 | Upcoming |
|  | Produce a first working notebook that discovers the network dataset, loads it into standard dataframe tooling, and runs queries spanning multiple peers. | 19.07 | Upcoming |
| W11-W13 | **4. Robustness and evaluation in a 2–5 machine testbed** | 20.07 - 09.08 | |
|  | Test behavior under realistic conditions: peers joining/leaving, partial availability, varying bandwidth. | 09.08 | Upcoming |
|  | Improve reliability (resume partial transfers, cache behavior, error handling) and document known limitations. | 09.08 | Upcoming |
|  | Optional (if feasible): evaluate byte-range or partial reads to reduce transfers for larger Parquet artifacts. | 09.08 | Upcoming |
| W14 | **5. Packaging, documentation, and final demonstration** | 10.08 - 12.08 | |
|  | Finalize the notebook as the primary demonstration artifact. | 12.08 | Upcoming |
|  | Produce setup scripts/instructions for reproducing the demo on 2–5 machines | 12.08 | Upcoming |
|  | Write a short evaluation report summarizing results, constraints, and recommended next steps for downstream integration work. | 12.08 | Upcoming |
|  | **Final code submitted** | **25/08/2026 12:00** | Upcoming |
|  | **Final report submitted** | **25/08/2026 12:00** | Upcoming |
|  | **Executive summary** | **25/08/2026** | Upcoming |
|  | **Poster** | **25/08/2026** | Upcoming |

### 1.2 Key Milestones

| Milestone | Date |
|-----------|------|
| Bachelor thesis start | May 11, 2026 |
| Midterm presentation | June 3, 2026 |
| Executive summary | July 24, 2026 |
| Oral defence | Weeks of August 17 - 25, 2026 |
| Final code submission | August 25, 2026 - 12:00 |
| Final report submission | August 25, 2026 - 12:00 |
| Poster | August 25, 2026 |
| Poster exhibition | August 28, 2026 - HEI |
| Pitch challenge | August 31, 2026 - Monthey |

## 2. Risk Register
| Risk | Probability | Impact | Mitigation measures |
|------|-------------|--------|---------------------|
| Unstable iroh API / breaking changes | Medium | High | Pin dependency versions early, use only well-documented primitives (blobs, gossip) |
| Multi-machine testbed setup difficulties (heterogeneous network) | Medium | Medium | Phase 2 on Docker first, migrate to VMs in phase 3 only after Docker validation |
| Rust + Python integration learning curve | High | Medium | Keep Rust component minimal (thin node agent), focus effort on Python layer (PyArrow/DuckDB) |
| Parquet footer-first read pattern incompatible with iroh-blobs range requests | Medium | Low | Treated as optional, fallback = full file download into local cache |


## 3. Reviews and Meetings 
### Meeting - [DATE]

**Participants:** Vincent Cordola - x   
**Type:**   
**Duration:** _ min

**Discussion points**
- _

**Decisions made**
- _

<!-- Reviews and Meetings template
**Participants:** Vincent Cordola - x   
**Type:**   
**Duration:** _ min

**Discussion points**
- _

**Decisions made**
- _
-->

## 4. Work Journal

### 11/05/2026 - 1. Report setup, GitHub page, planning

**Work done:**
- Reviewed the criteria for the bachelor's thesis using the information provided on ISC Learn.
- Created the project repository and also created the page for the report (template ISC bthesis on Typst).
- Initial review of MRIQC API to get a first idea of how to define the dataset layout convetions.

**Decisions / Observations / Blockers:**
- Create the project management file for this project is important for better monitoring

**Next steps:**
- Create the project management

---

### 12/05/2026 - Report setup, GitHub page, planning

**Work done:**
- Creation of the first version for project management markdown
- Research on MRIQC to subsequently establish dataset layout conventions
- Research on the overall project structure to subsentquently create an initial logical topology and gain a better understanding of each project.

**Decisions / Observations / Blockers:**

**Next steps:**
- Do the Dataset layout conventions
- Do the logical topology

---

### 13/05/2026 - Deliverable

**Work done:**
- Research on tools for designing the dataset layout
    - [MRIQC](https://mriqc.readthedocs.io/en/latest/reports/group.html)
    - [DuckDB Parquet support](https://duckdb.org/docs/current/data/parquet/overview)
    - [pyarrow write table API, parameters,schema...](https://arrow.apache.org/docs/python/generated/pyarrow.parquet.write_table.html)
    - [iroh_blobs](https://docs.rs/iroh-blobs/latest/iroh_blobs/)
    - [Parquet file anatomy](https://towardsdatascience.com/anatomy-of-a-parquet-file/)

**Decisions / Observations / Blockers:**
- Starting tomorrow, I'll have to write something "concrete," otherwise I risk spreading myself too thin. I'll try to come up with a first draft, even if it's minimal.

**Next steps:**
- Minimal version of Dataset layout convetions
- Minimal version of logical topology

---

### 14/05/2026

**Work done:**
- Start the minimal version of dataset layout
- Do some research about iroh, DuckDB
- Learning Rust


**Decisions / Observations / Blockers:**
- I'll need your approval of the dataset layout once it's finished

**Next steps:**
- Continue learning Rust
- Moving forward with the dataset layout
- Start the logical topology if possible

---

### 15/05/2026 - Deliverable

**Work done:**

**Decisions / Observations / Blockers:**

**Next steps:**

---

## Weekly Summary 11/05 - 15/05

**Abstract**

**Next week**

<!-- journal tempalte

### DD/MM/YYYY - Deliverable

**Work done:**

**Decisions / Observations / Blockers:**

**Next steps:**

-->