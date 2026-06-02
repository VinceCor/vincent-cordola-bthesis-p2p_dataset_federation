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
- **Milestone:** Tasks included in the PDF: [Donnée du travail de bachelor](https://isc-hei.github.io/isc-typst-tb-descriptions/)
- **Task:** Tasks created during the project to provide a clearer overview of the various steps to be completed

| Weeks | Deliverable | Target date | Status |
|---|-------------|-------------|--------|
| W1-W2 | **1. Onboarding & Technical Design** | 11.05 - 24.05 |  |
| **Milestone** | Rust & iroh crash-course completed | 24.05 | Done |
| **Milestone** | Dataset layout conventions defined | 24.05 | Done |
| Task | Report setup, GitHub page, planning | 24.05 | Done |
| Task | logical topology (to get an overview of the project, first draft) | 24.05 | Done |
| W3-W6 | **2. Core P2P sharing prototype** | 25.05 - 21.06 | |
| **Milestone** | Rust MVP: (i) join an ad-hoc iroh network, (ii) advertise Parquet files, and (iii) fetch Parquet files from peers. | 31.05 | In progress |
| Task | Setting up the Rust project | 31.05 | Done |
| Task | Join an ad hoc iroh network | 31.05 | Done |
| Task | Advertise and fetch Parquet files from peers | 31.05 | In progress |
| Task | End-to-end test on two terminals (same machine) | 31.05 | In progress |
| **Milestone** | Validation on a Docker Compose network of 3–5 containers. | 21.06 | Upcoming |
| W7-W10 | **3. Python/Jupyter integration and connectivity validation** | 22.06 - 19.07 | |
| **Milestone** | Python client layer: a unified Jupyter interface ensuring required files are present locally via on-demand retrieval. | 19.07 | Upcoming |
| **Milestone** | Migrate the testbed to virtual machines with separate network stacks to exercise NAT traversal and relay fallback for the first time. | 19.07 | Upcoming |
| **Milestone** | Produce a first working notebook that discovers the network dataset, loads it into standard dataframe tooling, and runs queries spanning multiple peers. | 19.07 | Upcoming |
| W11-W13 | **4. Robustness and evaluation in a 2–5 machine testbed** | 20.07 - 09.08 | |
| **Milestone** | Test behavior under realistic conditions: peers joining/leaving, partial availability, varying bandwidth. | 09.08 | Upcoming |
| **Milestone** | Improve reliability (resume partial transfers, cache behavior, error handling) and document known limitations. | 09.08 | Upcoming |
| **Milestone** | Optional (if feasible): evaluate byte-range or partial reads to reduce transfers for larger Parquet artifacts. | 09.08 | Upcoming |
| W14 | **5. Packaging, documentation, and final demonstration** | 10.08 - 12.08 | |
| **Milestone** | Finalize the notebook as the primary demonstration artifact. | 12.08 | Upcoming |
| **Milestone** | Produce setup scripts/instructions for reproducing the demo on 2–5 machines | 12.08 | Upcoming |
| **Milestone** | Write a short evaluation report summarizing results, constraints, and recommended next steps for downstream integration work. | 12.08 | Upcoming |
| **Milestone** | **Final code submitted** | **25/08/2026 12:00** | Upcoming |
| **Milestone** | **Final report submitted** | **25/08/2026 12:00** | Upcoming |
| **Milestone** | **Executive summary** | **25/08/2026** | Upcoming |
| **Milestone** | **Poster** | **25/08/2026** | Upcoming |

### 1.2 Key Milestones

| Milestone | Date |
|-----------|------|
| Bachelor thesis start | May 11, 2026 |
| Midterm presentation | June 3, 2026 |
| Oral defence | Weeks of August 17 - 25, 2026 |
| Final code submission | August 25, 2026 - 12:00 |
| Executive summary | August 25, 2026 |
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
### Meeting - 01/06/2026

**Participants:** Prof. Oscar Esteban - Vincent Cordola     
**Type:** Overview of the project's progress  
**Duration:** 30 min

**Discussion points**
- Presentation of the planning and the project's progress
- Current progress on the Rust component and potential additional features
- Potential inclusion of DuckDB to the project (mention in the project pdf)

**Decisions made**
- Overall, the project is progressing very well
- Stick to the features listed in the PDF, don't get sidetracked by extra features. These could be added once the "main" project is finished
- It remains to be seen whether DuckDB can be easily integrated and wheter that would be useful



### Meeting - 02/06/2026

**Participants:** Vincent Cordola - Origami lab     
**Type:** Presentation project overall
**Duration:** 20 min

**Discussion points**
- General information about the project / bachelor thesis
- More details about the p2p.
- Speaking about the potential implementation, problems that may arise

**Decisions made**
- _

**media**
- [Introduction p2p projet](doc/media/bthesis_proj_intro.pdf)

<!-- Reviews and Meetings template
### Meeting - [Date]

**Participants:** Vincent Cordola - x   
**Type:**   
**Duration:** _ min

**Discussion points**
- _

**Decisions made**
- _
-->

## 4. Work Journal

### 11/05/2026 - Report setup, GitHub page, planning

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

### 13/05/2026

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

### 15/05/2026

**Work done:**
- Progress on the layout convetions dataset
- Learning about iroh to see how blobs work

**Decisions / Observations / Blockers:**

**Next steps:**
- Finish the first draft for the dataset layout conventions
- Start logical topology
---

## Weekly Summary 11/05 - 17/05

**Abstract**
During this first week, i began laying the groundwork for the project. It was a bit of a struggle to get started and stay focused after moving to a new country. However, i was still able to make a good start on the planning, which will help me with the project's ongoing progress.

**Next week**
- I should finish the dataset layout document as well as the one on topology.
- Schedule weekly meetings and the interim presentation in early June.
- Keep learning Rust and iroh so I'll be ready to start implementation next week.

---

### 18/05/2026 - Public holiday (Journée nationale des Patriotes)

**Work done:**
- Preparing presentation of my project for the lab
- Rust learning
**Decisions / Observations / Blockers:**

**Next steps:**

---

### 19/05/2026

**Work done:**
- Converstation with Dr. Nikhil Bhagwat about QC studio
- Modification of the data layout convention document

**Decisions / Observations / Blockers:**

**Next steps:**
- Finalize the first draft of the data layout convetion
- Getting started with logical topology

---

### 20/05/2026

**Work done:**
- First draft for the dataset layout conventions
- Start of the document describing the project architecture
**Decisions / Observations / Blockers:**
- There will be updates in the future for the dataset layout conventions, this provides a starting point.

**Next steps:**
- Continue and finish the first draft for the architecture
- Continue with the Rust courses and possibly start focusing more on the early stages of development for the iroh component.
- Define how the development environment will be structured

---

### 21/05/2026

**Work done:**
- Progress on the architecture document
- Iroh documentation

**Decisions / Observations / Blockers:**
- I should be careful not to spend too much time reading the documentation, it’s better for me to start the implementation and learn as I go rather than just reading the documentation.

**Next steps:**
- Finish first draft architecture document
- Start implementation

---

### 22/05/2026

**Work done:**
- Finish first draft of architecture
- First search for the Rust MVP (List the main tasks in the planning)
- Start of the installation guide for the Rust MVP
- Tokio documentation

**Decisions / Observations / Blockers:**

**Next steps:**
- Continue the rust-mvp


## Weekly Summary 18/05 - 24/05

**Abstract**
During this second week, I did a lot of research on the technologies I would be using for this project. I also now have a good overall picture of how I want to structure the project.

**Next week**
If possible, I'll try to finish the Rust MVP (non Docker version). With all the examples I can find online, I should be able to quickly exchange PArquet files between two terminals. If all goes well, I'll be able to package this into Docker containers the following week.

---
### 25/05/2026

**Work done:**
- New search on iroh regarding endpoints
- Review of the general documentation

**Decisions / Observations / Blockers:**
- I need to provide some initial results regarding iroh, I don't need to go into that much detail in the documentation

**Next steps:**
- Continue join an ad hoc iroh network

---

### 26/05/2026

**Work done:**
- Good progress on Join an ad hoc iroh network 
- The dependencies should work fine for this part

**Decisions / Observations / Blockers:**
- Rust with Iroh is even more complicated than what I've seen before, I'm groing to have to keep it as simple as possible for this to work.
- The examples provided on the iroh computer GitHub repository are very helpful.

**Next steps:**
- Finish the Join an ad hoc iroh network part
- Make sure to document this section properly
- Start Advertise and fetch Parquet files from peers

---

### 27/05/2026

**Work done:**
- Finish code for join an ad-hoc iroh network
- Progress in the documentation
- Research on the "network" functionality of this component (QUIC, NAT, hole punching, ...)

**Decisions / Observations / Blockers:**

**Next steps:**
- Finish the installation guide on join an ad-hoc iroh network
- Start the section on Parquet file exchange

---

### 28/05/2026

**Work done:**
- Progress on the installation guide
- Documentation about the manifest

**Decisions / Observations / Blockers:**
- For the manifest, I'll be able to use / modify from what's been done for data lakehouse (for example, with Apache Iceberg).
- I'm currently using iroh relays for hole punching between endpoints. I shouldn't have any network issues for the rest of the projet. We'll see how it goes.

**Next steps:**
- Finish installation guide for join an ad-hoc iroh network
- Start advertise / fetch Parquet from peers

---

### 29/05/2026

**Work done:**
- Finish Join an ad hoc iroh network
- Documentation for the advertise and fetch parquet file
- Start parquet implementation.

**Decisions / Observations / Blockers:**
- Since I'm currently on track with my planning, I'm taking the time to focus a bit more on how I'll manage my files. To do this, I plan to use the Serde framwork to manage my manifests. This will provide a robust validation of my files right from the start and make any future changes easier.

**Next steps:**
- Continue advertise Parquet files, and fetch Parquet files from peers.


## Weekly Summary 25/05 - 31/05

**Abstract**
THe first use/implementation of iroh was a success. I was able to connect right away, and as a bonus, any firewall or other issues are already resolved. NAT hole punching is already handled by the iroh servers. This allows us to establish connections beyond the LAN without having to set up port forwarding or route our data through relay servers. The iroh relay server is there solely for the handshake.

**Next week**
- Finish advertise Parquet files, and fetch Parquet files from peers.
- Start test in docker environment 

---

### 01/06/2026

**Work done:**
- Code for advertise and fetch parquet file
- Test on two terminals

**Decisions / Observations / Blockers:**
- Focus on the points specifically mentioned in the PDF and don't start working on a manifest file

**Next steps:**
- Complete the documentation for advertise and fetch parquet file
- Start Docker if possible

---

### 02/06/2026

**Work done:**
- Presentation of my project at the lab [Introduction p2p projet](doc/media/bthesis_proj_intro.pdf)
- Continue the installation guide for listen-blobs and fetch-blobs

**Decisions / Observations / Blockers:**
- Provide a clear description of the system architecture (how the NAT hole punching works...)

**Next steps:**
- Finish installation guide and update system architecture with this part
- Start Docker


## Weekly Summary 01/06 - 07/05

**Abstract**


**Next week**


<!-- journal tempalte

### DD/MM/2026

**Work done:**

**Decisions / Observations / Blockers:**

**Next steps:**

-->