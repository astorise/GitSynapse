# Spécifications : Pipeline CD et Infrastructure Knative

## ADDED Requirements

### Requirement: Déploiement Automatisé GKE
Le code fusionné sur `main` MUST être déployé sur le cluster GKE automatiquement.

#### Scenario: Build et Push vers Artifact Registry
- **WHEN** le job CI `lint-and-test` réussit sur la branche `main`
- **THEN** le job `deploy` s'authentifie via Workload Identity
- **THEN** l'image Docker est construite et poussée sur `europe-west9-docker.pkg.dev/gitsynapse/...`
- **THEN** la commande `kubectl apply` est exécutée pour mettre à jour le service Knative.

### Requirement: Lissage de l'Autoscaling
Le service Knative SHALL ne pas s'éteindre de manière agressive entre deux réflexions de l'IA.

#### Scenario: Configuration du Window Knative
- **WHEN** le manifeste `knative-service.yaml` est déployé
- **THEN** l'annotation `autoscaling.knative.dev/window` MUST être présente et configurée à au moins `120s`.
- **THEN** l'annotation `autoscaling.knative.dev/target` (concurrence ciblée par pod) MUST être définie (ex: `10`).

### Requirement: Fondation du stockage des Repositories
L'infrastructure MUST déclarer le CRD `GitWorkspace` et préparer le montage du bucket.

#### Scenario: Manifeste du CRD
- **WHEN** l'infrastructure est déployée
- **THEN** le cluster possède un CRD `GitWorkspace` contenant les champs `repoUrl`, `branch`, et `bucketPrefix`.
#### Scenario: Volume Mount dans Knative
- **WHEN** le pod Knative FaaS démarre
- **THEN** il possède un `volumeMount` pointant vers `/mnt/workspaces` soutenu par un volume CSI configuré pour le bucket cloud.
