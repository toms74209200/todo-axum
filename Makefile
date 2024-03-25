OPENAPI_SPEC = https://raw.githubusercontent.com/toms74209200/openapi-todo-example/master/reference/spec.yaml

.PHONY: openapi ## Generate rust code from OpenAPI spec
openapi:
	openapi-generator-cli generate -i ${OPENAPI_SPEC} -g rust-axum -o ./openapi_gen

help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'