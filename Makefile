build-dev:
	@docker-compose build 

up-dev: 	
	@docker-compose up ranker-server

down-dev:
	@docker-compose down

clear-none-docker-image:
	@docker rm $(docker ps -a -q) 
	@docker rmi $(docker images -f "dangling=true" -q)
