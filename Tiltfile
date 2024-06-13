# 自動ビルド後に実行するカスタムフックを定義
local_resource(
    'prune_images',
    'docker system prune -a -f --volumes',
    deps=['compose.yaml', 'docker/Dockerfile'],  # 依存するファイルを指定
)
docker_compose('compose.yaml')
docker_build('web-dev', context='.', only='./app', ignore='./app/target', dockerfile='docker/Dockerfile', target='dev')