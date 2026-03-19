# Proposal: Création du MVP de la couche Git FaaS

## Objectif
Créer un microservice Serverless en Rust capable de servir d'interface entre nos LLMs locaux (Argo Workflows) et notre GitLab. Ce composant évite de donner un accès Git CLI direct ou des tokens GitLab bruts aux agents IA.

## Portée (Scope)
Implémentation d'un serveur HTTP `axum` exposant 4 endpoints RESTful essentiels pour le cycle de vie de l'IA :
1. Extraction de contexte ciblé (Shallow read).
2. Création de commits (In-memory write).
3. Génération de diffs structurés (Review).
4. Création de branche de résolution (Human-in-the-Loop).

Ce service sera packagé dans une image Docker distroless et accompagné d'un manifeste de déploiement Knative configuré pour Linkerd.