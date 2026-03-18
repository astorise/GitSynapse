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

#### Scenario: Échec sur non-respect du formatage
- **WHEN** le code n'est pas formaté selon `cargo fmt`
- **THEN** l'étape `cargo fmt --all -- --check` échoue et bloque le pipeline.

#### Scenario: Échec sur warnings Clippy
- **WHEN** le linter détecte des mauvaises pratiques
- **THEN** l'étape `cargo clippy -- -D warnings` échoue (les warnings sont traités comme des erreurs bloquantes).

#### Scenario: Succès des tests unitaires
- **WHEN** le code compile et les tests passent
- **THEN** l'étape `cargo test --verbose` se termine avec succès et permet le passage au job suivant.

### Requirement: Validation de l'image Docker
Le `Dockerfile` MUST toujours être fonctionnel.

#### Scenario: Build de l'image FaaS
- **WHEN** le job `lint-and-test` est terminé avec succès
- **THEN** le job `docker-build` utilise l'action officielle `docker/build-push-action`
- **THEN** l'image est construite avec succès (sans être poussée vers un registre externe).
