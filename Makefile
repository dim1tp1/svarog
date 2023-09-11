TASKS = lbm lbc
ifdef SGX
	CARGO := occlum-cargo
	TARGET_TRIPLE := x86_64-unknown-linux-musl/
else
	CARGO := cargo
	undefine TARGET_TRIPLE
	override undefine TARGET_TRIPLE
endif

.PHONY: all clean

all: debug

clean:
	cargo clean
	@for task in $(TASKS); do\
		`which echo` -e '\033[1;35m[DELETE]\033[0m' `readlink -f out/$$task`; \
		rm -f out/$$task; \
	done

# Quote the value to avoid adding trailing spaces into the variable.
debug: MODE = "debug"
debug: build

release: MODE = "release"
release: _CARGO_BUILD_FLAGS += --release 
release: build

build:
	@`which echo` -e '\033[1;35m[$(MODE)]\033[0m'
	$(CARGO) fmt
	$(CARGO) clippy --workspace --all-targets --tests -- -D warnings
	$(CARGO) build $(_CARGO_BUILD_FLAGS)
	@for task in $(TASKS); do\
		`which echo` -e '\033[1;35m[DELETE]\033[0m' `readlink -f out/$$task`; \
		rm -f out/$$task; \
	done
	@for task in $(TASKS); do\
		`which echo` -e '\033[1;35m[COPY]\033[0m' target/$(TARGET_TRIPLE)$(MODE)/$$task '\033[1;35m->\033[0m' `readlink -f out/$$task`; \
		rsync -a target/$(TARGET_TRIPLE)$(MODE)/$$task out/;\
	done
	tmux kill-session -t LB || true
	rsync -a assets/ out/
	mkdir -p out/data || true
	chmod 700 ./out/*

proto:
	make -C ./lx_grpc -f protoc.mk
