ENV_VARS = \
	-e TCP_0=${TCP_0} \
	-e TCP_1=${TCP_1} \
	-e TCP_2=${TCP_2} \
	-e TCP_3=${TCP_3} \
	-e TCP_4=${TCP_4} \
	-e TCP_5=${TCP_5} \
	-e TCP_6=${TCP_6} \
	-e TCP_7=${TCP_7} \
	-e TCP_8=${TCP_8} \
	-e TCP_9=${TCP_9} \
	-e TCP_10=${TCP_10} \
	-e TCP_11=${TCP_11} \
	-e TCP_12=${TCP_12} \
	-e TCP_13=${TCP_13} \
	-e TCP_14=${TCP_14} \
	-e TCP_15=${TCP_15} \
	-e TCP_16=${TCP_16} \
	-e TCP_17=${TCP_17} \
	-e TCP_18=${TCP_18} \
	-e TCP_19=${TCP_19} \
	-e TCP_20=${TCP_20} \
	-e TCP_21=${TCP_21} \
	-e TCP_22=${TCP_22} \
	-e TCP_23=${TCP_23} \
	-e TCP_24=${TCP_24} \
	-e TCP_25=${TCP_25} \
	-e TCP_26=${TCP_26} \
	-e UDP_0=${UDP_0} \
	-e UDP_1=${UDP_1} \
	-e UDP_2=${UDP_2} \
	-e UDP_3=${UDP_3} \
	-e UDP_4=${UDP_4} \
	-e UDP_5=${UDP_5} \
	-e UDP_6=${UDP_6} \
	-e UDP_7=${UDP_7} \
	-e UDP_8=${UDP_8} \
	-e UDP_9=${UDP_9} \
	-e UDP_10=${UDP_10} \
	-e UDP_11=${UDP_11} \
	-e UDP_12=${UDP_12} \
	-e UDP_13=${UDP_13} \
	-e UDP_14=${UDP_14} \
	-e UDP_15=${UDP_15} \
	-e UDP_16=${UDP_16} \
	-e UDP_17=${UDP_17} \
	-e UDP_18=${UDP_18} \
	-e UDP_19=${UDP_19} \
	-e UDP_20=${UDP_20} \
	-e UDP_21=${UDP_21} \
	-e UDP_22=${UDP_22} \
	-e UDP_23=${UDP_23} \
	-e UDP_24=${UDP_24} \
	-e UDP_25=${UDP_25} \
	-e UDP_26=${UDP_26} \
	
# Image Name + other ENV vars
IMAGE_NAME=particle_project

REPO_PORT_FLAG= -v "$(shell pwd):/app/src"

DOCKER_FLAG= -v /var/run/docker.sock:/var/run/docker.sock

.PHONY: build interactive

build:
	docker build . -t ${IMAGE_NAME} 

interactive:
	docker run -it ${REPO_PORT_FLAG} ${DOCKER_FLAG} ${ENV_VARS} ${IMAGE_NAME} /bin/bash