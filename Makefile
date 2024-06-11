up:
	tilt up
up-prod:
	tilt up --file Tiltfile-prod
down:
	tilt down
ps:
	docker container ps
images:
	docker image ls

# bash-コンテナ名で実行すると bash に入れる
bash-%:
	docker container run -it --rm ${@:bash-%=%} bash