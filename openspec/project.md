# Constitution du Projet : Git FaaS Layer

## Contexte
Ce projet est le cœur d'une usine logicielle pilotée par l'IA (AI Engineering). Il fournit une couche FaaS (Function-as-a-Service) permettant à des agents IA (s'exécutant dans des Argo Workflows) de lire et modifier le code source sur GitLab de manière sécurisée et asynchrone.

## Stack Technique Obligatoire
- **Langage** : Rust (Édition 2021).
- **Framework Web** : `axum` avec `tokio` (asynchrone).
- **Manipulation Git** : `git2-rs` (libgit2 binding). Pas d'appels à la commande CLI `git`.
- **Sérialisation** : `serde` et `serde_json`.
- **Infrastructure** : Kubernetes / Knative.
- **Service Mesh** : Linkerd (pour le mTLS natif).

## Règles d'Architecture
- **Stateless** : Les pods FaaS ne doivent conserver aucun état local persistant. Utiliser les clones en mémoire (in-memory Object Database) de `git2` ou un volume éphémère (`tmpfs`) si absolument nécessaire.
- **Sécurité** : Les endpoints HTTP ne sont exposés qu'en interne au cluster via le proxy Linkerd.
- **Performance** : Optimiser pour les démarrages à froid (Cold Starts) rapides typiques du Serverless.