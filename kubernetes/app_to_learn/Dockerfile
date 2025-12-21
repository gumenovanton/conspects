FROM golang:latest AS builder

WORKDIR /build

COPY . .

RUN go mod tidy
RUN go build -o app .

FROM ubuntu:latest

WORKDIR /app

COPY --from=builder /build/app /app/app

EXPOSE 8080

CMD ["./app"]
