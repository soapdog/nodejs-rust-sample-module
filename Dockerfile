FROM rust:1.19.0

RUN apt update && \
    apt install build-essential sudo -y && \
    curl -sL https://deb.nodesource.com/setup_8.x | sudo -E bash - && \
    apt-get install -y nodejs


WORKDIR /usr/src/myapp
COPY . .

RUN npm i -g neon-cli && npm i && neon build

EXPOSE 5000
CMD ["node", "index.js"]