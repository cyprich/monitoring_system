FROM node:25-alpine3.22 AS builder 
WORKDIR /app 

COPY package.json package-lock.json ./
RUN npm install 

COPY . . 
RUN npm run build 

FROM alpine:3.23.3
COPY --from=builder /app/dist/ /srv
