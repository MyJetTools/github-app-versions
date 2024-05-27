FROM ubuntu:22.04
COPY ./target/release/github-app-versions ./target/release/github-app-versions
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/github-app-versions"]