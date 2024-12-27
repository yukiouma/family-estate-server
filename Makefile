.PHONY: build
build:
	docker build --network=host -t family-estate/server:v0.1.1 .

.PHONY: dev
dev:
	@if [ -z "$$SERVER_PORT" ]; then \
		echo "SERVER_PORT is missing"; \
	elif [ -z "$$DATABASE_URL" ]; then \
		echo "DATABASE_URL is missing"; \
	else \
		docker run -d --rm --name family-estate-server -p ${SERVER_PORT}:${SERVER_PORT} -e SERVER_PORT=${SERVER_PORT} -e DATABASE_URL=${DATABASE_URL} family-estate/server:v0.1.0; \
	fi