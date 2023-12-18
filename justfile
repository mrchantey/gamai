# Sync with upstream ../forky/justfile
set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]

crates := 'gamai'
testable := 'gamai'
# features := '--features forky_play/shader_debug'
features := ''
backtrace := '0'
# backtrace := 'full'


default:
	just --list

### UTILS ###
@all command:
	for file in {{crates}}; do \
		just {{command}} $file; \
	done

cli *args:
	cargo run --manifest-path ../forky/Cargo.toml -p forky_cli -- {{args}}

clean *args:
	cargo clean -p {{args}}

clean-repo:
	cargo clean
	rm -rf ./target
	rm -rf ./Cargo.lock
	just all clean
# just ssl
# just test-all

fix *args:
	for file in {{crates}}; do \
			cargo fix --allow-dirty --lib -p $file {{args}}; \
	done

fmt *args:
	for file in {{crates}}; do \
			cargo fmt -p $file {{args}}; \
	done

storage-push:
	just -f ./config/storage.justfile push

storage-pull:
	just -f ./config/storage.justfile pull

### RUN ###

run crate example *args:
	RUST_BACKTRACE={{backtrace}} cargo run -p {{crate}} --example {{example}} {{features}} {{args}}

run-w *args:
	just watch 'just run {{args}}'

watch *args:
	forky watch \
	-w '**/*.rs' \
	-i '{.git,target,html}/**/*' \
	-i '**/mod.rs' \
	-i '**/*_g.rs' \
	-- {{args}}
### TEST ###

test-all *args:
	for file in {{testable}}; do \
		just test $file {{args}}; \
	done

test crate *args:
	RUST_BACKTRACE={{backtrace}} cargo run -p {{crate}} --example test_{{crate}} {{features}} -- {{args}}

test-w crate *args:
	RUST_BACKTRACE={{backtrace}} just watch 'cargo run -p {{crate}} --example test_{{crate}} {{features}} -- -w {{args}}'

test-all-wasm *args:
	just test-wasm keera_web {{args}}

test-wasm crate *args:
	just copy-wasm-assets
	sweet -p {{crate}} --example test_{{crate}} {{args}} --static ./target/static
# just build-css {{crate}}

expand crate example *args:
	just watch 'cargo expand -p {{crate}} --example {{example}} {{args}}'

### WASM ###

build-wasm crate example:
	just copy-wasm-assets
	cargo build -p {{crate}} --example {{example}} --target wasm32-unknown-unknown
	RUST_BACKTRACE={{backtrace}} wasm-bindgen \
	--out-dir ./html/wasm \
	--out-name bindgen \
	--target web \
	./target/wasm32-unknown-unknown/debug/examples/{{example}}.wasm \
	--no-typescript \

watch-wasm *args:
	just watch 'just build-wasm {{args}}'
# just watch 'just copy-wasm-assets'

serve-wasm *args:
	cd ./html && live-server --entry-file=index.html --host=0.0.0.0 --watch=wasm/bindgen_bg.wasm,index.html,style.css {{args}}

serve-https *args:
	just serve-wasm --https=https.config.js {{args}}

copy-wasm-assets:
	rm -rf ./html/assets
	rm -rf ./target/static
	mkdir ./target/static || true
	cp -r ./crates/keera_play/assets ./html/assets
	cp -r ./crates/keera_play/assets ./target/static/assets

ssl:
	openssl genrsa -out target/client-key.pem 2048
	openssl req -new -key target/client-key.pem -subj "/CN=$cn\/emailAddress=admin@$cn/C=US/ST=Ohio/L=Columbus/O=Widgets Inc/OU=Some Unit" -out target/client.csr
	openssl x509 -req -in target/client.csr -signkey target/client-key.pem -out target/client-cert.pem


deploy-device-web:
	rm -rf ./crates/keera_web/html
	cp -r ./html ./crates/keera_web/html
	cd ./crates/keera_web && firebase deploy

### CSS ###
watch-css crate *args:
	forky watch \
	just build-css {{crate}} {{args}} \
	-w '**/*.css' \
	-i '{.git,target,html}/**/*' \
# cargo watch --why --ignore '{justfile,.gitignore}' --ignore '**.{rs,txt,md,wasm,wat,wgsl}' --ignore 'html*' -- just build-css {{crate}} {{args}}

@build-css crate *args:
	just lightning ./crates/{{crate}}/src/index.css ./target/static/style.css {{args}}
# just lightning ./crates/{{crate}}/src/index.css ./target/static/style.css {{args}}

lightning in out *args:
	lightningcss {{in}} --bundle -m -o {{out}} {{args}}
