-include ./sedder_helper.mk

CMDS := api
ENVS := prod dev

CC = cargo

BUILDIMAGE = rust:1.54-buster
RUNIMAGE = alpine
REGISTRY ?= registry.vunder.dev/trivet/vetmanager
GITLAB_USER ?= gitlab-ci-token
GITLAB_ACCESS_TOKEN ?= $(CI_JOB_TOKEN)

# project_name is defined as the last part of package/git repo
# PROJECT_NAME = $(shell echo '$(GO_PROJECT_PACKAGE)' | awk -F'/' '{print $$NF}' )
PROJECT_NAME = vetmanager-backend
CMD_PKG_PATH = ./cmd/$(CMD_NAME)
CMD_CONFIG_FILENAME ?= $(CMD_NAME).yml
CMD_BIN_FILE = $(PROJECT_NAME)-$(CMD_NAME)-$(OS)-$(ARCH)
CMD_BIN_PATH = $(BIN_DIR)/$(CMD_BIN_FILE)
CMD_IMAGE ?= $(REGISTRY)/$(PROJECT_NAME)/$(CMD_NAME):$(VERSION)

VERSION ?= $(shell git describe --tags --always --dirty 2>/dev/null || echo 'latest')
DATE = $(shell date +'%a, %d %b %Y %T %z')

all: build

dev: SERVICES?=$(CMDS) elasticsearch
dev: dockerfile-dev
	docker-compose up --build $(SERVICES) mysql nats

build: $(foreach cmd,$(CMDS),build-$(cmd))

define BUILD_template =
.PHONY: build-$(1)
build-$(1): CMD_NAME=$(1)
build-$(1):
	$(CC) build --release --bin $$(CMD_NAME)
endef
$(foreach cmd, $(CMDS), $(eval $(call BUILD_template,$(cmd))))

define RUN_template =
.PHONY: run-$(1)
run-$(1): CMD_NAME=$(1)
run-$(1):
	$(CC) run --bin $$(CMD_NAME) -- $$(ARGS)
endef
$(foreach cmd, $(CMDS), $(eval $(call RUN_template,$(cmd))))

define DOCKERFILE_template =
.PHONY: dockerfile-$(1)
dockerfile-$(1): $(foreach cmd,$(CMDS),dockerfile-$(cmd)-$(1))
endef
$(foreach env,$(ENVS),$(eval $(call DOCKERFILE_template,$(env))))

# 1-cmd 2-env
define DOCKERFILE_CMD_template =
.PHONY: dockerfile-$(1)-$(2)
dockerfile-$(1)-$(2): FILE=Dockerfile.$(2).in
dockerfile-$(1)-$(2): CMD_NAME=$(1)
dockerfile-$(1)-$(2):
	$$(call check_defined, $$(VARS))
	sed -E $$(PARAMS) $$(FILE) > Dockerfile.$(1)
endef
$(foreach env,$(ENVS),$(foreach cmd,$(CMDS),$(eval $(call DOCKERFILE_CMD_template,$(cmd),$(env)))))


define DOCKER_BUILD_template =
.PHONY: docker-build-$(1)
docker-build-$(1): $(foreach cmd,$(CMDS),docker-build-$(cmd)-$(1))
endef
$(foreach env,$(ENVS),$(eval $(call DOCKER_BUILD_template,$(env))))

# 1-cmd 2-env
define DOCKER_BUILD_CMD_template =
.PHONY: docker-build-$(1)-$(2)
docker-build-$(1)-$(2): FILE=Dockerfile.$(2).in
docker-build-$(1)-$(2): CMD_NAME=$(1)
docker-build-$(1)-$(2): dockerfile-$(1)-$(2)
	docker build -t $(REGISTRY)/$(1):$(VERSION) -f Dockerfile.$(1) .
endef
$(foreach env,$(ENVS),$(foreach cmd,$(CMDS),$(eval $(call DOCKER_BUILD_CMD_template,$(cmd),$(env)))))

cmds:
	@echo $(CMDS)

define ECHO_template =
.PHONY: $(1)
$(1): CMD_NAME=$(2)
$(1):
	@echo $(3)
endef
$(foreach cmd, $(CMDS), $(eval $(call ECHO_template,compose-image-tag-$(cmd),$(cmd),$$(CMD_IMAGE))))
$(eval $(call ECHO_template,compose-go-project-path,,$$(GO_PROJECT_PACKAGE)))

# sedder for sonar-properties
# generates sonar-properties target that seds ./sonar-project.properties.in with version etc. with VERSION etc -> output to ./sonar-project.properties
$(eval $(call SEDDER_TARGET_template,sonar-properties,sonar-project.properties.in,,sonar-project.properties))

clean:
	docker-compose down
	rm -rf Dockerfile $(foreach cmd,$(CMDS), Dockerfile.$(cmd))
	rm -rf sonar-project.properties

stats:
	scc --exclude-dir 'vendor,node_modules,data,.git,docker/etcdkeeper,utils' --wide
