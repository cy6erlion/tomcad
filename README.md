Manage your Caddy server from a single .toml file.

## Example

``` toml
[apps.http.servers.myhostname]
# Only listen for HTTPS connections
# Caddy automatically redirects HTTP -> HTTPS
listen = [":443"]

# TLS automation for all domains
[apps.tls.automation]
# An empty policy instructs Caddy to use its standard
# internal defaults, automatically obtaining and renewing TLS
# certificates via Let's Encrypt or ZeroSSL for every domain
# configured in the web server.
[[apps.tls.automation.policies]]
subjects = []

######################################################################
# Redirect www.example.com to https://example.com
[[apps.http.servers.myhostname.routes]]
terminal = true
[[apps.http.servers.myhostname.routes.match]]
host = ["www.example.com"]
[[apps.http.servers.myhostname.routes.handle]]
handler = "static_response"
status_code = 301
[apps.http.servers.myhostname.routes.handle.headers]
Location = ["https://example.com{http.request.uri}"]

# example.com
[[apps.http.servers.myhostname.routes]]
[[apps.http.servers.myhostname.routes.match]]
host = ["example.com", "www.example.com"]
[[apps.http.servers.myhostname.routes.handle]]
handler = "file_server"
root = "/var/www/example.com"
hide = [".git"]
######################################################################

# Simple reverse proxy
[[apps.http.servers.myhostname.routes]]
[[apps.http.servers.myhostname.routes.match]]
host = ["example2.com"]
[[apps.http.servers.myhostname.routes.handle]]
handler = "reverse_proxy"
[[apps.http.servers.myhostname.routes.handle.upstreams]]
dial = "localhost:5000"

######################################################################
# Advenced route any traffic to examaple.com/dashboard/* to dashboard server
[[apps.http.servers.myhostname.routes]]
match = [{ host = ["example.com"] }]
[[apps.http.servers.myhostname.routes.handle]]
handler = "subroute"
[[apps.http.servers.myhostname.routes.handle.routes]]
match = [{ path = ["/dashboard*"] }]
[[apps.http.servers.myhostname.routes.handle.routes.handle]]
handler = "rewrite"
strip_path_prefix = "/dashboard"
[[apps.http.servers.myhostname.routes.handle.routes.handle]]
handler = "reverse_proxy"
[[apps.http.servers.myhostname.routes.handle.routes.handle.upstreams]]
dial = "localhost:6000"
```
