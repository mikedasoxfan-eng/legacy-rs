# 1.1 layer boundaries and dimension saves

The production Hills, Shore and SwampRivers layers now share their actual cell implementation with the probe. Six original-bytecode streams match exactly, covering321,840getInts calls and619,920outputcells. Inputs include23center values,32cardinal masks,three salts, coordinate wrapping, mixed rectangles and private original chunk/world seed state. The original complete layer graph also remains exact after the refactor. Reproduce with tools/verify_r11_neighborhood.ps1.

Dimension persistence matches the original at three seeds. Nine comparison sections cover Java/Rust saved streams and both crossload directions, comparing54complete chunks plus world metadata. Each case saves distinct payloads at identical coordinates under the root, DIM-1 and DIM1; settings, stored player dimension and world type are checked. Each stream is4,438,546bytes. Direct original WorldInfo copy-constructor type propagation and Nether spawn predicates are included. Reproduce with tools/verify_r11_dimstorage.ps1.

Storage runtime required no semantic changes in this lane. The only scenario change exposes an existing predicate for the probe. Original source and binaries remain outside Git; the reports retain hashes of the compared data. Full live client/dimension transitions and final regression suites remain separate gates.
