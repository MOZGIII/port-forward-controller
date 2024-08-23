group "default" {
  targets = ["main"]
}

target "main" {
  inherits = ["docker-metadata-action-main"]
  dockerfile = "Dockerfile"
  target = "main"
}

# Targets to allow injecting customizations from Github Actions.

target "docker-metadata-action-main" {}
