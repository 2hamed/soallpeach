FROM golang:1.13

COPY prime.go /app

WORKDIR /app

RUN go build prime.go

ENTRYPOINT [ "/app/prime" ]