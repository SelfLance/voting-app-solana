FROM    node:20
WORKDIR /voting-dapp
COPY  package*.json ./
RUN     npm install
COPY    . .

CMD     ["npm", "dev"]