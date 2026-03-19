# Spécifications : Pipeline CI/CD

## Purpose
Définir les exigences du pipeline CI/CD pour le projet GitSynapse, garantissant la qualité du code Rust et la validité de l'image Docker à chaque contribution.

## Requirements

### Requirement: Déclenchement du workflow
Le workflow SHALL s'exécuter automatiquement sur les événements standards de collaboration.

#### Scenario: Push et Pull Request
- **WHEN** un développeur ou un agent IA pousse des commits sur la branche `main` ou ouvre une Pull Request ciblant `main`
- **THEN** le workflow nommé "Rust CI" se déclenche automatiquement.

### Requirement: Validation du code Rust (Lint & Test)
Le code MUST respecter les standards idiomatiques de Rust.

#### Scenario: Répertoire de travail des steps cargo
- **GIVEN** que la crate Rust se trouve dans le sous-répertoire `git-faas-layer/`
- **THEN** chaque step cargo (`fmt`, `clippy`, `test`) SHALL spécifier `working-directory: git-faas-layer` pour localiser le `Cargo.toml`.

#### Scenario: Échec sur non-respect du formatage
- **WHEN** le code n'est pas formaté selon `cargo fmt`
- **THEN** l'étape `cargo fmt --all -- --check` échoue et bloque le pipeline.

#### Scenario: Échec sur warnings Clippy
- **WHEN** le linter détecte des mauvaises pratiques
- **THEN** l'étape `cargo clippy --all-targets --all-features -- -D warnings` échoue (les warnings sont traités comme des erreurs bloquantes).

#### Scenario: Succès des tests unitaires
- **WHEN** le code compile et les tests passent
- **THEN** l'étape `cargo test --verbose` se termine avec succès et permet le passage au job suivant.

### Requirement: Validation de l'image Docker
Le `Dockerfile` MUST toujours être fonctionnel.

#### Scenario: Contexte de build Docker
- **GIVEN** que le `Dockerfile` se trouve dans `git-faas-layer/`
- **THEN** le job `docker-build` SHALL utiliser `context: git-faas-layer` dans l'action `docker/build-push-action`.

#### Scenario: Build de l'image FaaS
- **WHEN** le job `lint-and-test` est terminé avec succès
- **THEN** le job `docker-build` utilise l'action officielle `docker/build-push-action`
- **THEN** l'image est construite avec succès (sans être poussée vers un registre externe).

### Requirement: Compatibilité de l'image Rust dans le Dockerfile
L'image Rust utilisée dans le stage `builder` MUST être compatible avec la `edition` déclarée dans `Cargo.toml`.

#### Scenario: Support de Rust edition 2024
- **GIVEN** que `Cargo.toml` déclare `edition = "2024"`
- **THEN** l'image Docker du stage `builder` SHALL utiliser Rust ≥ 1.85 (première version supportant l'edition 2024).

### Requirement: Dépendances système pour la build vendorisée
Le stage `builder` MUST installer toutes les dépendances système requises par les dépendances Rust vendorisées.

#### Scenario: `perl` requis pour OpenSSL vendorisé
- **GIVEN** que la dépendance `git2` utilise la feature `vendored-openssl`
- **THEN** le `Dockerfile` SHALL installer `perl` via `apt-get` dans le stage `builder`
- **BECAUSE** le script de configuration d'OpenSSL (`./Configure`) est un script Perl absent des images `slim`.
