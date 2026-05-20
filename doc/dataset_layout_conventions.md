# Dataset Layout Conventions

## Table of Contents
1. [Scope and objectives](#1-scope-and-objectives)
2. [Design Constraints](#2-design-constraints)
3. [Directory structure](#3-directory-structure-on-each-peer)
4. [Dataset metadata](#4-dataset-metadata)
5. [Parquet file schema](#5-parquet-file-schema)
6. [Integrity](#6-integrity)

## 1. Scope and objectives
This document lists the representation options for a federated Parquet dataset distributed over an ad-hoc P2P network (iroh). It covers three dimensions:
1. Directory structure: how to organize files on each peer.
2. File naming and identification.
3. Dataset metadata: how to describe the contents of a peer.

This is a preliminary version of this document, it will evolve as the project progresses.

## 2. Design Constraints
Here are the initial constraints we need to consider before we can carry out this project.

| Constraint | Value |
|---|---|
| Number of peers | 2–5 machines |
| Data file format | Parquet |
| Transport | iroh-blobs |
| Notebook access | PyArrow / pandas / DuckDB |

## 3. Directory structure on each peer.
For this PoC, we'll keep our file structure as simple as possible. This may evolve in the future.

Here is an example of a structure that is sufficient to get this project started. All files are stored flat in a single data/ directory. For external data in a cache folder. Subdirectory nesting may be introduced in future versions.   

```
Files follow the pattern {modality}_{site}_{year}.parquet
All components use lowercase

data/
    t1_hes-so-sion_2026.parquet
    bold_hes-so-sion_2026.parquet
cache/
    t1_mcgill-montreal_2026.parquet
```


## 4. Dataset Metadata
For a client to discover what a peer offers without downloading all files, a way to share the metadata for each node is needed.


### JSON manifest
A `manifest.json` file that each node possesses and that is sent to everyone so that each node has an idea of what the others possess. This section will be covered in more detail later
```json
{
  "peer_id": "abc123",
  "version": "1.0",
  "files": [
    {
      "name": "t1_hes-so-sion_2026.parquet",
      "iroh_hash": "123abc",
      "size_bytes": 420000,
      "modality": "T1w",
      "schema": ["participant_id", "cjv","cnr"],
      "row_count": 450
    }
  ]
}
```

### Parquet metadata (footer)
Parquet natively stores a **footer** containing the schema [R6](#r6--parquet-file-format). This information is readable without downloading the entire file, but only if the file is already local or if iroh supports range requests.   
As noted in the project description [R7](#r7--bthesis-vincent-cordola), Iroh-blobs’ verified range requests have not yet been tested against Parquet’s footer-first read
pattern, the Parquet partial-read optimization is treated as optional.

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
- Requires validation step on the producer's side

### Strict schema
Fixed schema
- All peers produce structurally identical files
- Any extension requires a new schema version

### Choices for this project
To start with we'll use the shared minimal schema, which will give us a common foundation with a minimum of flexibility. For example, we could have the following as required columns: `participant_id`,`site`,`modality`

## 6. Integrity
Data integrity during transmission is guaranteed by iroh-blobs, which uses content-addressed hashing (BLAKE3)[R3](#r3--iroh-computer-blob). At this time, we do not have any data validation checks in place when data is received, but this could be added later to ensure more thorough validation of our data.

## References
##### R1 | [DuckDB Reading and Writing Parquet file](https://duckdb.org/docs/data/parquet/overview.html)
##### R2 | [PyArrow parquet](https://arrow.apache.org/docs/python/generated/pyarrow.parquet.write_table.html)
##### R3 | [Iroh computer blob](https://docs.iroh.computer/protocols/blobs)
##### R4 | [MRIQC IQMs](https://mriqc.readthedocs.io/en/latest/measures.html)
##### R5 | [MRIQC Web-API](https://www.biorxiv.org/content/10.1101/216671v1.full.pdf)
##### R6 | [Parquet File Format](https://parquet.apache.org/docs/file-format/)
##### R7 | [bthesis Vincent Cordola](media/bthesis_Vincent_Cordola.pdf)