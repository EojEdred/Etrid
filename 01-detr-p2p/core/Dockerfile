
FROM golang:1.21
WORKDIR /app
COPY . .
RUN go build -o detr main.go
CMD ["./detr"]
