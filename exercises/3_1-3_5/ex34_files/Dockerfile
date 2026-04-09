FROM docker:latest
WORKDIR /usr/src/dir

COPY . .

RUN chmod +x builder.sh

ENTRYPOINT ["./builder.sh"]
