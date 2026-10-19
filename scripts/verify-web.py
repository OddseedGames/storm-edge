from hashlib import sha256
from pathlib import Path
import json
import sys

root = Path(sys.argv[1] if len(sys.argv) > 1 else "dist")
manifest = json.loads((root / "build.json").read_text())
files = manifest["files"]
html = (root / "index.html").read_text()
required = [
    "index.html", "storm-edge.js", "storm-edge_bg.wasm", ".nojekyll",
    *[f'{manifest["assets"]}/{name}' for name in [
        "shaders/rock.wgsl", "shaders/atmosphere.wgsl",
        "textures/weathered-rock.png", "textures/PROVENANCE.md",
        "fonts/IBMPlexMono-Regular.ttf", "fonts/OFL.txt", "audio/wind.wav",
    ]],
]
for name in required:
    if name not in files:
        raise SystemExit(f"Missing packaged file: {name}")
actual = {path.relative_to(root).as_posix() for path in root.rglob("*") if path.is_file()}
if actual != set(files) | {"build.json"}:
    raise SystemExit("Package contains files absent from its manifest")
for name, metadata in files.items():
    path = root / name
    if sha256(path.read_bytes()).hexdigest() != metadata["sha256"]:
        raise SystemExit(f"Checksum mismatch: {name}")
    if path.stat().st_size != metadata["bytes"] or metadata["bytes"] > 100 * 1024 * 1024:
        raise SystemExit(f"Invalid packaged size: {name}")
if (root / "storm-edge_bg.wasm").read_bytes()[:8] != b"\0asm\x01\0\0\0":
    raise SystemExit("Invalid WASM header")
if "__" in html or f'data-assets="{manifest["assets"]}"' not in html:
    raise SystemExit("HTML placeholders or asset root are invalid")
for name in ["storm-edge.js", "storm-edge_bg.wasm"]:
    version = files[name]["sha256"][:16]
    if f"./{name}?v={version}" not in html:
        raise SystemExit(f"Missing versioned URL for {name}")
if f'content="{manifest["build"]}"' not in html:
    raise SystemExit("HTML build version differs from the manifest")
print(f'Package verified: {len(files)} files, build {manifest["build"]}')
