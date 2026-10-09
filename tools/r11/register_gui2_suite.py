"""Register the GUI suite once the separately maintained verifier is present."""
from pathlib import Path

path = Path(__file__).resolve().parents[1] / 'verify10.sh'
if not path.exists():
    print('Deferred: tools/verify10.sh is not in this worktree. Run this after merging the base verifier.')
    raise SystemExit(0)
text = path.read_text()
call = 'bash tools/verify_r11gui2.sh --built || fail=1'
if call not in text:
    offset = text.rfind('\npflush\n')
    if offset < 0:
        raise SystemExit('Cannot find the final pflush; verifier was left unchanged.')
    text = text[:offset] + '\n# --- gui (1.0), round 2 ---\n' + call + '\n# --- end gui ---\n' + text[offset:]
    path.write_text(text, newline='\n')
print('GUI round-two suite registered before the final pflush.')
