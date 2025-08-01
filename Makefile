run-docker:
	docker compose down && docker compose up -d --build

create-mig-file:
	@read -p "Enter migration name: " name; \
	if [ -z "$$name" ]; then \
		echo "Migration name cannot be empty"; \
		exit 1; \
	fi; \
	sqlx migrate add "$$name"

run-migrations:
	sqlx migrate run

git-pull:
	git pull origin main
	
git-push:
	git add .
	git commit -m "Update"
	git push origin main
