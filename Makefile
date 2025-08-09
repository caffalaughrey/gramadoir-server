IMAGE_NAME    := caffalaughrey/gramadoir
IMAGE_TAG     := latest

.PHONY: all docker-build

# Default: fetch deps and build the Docker image
all: docker-build

# Build the Docker image
docker-build:
	@echo "Building Docker image $(IMAGE_NAME):$(IMAGE_TAG)…"
	docker build \
	  -t $(IMAGE_NAME):$(IMAGE_TAG) \
	  .
