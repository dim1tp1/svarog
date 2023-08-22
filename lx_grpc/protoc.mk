.PHONY: all

all: rust

rust:
	cargo build --release --manifest-path=protoc/Cargo.toml
	rsync -avP protoc/target/release/luban_protoc ./
	./luban_protoc

java:
	rm -rf src_java
	mkdir src_java
	protoc --java_out=src_java --grpc-java_out=src_java --proto_path=proto proto/*.proto