# 🧠 GitSynapse : The AI-Native Software Forge

[![Rust](https://img.shields.io/badge/rust-1.80%2B-blue.svg)](https://www.rust-lang.org)
[![VanillaJS](https://img.shields.io/badge/frontend-Vanilla%20JS-yellow.svg)](#)
[![Architecture](https://img.shields.io/badge/arch-Microservices%20%2F%20FaaS-success.svg)](#)
[![Spec](https://img.shields.io/badge/spec-OpenSpec-purple.svg)](https://openspec.dev)

**GitSynapse** n'est pas qu'un simple gestionnaire de code. C'est une forge logicielle complète (reprenant les concepts de GitLab et Backstage) entièrement repensée pour le paradigme de l'**AI Engineering**. 

Dans un monde où le code est massivement généré, audité et refactorisé par des LLMs (Large Language Models), les outils traditionnels ne suffisent plus. GitSynapse orchestre la collaboration hybride entre les agents IA autonomes et les experts humains à travers un workflow **Human-in-the-Loop (HITL)** strict et sécurisé.

## 🗺️ Roadmap & Architecture Globale

GitSynapse est conçu de manière modulaire. Le développement est divisé en trois piliers majeurs :

### 1. Le Moteur Core (Git FaaS Layer) - *En cours*
Le cœur du réacteur. Un backend cloud-native écrit en **Rust**.
- Agit comme un pare-feu cognitif entre l'IA et le code source.
- Exécute des clones partiels et des commits 100% en mémoire (`in-memory` via `git2-rs`).
- Expose une API RESTful stricte (`axum`) pour les agents IA (Argo Workflows).

### 2. Le Moteur d'Orchestration (CI/CD/CT) - *À venir*
Le pipeline asynchrone qui donne vie à l'IA.
- **Continuous Training (CT) :** Fine-tuning automatique des modèles locaux basés sur les interactions GitLab (DPO/RLHF).
- **Shadow Testing :** Déploiement comparatif automatique (legacy vs nouveau code généré) via Istio/Linkerd.
- Routage multi-agents (Junior/Senior) pour la revue de code.

### 3. Le Portail Développeur (Frontend & Dashboards) - *À venir*
L'interface de supervision humaine, conçue pour la clarté et la rapidité.
- Écrit en **Vanilla JS** pour des performances extrêmes et un build instantané.
- **Human Approval Hub :** Interface de revue par intention (et non plus ligne par ligne).
- **Time-Travel Debugger :** Dashboards visuels permettant de relancer un raisonnement IA à une étape précise en cas d'échec.

## 🚀 Développement piloté par les Spécifications (SDD)

Ce projet est construit de A à Z en utilisant la méthode **Spec-Driven Development** avec le standard [OpenSpec](https://openspec.dev). Le code source est généré et itéré par des agents de codage guidés par des propositions strictes.

Consultez le dossier `/openspec` pour comprendre les décisions d'architecture.

### Workflow d'itération actuel :
1. `init-git-faas-rust` : Création du MVP FaaS (Core Engine).
2. `setup-ci-pipeline` : Intégration Continue (Lint, Test, Build).

## 📦 Lancement Rapide du Core (Local)

*Assurez-vous d'avoir Rust et Cargo installés.*

```bash
git clone [https://github.com/votre-orga/gitsynapse.git](https://github.com/votre-orga/gitsynapse.git)
cd gitsynapse
export GITLAB_TOKEN="votre_token_personnel"
cargo run