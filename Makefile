# Image Name + other ENV vars
IMAGE_NAME=particle_project

.PHONY: build interactive

build:
	docker build . -t ${IMAGE_NAME} 

interactive:
	docker run -it -v "$(shell pwd):/app/src" ${IMAGE_NAME} /bin/bash