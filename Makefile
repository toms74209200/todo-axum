OPENAPI_SPEC = reference/spec.yaml

.PHONY: openapi
openapi: ## Generate rust code from OpenAPI spec
	openapi-generator-cli generate -i ${OPENAPI_SPEC} -g rust-axum -o ./openapi_gen

.PHONY: openapi-test
openapi-test: ## Generate python test client code from OpenAPI spec
	openapi-generator-cli generate -i ${OPENAPI_SPEC} -g python -o ./test/openapi_gen

help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'