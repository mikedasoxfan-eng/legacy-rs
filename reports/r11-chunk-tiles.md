# Original 1.1 tile ownership coordinates

Both block setters now pass world coordinates to their tile fallback. The metadata setter retains remote tile objects when only metadata changes.

`tools/verify_r11_chunk_tiles.ps1` regenerates 768 unchanged original-bytecode cases and compares the actual Rust World.set_block_and_metadata and World.set_block callback paths. Negative and positive chunk boundaries, eight container blocks, scanning/deferred and ordinary tile ownership, remote/local flags and metadata variants match the original loaded/pending list lengths and every tile position. The 1.1 fallback fix matters while tiles are being scanned: newly allocated tile objects live in the deferred list and the Chunk getter still returns null.
