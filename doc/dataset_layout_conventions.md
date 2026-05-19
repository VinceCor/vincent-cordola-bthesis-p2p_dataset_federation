# Dataset Layout Conventions

## Table of Contents
1. [Scope and objectives](#1-scope-and-objectives)
2. [Design Constraints](#2-design-constraints)
3. [Directory structure](#3-directory-structure-on-each-peer)
4. [Dataset metadata](#4-dataset-metadata)
5. [Parquet file schema](#5-parquet-file-schema)

## 1. Scope and objectives
This document lists the representation options for a federated Parquet dataset distributed over and ad-hoc P2P network (iroh). It covers three dimensions:
1. Directory structure: how to organize files on each peer.
2. File naming and identification.
3. Dataset metadata: how to desribe the contents of a peer.

This is a preliminary version of this document, it will evolve as the project progresses.

## 2. Design Constraints
Here are the initial constraints we need to consider before can carry out this project.

| Constraint | Value |
|---|---|
| Number of peers | 2–5 machines |
| Data file format | Parquet |
| Transport | iroh-blobs |
| Notebook access | PyArrow / pandas / DuckDB |

## 3. Directory structure on each peer.
For this PoC, we'll keep our file structure as simple as possible (one or more Parquet files per node). This may evolve in the future.

## 4. Dataset Metadata
For a client to discover what a peer offers without downloading all files, a way to share the metadata for each node is needed.


### JSON manifest
A `manifest.json` file that each node possesses and that is sent to everyone so that each node has an idea of what the others possess.
```json
This section will be covered in more detail later
{
  "peer_id": "abc123",
  "iroh_hash": "123456",
  "row_count": 350
}
```

### Parquet metadata (footer)
Parquet natively stores a **footer** containing the schema. This information is readable without downloading the entire file, but only if the file is already local or if iroh supports range requests.   
Iroh-blobs’ verified range requests have not yet been tested against Parquet’s footer-first read
pattern, the Parquet partial-read optimization is treated as optional

## 5. Parquet file schema
For this project, we also need to ensure that each node exchanges data, to do this, we need to determine whether they should follow a specific schema.
### Free schema per peer
Each peer defines its own columns.
- No constraints on the producer side
- Requires schema unification on the client
- DuckDB may fail if columns differ between files

### Shared minimal schema
A minimal set of columns is mandatory, additional columns are allowed.
- Cross-peer JOIN on common columns is guaranteed
- Additional columns aren't a problem if we ignore them
- Requires validation step on the produces side
### Strict schema
Fixed schema
- All peers produce structurally identical files
- Any extension requires a new schema version

## References
| ID | Source |
|---|---|
| R1 | https://duckdb.org/docs/data/parquet/overview.html |
| R2 | https://arrow.apache.org/docs/python/generated/pyarrow.parquet.write_table.html |
| R3 | https://docs.iroh.computer/protocols/blobs |
| R4 | https://mriqc.readthedocs.io/en/latest/measures.html |
| R5 | https://www.biorxiv.org/content/10.1101/216671v1.full.pdf |
| R6 | https://medium.com/@sanjeets1900/understanding-the-parquet-file-format-part-1-428d40944393 |
| R7 | [bthesis Vincent Cordola](media/bthesis_Vincent_Cordola.pdf) |

