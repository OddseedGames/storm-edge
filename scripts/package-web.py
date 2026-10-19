from hashlib import sha256
from pathlib import Path
import json
import os
import shutil
import sys

project = Path(__file__).resolve().parents[1]
output = Path(sys.argv[1])


def digest(path):
    return sha256(path.read_bytes()).hexdigest()


assets = project / "assets"
asset_files = sorted(path for path in assets.rglob("*") if path.is_file())
asset_hash = sha256()
for path in asset_files:
    asset_hash.update(path.relative_to(assets).as_posix().encode())
    asset_hash.update(bytes.fromhex(digest(path)))
asset_version = asset_hash.hexdigest()[:16]
asset_root = f"assets-{asset_version}"
shutil.copytree(assets, output / asset_root)

js_hash = digest(output / "storm-edge.js")
wasm_hash = digest(output / "storm-edge_bg.wasm")
template = (project / "web/index.html").read_text()
build_hash = sha256((js_hash + wasm_hash + asset_version + template).encode()).hexdigest()[:16]
html = template.replace("__BUILD_VERSION__", build_hash)
html = html.replace("__ASSET_VERSION__", asset_version)
html = html.replace("__JS_VERSION__", js_hash[:16])
html = html.replace("__WASM_VERSION__", wasm_hash[:16])
(output / "index.html").write_text(html)
(output / ".nojekyll").touch()

files = {}
for path in sorted(output.rglob("*")):
    if path.is_file():
        files[path.relative_to(output).as_posix()] = {
            "sha256": digest(path),
            "bytes": path.stat().st_size,
        }
manifest = {
    "build": build_hash,
    "commit": os.environ.get("GITHUB_SHA"),
    "assets": asset_root,
    "files": files,
}
(output / "build.json").write_text(json.dumps(manifest, ensure_ascii=False, indent=2) + "\n")
print(f"Web build: {build_hash}; assets: {asset_root}")
