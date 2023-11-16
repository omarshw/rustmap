# RustMap 🌐🕵️
Fast and lightweight port scanner written in Rust. Quickly check for open ports on networks within less than 10 seconds.

## Features 🚀
- **Speedy Scanning**: RustMap is designed to be fast and efficient, allowing you to scan networks swiftly.
- **Lightweight**: Minimal resource usage, making it suitable for various environments.
- **Easy to Use**: Simple command-line interface for quick scanning.
- **Concurrent Port Checking**: Utilizes concurrency to check multiple ports simultaneously.
- **Summary Report**: Provides a short summary at the end of the scan.

## Usage 🛠️
1. Clone the repository:
```bash
git clone https://github.com/omar-shawkat/rustmap
cd rustmap
```

2. Use the requirements.sh script to install Rust & Cargo
```bash
chmod +x requirements.sh
sudo ./requirements.sh
```

3. Run Rustmap with the target IP:
```bash
cargo run <ip_address>
```
