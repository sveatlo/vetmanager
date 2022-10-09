COMPONENTS=backend frontend
ENVS=dev prod

.PHONY: dev
dev: $(foreach component,$(COMPONENTS),$(component)-dockerfile-dev)
	docker-compose up --build

.PHONY: clean
clean: $(foreach component,$(COMPONENTS),$(component)-clean)
	docker-compose down
	rm -rf Dockerfile $(foreach cmd,$(CMDS), Dockerfile.$(cmd))
	rm -rf sonar-project.properties

# 1 - component
# 2 - target
define PASSALONG_template =
$(1)-$(2):
	@echo "# running \"$(2)\" in $(1)"
	@make -s -C $(1) $(2)
endef
$(foreach env,$(ENVS),$(foreach component,$(COMPONENTS),$(eval $(call PASSALONG_template,$(component),dockerfile-$(env)))))
$(foreach component,$(COMPONENTS),$(eval $(call PASSALONG_template,$(component),clean)))
