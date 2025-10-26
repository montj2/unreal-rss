# Development Environment Setup

**Date**: October 26, 2025
**Status**: Prerequisites need installation

---

## Required Tools Installation

Your environment needs these tools installed before development can begin. Run these commands in order:

### 1. Install Rust (Stable Toolchain)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustc --version  # Verify
cargo --version  # Verify
```

**Expected output**:
```
rustc 1.72.0 (or later)
cargo 1.72.0 (or later)
```

### 2. Install Node.js & npm (v18 or later)

Option A: Using nvm (recommended)
```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
source ~/.bashrc
nvm install 18
nvm use 18
node --version  # Verify (should be v18.x)
npm --version   # Verify
```

Option B: Using system package manager (requires sudo)
```bash
sudo apt-get update
sudo apt-get install -y nodejs npm
```

Option C: Download prebuilt binary
```bash
mkdir -p ~/.local/bin
cd ~/.local/bin
wget https://nodejs.org/dist/v18.17.1/node-v18.17.1-linux-x64.tar.xz
tar xf node-v18.17.1-linux-x64.tar.xz
export PATH=~/.local/bin/node-v18.17.1-linux-x64/bin:$PATH
echo 'export PATH=$HOME/.local/bin/node-v18.17.1-linux-x64/bin:$PATH' >> ~/.bashrc
source ~/.bashrc
```

**Expected output**:
```
v18.17.1 (or later)
9.6.7 (or later)
```

### 3. Install Pre-commit Framework

```bash
pip3 install pre-commit
pre-commit --version  # Verify
```

If pip3 not available, install Python first:
```bash
# Via system package manager (requires sudo)
sudo apt-get install -y python3 python3-pip

# Or using miniforge (no sudo needed)
mkdir -p ~/miniforge3
wget -O ~/miniforge3/installer.sh https://github.com/conda-forge/miniforge/releases/latest/download/Miniforge3-Linux-x86_64.sh
bash ~/miniforge3/installer.sh -b
source ~/miniforge3/bin/activate
pip install pre-commit
```

**Expected output**:
```
pre-commit 3.x.x
```

### 4. Install System Dependencies (Linux only)

Required for Tauri app to build:

```bash
# Ubuntu/Debian
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev \
  libappindicator3-dev librsvg2-dev patchelf

# Fedora
sudo dnf install -y gtk3-devel webkit2gtk-devel \
  libappindicator-gtk3-devel librsvg2-devel

# Arch
sudo pacman -S gtk3 webkit2gtk libappindicator-gtk3 librsvg2
```

### 5. Verify Installation

```bash
# Run all verification checks
rustc --version
cargo --version
node --version
npm --version
git --version
pre-commit --version

# All should show version numbers without "command not found"
```

---

## Quick Install (All in One)

If you have unrestricted shell access, run this combined script:

```bash
#!/bin/bash
set -e

echo "Installing Rust..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env

echo "Installing Node.js via nvm..."
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
nvm install 18
nvm use 18

echo "Installing pre-commit..."
pip3 install pre-commit

echo "Verifying installation..."
rustc --version
cargo --version
node --version
npm --version
git --version
pre-commit --version

echo "✅ All tools installed!"
```

---

## Setup Project After Tools Installed

Once all tools are installed, run these commands from `/home/montj2/unreal-rss`:

```bash
cd /home/montj2/unreal-rss

# Initialize git (if not already done)
git init
git branch -M main
git config user.name "Your Name"
git config user.email "your.email@example.com"

# Install npm dependencies
npm install

# Install pre-commit hooks
pre-commit install
pre-commit install --hook-type pre-commit

# Verify setup
npm run type-check 2>/dev/null || echo "TypeScript setup pending"
pre-commit run --all-files || echo "Pre-commit hooks still need Rust/some tools"

# Add all files
git add .

# First commit
git commit -m "docs: initial documentation and specifications [PHASE-1-INFRASTRUCTURE]"

# Check status
git log -1 --oneline
git status
```

---

## Troubleshooting

### "command not found: rustc"
→ Run: `source $HOME/.cargo/env`

### "command not found: node"
→ Run: `source ~/.bashrc` or `source ~/.nvm/nvm.sh`

### "Permission denied" on system package install
→ Use `sudo` or install without system packages (see Option C for Node)

### Pre-commit hooks not running
→ Reinstall: `pre-commit uninstall && pre-commit install`

### Tests failing with "command not found: cargo"
→ Verify Rust is installed: `cargo --version`

---

## Next Steps

1. **Install all tools** using commands above
2. **Verify installation**: Run verification command
3. **Setup project**: Run setup commands
4. **Create git repo**: Initialize and commit
5. **Share with me**: Provide git repo URL so we can verify together

Once done, let me know and we'll:
- Verify all pre-commit hooks work
- Test GitHub Actions CI/CD workflow
- Make sure infrastructure is solid before Phase 1 coding

---

**Status**: ⏳ Awaiting tool installation
**Blocker**: Rust, Node.js, pre-commit not yet installed
**Next**: Run installation commands above, then confirm completion
