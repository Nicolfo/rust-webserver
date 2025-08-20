# Rust Webserver
This project aims to provide a WebServer capable of hosting static content, like SPA or static websites. At the moment it doesn't provide https support (connection should be terminated elsewhere, like by an ingress controller).
It's written in Rust and provide a caching mechanism to minimize disk reading. 
