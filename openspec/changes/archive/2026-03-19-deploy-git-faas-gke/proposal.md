# Proposal: Déploiement GKE (Knative), Autoscaling et GitWorkspace CRD

## Objectif
Automatiser le déploiement continu (CD) de la couche Git FaaS vers notre cluster Google Kubernetes Engine, en optimisant la gestion de la charge (autoscaling) et en introduisant la fondation pour le stockage persistant des dépôts Git.

## Portée (Scope)
1. **Pipeline de Déploiement Continu (GitHub Actions)** : Pousser l'image Docker sur Google Artifact Registry (GAR) et déployer le manifeste sur GKE via Workload Identity (sans mot de passe).
2. **Knative Autoscaling** : Configuration du lissage des requêtes (`window`) pour s'adapter au trafic asynchrone et "en rafale" (burst) typique des agents IA.
3. **Architecture de Stockage** : Définition d'un CRD Kubernetes `GitWorkspace` et configuration du montage du bucket (S3/GCS) via le driver CSI dans le pod FaaS.