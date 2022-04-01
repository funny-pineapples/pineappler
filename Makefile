build:
	@docker build -t pineappler .
deploy:
	@docker-compose up -d
