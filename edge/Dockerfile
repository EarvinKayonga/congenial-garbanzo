FROM node:13-slim


WORKDIR /usr/src/app
COPY package*.json ./

RUN npm install
RUN npm ci --only=production

# Bundle app source
COPY . .

ENV ADDR "http://localhost:9000"
EXPOSE 3000
CMD [ "node", "index.js" ]