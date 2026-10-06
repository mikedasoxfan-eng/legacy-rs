"""Strict RGB comparison of every shot named in a scene script; no tolerance."""
from collections import Counter
import json
from pathlib import Path
import sys
from PIL import Image, ImageChops

scenes, java_dir, rust_dir, diff_dir = map(Path, sys.argv[1:])
names = [line.split('#', 1)[0].strip().split(maxsplit=1)[1]
         for line in scenes.read_text().splitlines()
         if line.split('#', 1)[0].strip().startswith('shot ')]
assert names, 'No screenshots requested'
diff_dir.mkdir(parents=True, exist_ok=True)
results = {}
for name in names:
    print('== gui10: ' + name)
    j, r = java_dir / (name + '.png'), rust_dir / (name + '.png')
    if not j.is_file() or not r.is_file():
        results[name] = {'error': 'missing capture'}
        print(name + ': missing capture')
        continue
    a, b = Image.open(j).convert('RGB'), Image.open(r).convert('RGB')
    if a.size != b.size:
        results[name] = {'error': 'different dimensions'}
        print(f'FIRST DIFFERENCE: image dimensions: Java {a.size}, Rust {b.size}')
        print(name + ': dimensions differ')
        continue
    diff = ImageChops.difference(a, b)
    pixels = Counter(diff.get_flattened_data())
    bad = sum(count for color, count in pixels.items() if color != (0, 0, 0))
    maximum = max(map(max, pixels))
    results[name] = {'pixels': bad, 'max_channel_difference': maximum}
    if bad:
        for index, (java_pixel, rust_pixel) in enumerate(zip(a.get_flattened_data(), b.get_flattened_data())):
            if java_pixel != rust_pixel:
                first = {'x': index % a.width, 'y': index // a.width, 'java': java_pixel, 'rust': rust_pixel}
                results[name]['first_difference'] = first
                print(f"FIRST DIFFERENCE: pixel ({first['x']},{first['y']}).RGB: Java {java_pixel}, Rust {rust_pixel}")
                break
    print(f'{name}: ' + ('EXACT' if not bad else f'{bad} pixels differ; maximum {maximum}'))
    if bad:
        diff.point(lambda v: 255 if v else 0).save(diff_dir / (name + '.png'))
    else:
        (diff_dir / (name + '.png')).unlink(missing_ok=True)
(diff_dir / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
exact = all(v.get('pixels') == 0 for v in results.values())
print('ALL GUI CAPTURES EXACT' if exact else 'SOME GUI CAPTURES DIFFER')
sys.exit(0 if exact else 1)
