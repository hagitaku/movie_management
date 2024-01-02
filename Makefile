# ----------------------------------------------------
# 
# ----------------------------------------------------

.SILENT: help
.PHONY: help
help:
	echo "backend: 	Build backend"
	echo "barusu: 	滅びの呪文。全てのコンテナを根こそぎ消し去る"

.PHONY: backend
backend:
	docker compose up -d backend

.PHONY: log-backend
log-backend:
	docker compose logs -f backend

.PHONY: down
down:
	docker compose down 


.PHONY: barusu
barusu:
	docker-compose down --rmi all --volumes --remove-orphans

