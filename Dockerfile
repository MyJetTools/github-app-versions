FROM ubuntu:22.04
COPY ./target/release/github_app_versions ./target/release/github_app_versions
COPY ./wwwroot ./wwwroot 
ENTRYPOINT ["./target/release/github_app_versions"]