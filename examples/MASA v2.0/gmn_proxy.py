"""Lightweight proxy: uses curl to forward requests, clearing proxy env."""
import json, subprocess, http.server, sys, tempfile, os

class ProxyHandler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(length)
        try:
            data = json.loads(body)
            if data.get("model") == "gpt55":
                data["model"] = "gpt-5.5"
                print("[proxy] Rewrote model", flush=True)
            body = json.dumps(data).encode()
        except Exception:
            pass

        url = "https://gmncode.com" + self.path
        auth = self.headers.get("Authorization", "")

        tmp = tempfile.NamedTemporaryFile(delete=False, suffix=".json", mode="wb")
        tmp.write(body)
        tmp.close()

        try:
            env = os.environ.copy()
            for k in list(env.keys()):
                if k.lower() in ("http_proxy", "https_proxy", "all_proxy", "no_proxy"):
                    del env[k]

            result = subprocess.run(
                ["curl.exe", "-s", "-w", "\n%{http_code}",
                 "-X", "POST", url,
                 "-H", "Content-Type: application/json",
                 "-H", f"Authorization: {auth}",
                 "-d", f"@{tmp.name}"],
                capture_output=True, timeout=120, env=env
            )
            stdout = result.stdout
            newline = stdout.rfind(b"\n")
            if newline >= 0:
                status_str = stdout[newline+1:].strip().decode()
                resp_body = stdout[:newline]
                status = int(status_str)
            else:
                resp_body = stdout
                status = 200

            print(f"[proxy] -> {status}", flush=True)
            self.send_response(status)
            self.send_header("Content-Type", "application/json")
            self.end_headers()
            self.wfile.write(resp_body)
        except Exception as e:
            print(f"[proxy] Error: {e}", flush=True)
            self.send_response(502)
            self.end_headers()
            self.wfile.write(f'{{"error":"proxy error"}}'.encode())
        finally:
            try: os.unlink(tmp.name)
            except: pass

    def log_message(self, fmt, *args):
        print(f"[proxy] {args[0]} {args[1]} {args[2]}", flush=True)

if __name__ == "__main__":
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8001
    server = http.server.HTTPServer(("127.0.0.1", port), ProxyHandler)
    print(f"GMN proxy listening on 127.0.0.1:{port}", flush=True)
    server.serve_forever()
