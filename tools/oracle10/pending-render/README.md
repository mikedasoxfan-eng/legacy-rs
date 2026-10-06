Round 3 restored all ten Mode_render_* checks to src/net/minecraft/src/.
They use RendererOracle's opt-in extended snapshots and the original jar's
Tessellator; tools/verify_r10render2.sh invokes the shared oracle.sh build/runner.

The historical endpoints under headless/ are retained only as migration reference.
They are neither compiled nor used by any release rendering verification suite.
