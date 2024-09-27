# mandelbrot-parallel-rust

Cálculo del conjunto de Mandelbrot

## Rust

```bash
cargo build
cargo run -p mandelbrot --release
```

## Python run Rust

```bash
cd mandelbrot_py
python3 -m venv venv
source venv/bin/activate
python3 -m pip install pip --upgrade
python3 -m pip install -r requirement.txt
cd ../mandelbrot_lib
maturin develop
cd ../mandelbrot_py
python3 main.py
```

## Cross

```bash
cargo install cross
cross build --target x86_64-pc-windows-gnu --release
cross build --target x86_64-unknown-linux-gnu --release
```
