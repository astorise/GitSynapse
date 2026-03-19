# Design Document: Déploiement et Stockage Stateful pour FaaS

## 1. CD et Workload Identity Federation
Pour des raisons de sécurité, nous n'utiliserons pas de clé de compte de service (JSON) statique. Le workflow GitHub Actions s'authentifiera sur GCP en utilisant l'action `google-github-actions/auth` via OIDC (Workload Identity). L'image sera hébergée sur `europe-west9-docker.pkg.dev`.

## 2. Le lissage Knative (`autoscaling.knative.dev/window`)
Les agents IA (Argo Workflows) génèrent un trafic très particulier : ils envoient des blocs de code, puis font de longues pauses d'inférence GPU.
- Si le `window` est trop court, Knative va éteindre le pod FaaS pendant la pause de l'IA (Scale-to-Zero), provoquant un "Cold Start" à la requête suivante.
- Nous allons définir `autoscaling.knative.dev/window: "120s"`. Knative évaluera la moyenne des requêtes sur 2 minutes avant de décider d'éteindre ou de créer de nouveaux pods, lissant ainsi parfaitement la charge.

## 3. Le CRD `GitWorkspace` et le montage CSI
Pour le MVP, nous utilisons la puissance de GKE avec le driver **GCS Fuse CSI** (qui permet de monter un bucket cloud comme un dossier local `/mnt/workspaces`).
- **Le CRD `gitworkspaces.gitsynapse.dev`** : Il définira le lien entre une URL GitLab, une branche, et un préfixe dans le bucket.
- **Le Pod** : L'annotation `gke-gcsfuse/volumes: "true"` sera ajoutée au Service Knative pour que le conteneur Rust accède directement au cache Git sans utiliser sa propre RAM/Disque éphémère.