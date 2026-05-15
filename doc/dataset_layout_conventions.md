# Dataset Layout Conventions

## Table of Contents

## 1. Scope and objectives
This document lists the representation options for a federated Parquet dataset distributed over and ad-hoc P2P network (iroh). It covers three dimensions:
1. Directory structure: how to organize files on each peer.
2. File naming and identification.
3. Dataset metadata: how to desribe the contents of a peer.

### 2. Design Constraints
| Constraint | Value |
|---|---|
| Number of peers | 2–5 machines |
| Estimated file size||
| Transport | iroh-blobs |
| Notebook access | PyArrow / pandas / DuckDB |

## 2. Directory structure on each peer.
Three approaches are possible for organizing Parquet files locally. If we use MRIQC as an example for storing our data.

### Flat directory

```
modality / site / year

examples:

data/ 
    T1_hes-SO-Sion_2026.parquet
    bold_hes-SO-Sion_2026.parquet
    T1_Mcgill-Montreal_2026.parquet
```
advantage / disadvantage
| Criteria | |
|---|---|
| Scalability | Need strict naming convention to avoid possible collision |
| iroh compatibility | One file = one blob|




### Modality / site hierarchy
```
examples:

data/
  modality=T1/
    site=hes-so-Sion/
      run-20240101.parquet
  modality=bold/
    site=McGill/
      run-20240210.parquet
```
advantage / disadvantage
| Criteria | |
|---|---|
| Scalability | Native partitioning |
| iroh compatibility | Iroh-blobs adresses by file, directories are not a native concept, requires iterating the local tree |

### Only one Parquet per site
```
examples:

data/
  hes-SO_Sion.parquet
  McGill.Parquet
```
advantage / disadvantage
| Criteria | |
|---|---|
| Scalability | Potentially large files |
| iroh compatibility | Direct adressing: one peer = one blob |

## 3. Dataset Metadata
For a client to discover what a peer offers without downloading all files, a way to share the metadata for each node is needed.


### JSON manifest
A `manifest.json` file published 

### Distributed manifest via iroh-gossip

### Parquet metadata (footer)

## References
| ID | Source |
|---|---|
| R1 | https://duckdb.org/docs/data/parquet/overview.html |
| R2 | https://arrow.apache.org/docs/python/generated/pyarrow.parquet.write_table.html |
| R3 | https://docs.iroh.computer/protocols/blobs |
| R4 | https://mriqc.readthedocs.io/en/latest/measures.html# |
| R5 | https://www.biorxiv.org/content/10.1101/216671v1.full.pdf |
| R6 | https://medium.com/@sanjeets1900understanding-the-parquet-file-format-part-1-428d40944393
 |

