
Steps

```bash
cargo intsall trunk

rustup target add wasm32-unknown-unknown

# create a new project, or use an existing one
# ...

# run the dev server
trunk serve

```

## Docker issues

```bash
sudo apt-get remove docker docker-engine docker.io
sudo apt install docker.io
docker --version
sudo docker run hello-world
sudo docker images
sudo docker ps -a
sudo docker ps
sudo docker search hello-world
sudo apt install docker-compose
sudo docker-compose --profile nodejs-express up

git clone https://github.com/brooks-builds/full-stack-todo-rust-course.git full-stack-todo
cd full-stack-todo
sudo systemctl stop postgresql
sudo docker-compose --profile nodejs-express up
```
