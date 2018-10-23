#!/usr/bin/env python3

import os
import http.server
import socketserver

PORT = 8001

class MyHandler(http.server.SimpleHTTPRequestHandler):
    extensions_map = {
    '.html': 'text/html',
    '.js': 'application/x-javascript',
    '.wasm': 'application/wasm',  # we needed our own handler subclass for this?!
    '': 'application/octet-stream',
}

if __name__ == "__main__":
    os.chdir("./dist")
    httpd = socketserver.TCPServer(("", PORT), MyHandler)
    print("serving at port", PORT)
    httpd.serve_forever()
