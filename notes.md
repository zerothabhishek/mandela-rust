

## 6.apr.2024

ws_tokio_broadcasts2

- broadcasts with tokio       y
- error handling              .
- remove connections          y
- Approach review - AI        .
- test with many connections  .
- integrate Redis             .

- json serialize/deserialize for message   .
- 

## 9.apr.2024

- broadcasts with tokio         y
- remove connections            y
- integrate Redis               y
- error handling                ~
- Approach review - AI          .
- test with many connections    .
- json for message              .
- redis.publish with metadata   .
- try with tokio alternatives   .
- RPC with Rails app            .


## 18.apr.2024

- Define Message struct
- Implement findChannel
- Implement processMessage:
  - SUB, UNSUB, HERE_NOW
- 

## 21.apr.2024

TODO
- add: handle_data
- test
- implement: find_channel
- add: handle_unsub
- add: handle_disconnect

## 1.may.2024

TODO:
- impl: MandelaMsgInternal to Redis
- impl: find_channel
- impl: handle_unsub
- impl: handle_disconnect

## 7.may.2024

TODO:
- impl: MandelaMsgInternal to Redis   .
- impl: find_channel                  y
- impl: handle_unsub                  .
- impl: handle_disconnect             .

## 16.may.2024

- fix: Data messages not working      y
- impl: http -> ws upgrade            . 
- impl: MandelaMsgInternal to Redis   ~
- impl: handle_unsub                  Y
- impl: handle_disconnect             Y
- impl: introduce ENV                 .
- impl: add SSL support               .
- impl: add unit tests                .




To Test in browser:
ws = new WebSocket("ws://127.0.0.1:9001");
ws.send(JSON.stringify({m: {ch:"Collab", id:"123", t: 'Sub'}, d: ''}));

ws.send(JSON.stringify({m: {ch:"Collab", id:"123", t: 'Data'}, d: 'Hey!'}));


## 19.may.2024

- impl: introduce ENV                 Y
- impl: handle_disconnect             .
- impl: MandelaMsgInternal to Redis   ~
- impl: turn main into library        .
- impl: http -> ws upgrade            . 
- impl: add SSL support               .
- impl: add unit tests                .

## 21.may.2024

- impl: handle_disconnect             .
- impl: MandelaMsgInternal to Redis   ~
- impl: turn main into library        .
- impl: http -> ws upgrade            . 
- impl: add SSL support               .
- impl: add unit tests                .

## 28.may.2024

- impl: handle_disconnect             .
- impl: MandelaMsgInternal to Redis   ~
- impl: turn main into library        .
- impl: http -> ws upgrade            . 
- impl: add SSL support               .
- impl: add unit tests                .


## 1.june.2024

- impl: handle_disconnect             .
- impl: MandelaMsgInternal to Redis   ~
- impl: turn main into library        .
- impl: http -> ws upgrade            . 
- impl: add SSL support               .
- impl: add unit tests                .


## 6.june.2024

- channel structure: label#id       y
- frontend demo                     ~
- unit tests                        .
- MandelaMsgInternal                .
- here_now                          .
- frontend demo - Rust based        x

## 10.june.2024

- unit tests                        ~
- MandelaMsgInternal                .
- feature: here_now                 .
- feature: repeat                   .
- bug: reconnection                 .


## 14.june.2024

- unit tests                        ~
- MandelaMsgInternal                .
- main -> library                   .
- feature: here_now                 .
- feature: repeat                   ~
- bug: reconnection                 .



