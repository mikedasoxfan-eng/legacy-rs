Render round 2's original-bytecode checks, written before the harness was merged, against their own
GL11/GL12/GL13 recorder and a Tessellator override. They are parked here (not compiled) because
tools/oracle10 compiles every src/ file together with the harness's shared recorder (RendererOracle).
Render round 3 ports them onto the harness recorder and moves them back to src/.
