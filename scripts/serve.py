from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
import argparse
import functools

parser = argparse.ArgumentParser()
parser.add_argument("--port", type=int, default=8085)
args = parser.parse_args()
root = Path(__file__).resolve().parents[1] / "dist"


class Handler(SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Cache-Control", "no-store")
        super().end_headers()


handler = functools.partial(Handler, directory=str(root))
server = ThreadingHTTPServer(("127.0.0.1", args.port), handler)
print(f"Кромка бури: http://localhost:{args.port}", flush=True)
server.serve_forever()
