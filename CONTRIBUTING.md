# Contributing to SoroScope

Thank you for your interest in contributing to **SoroScope**! We are excited to have you as part of our community.

As a project in the **Stellar Wave Program**, we value collaboration and clear communication. This guide will help you get started with contributing to our monorepo.

## 🚀 Getting Started

### Prerequisites
- **Rust** (latest stable)
- **Node.js** (>= 18)
- **Soroban CLI**

### Monorepo Setup
1.  **Fork** the repository and clone it locally.
2.  **Rust Core**: Build the backend.
    ```bash
    cargo build -p soroscope-core
    ```
3.  **Web Dashboard**: Install frontend dependencies.
    ```bash
    cd web
    npm install
    ```
4.  **Contracts**: Compile the sample contracts.
    ```bash
    cargo build --target wasm32-unknown-unknown --release
    ```

## 🛠️ Development Standards

### Rust Code
- **Formatting**: Always run `cargo fmt` before committing.
- **Linting**: Run `cargo clippy` to check for common mistakes.
- **Tests**: Ensure all tests pass with `cargo test`.

### Frontend (Next.js)
- **Styling**: Use Tailwind CSS for consistency.
- **Linting**: Run `npm run lint` within the `/web` directory.
- **Components**: Keep components modular and placed in `/web/components`.

### Contracts
- Use **Soroban SDK v22.0.0** or higher.
- Avoid deprecated methods like `register_contract` (use `register` instead).

## 🐛 Reporting Issues

If you find a bug or have a feature request, please search existing [Issues](https://github.com/SoroLabs/soroscope/issues) first. 

### Bug Reports
When opening a bug report, please include:
- A clear, descriptive title.
- Steps to reproduce the issue.
- Your environment details (OS, Node version, Rust version).
- Expected vs. actual behavior.

### Feature Requests
We love fresh ideas! Please describe the use case and why this feature would be valuable for Soroban developers.

## 📮 How to Open a Pull Request

We follow the standard **Fork & Pull** workflow. If you're new to this, here is the exact step-by-step:

### 1. Fork & Clone
1.  Click the **Fork** button at the top of the [Soroban Scope repository](https://github.com/SoroLabs/soroscope).
2.  Clone your fork locally:
    ```bash
    git clone https://github.com/YOUR_USERNAME/soroscope.git
    cd soroscope
    ```
3.  Add the original repository as an `upstream` remote:
    ```bash
    git remote add upstream https://github.com/SoroLabs/soroscope.git
    ```

### 2. Create a Branch
Always work on a new branch, never on `main`:
```bash
git checkout -b feat/your-feature-name
```

### 3. Make Changes & Verify
Implement your changes and run the local verification commands:
```bash
# For Rust/Contracts
cargo fmt
cargo test

# For Web
cd web
npm run lint
```

### 4. Push & Open PR
1.  Commit and push your branch:
    ```bash
    git add .
    git commit -m "feat: descriptive message"
    git push origin feat/your-feature-name
    ```
2.  Go to the [Soroban Scope PR page](https://github.com/SoroLabs/soroscope/pulls).
3.  You should see a yellow banner saying **"Compare & pull request"**. Click it!
4.  Write a clear description of your changes and submit.

### 5. Address Feedback
A maintainer will review your code. If changes are requested, simply commit them to your branch and push again—the PR will update automatically.

## 🤝 Questions or Feedback?
Feel free to open an **Issue** or reach out to the **SoroLabs** team. Let's build the best Soroban developer tools together!
