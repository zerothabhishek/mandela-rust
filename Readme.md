
## Mandela Rust

Mandela is a Websocket pub-sub library written in Rust. It uses Redis as the pub-sub backend.


## The idea

Mandela is somewhat inspired by [Rails ActionCable](https://guides.rubyonrails.org/action_cable_overview.html) 

There is a server that listens for Websocket connections. A client can Subcribe to a channel (say _announcements#456_) and send messages to it. All clients subscribed to that channel will receive the message.  

We can run multiple instance of the server to distribute the load.  

Internally, the messages are wired through Redis pub-sub.  

Each server maintains a list of all connected clients, and also an active connection to Redis. When a new message arrives, it is sent to Redis (after some checks). Redis publishes the message, and all servers get it. They then deliver the message to all the websocket clients subscribed to the channel.  


Channels are defined as Json. There are examples in _channel_config_ folder.

## The client

A Websocket client can send messages as follows:

Susbcribe:

```
  {
    "m": {
      "ch": "Announcements#456",
      "t": "Sub"
    },
    "d":""
  }
```

Unsubscribe:

```
  {
    "m": {
      "ch": "Announcements#456",
      "t": "Unsub"
    },
    "d":""
  }
```

Send a message:

```
  {
    "m": {
      "ch": "Announcements#456",
      "t": "Msg"
    },
    "d": "Hello World"
  }
```

These can be sent over a usual Websocket connection.  
Example:

```
var ws = new WebSocket("ws://localhost:8080/ws");
ws.onopen = function() {
  ws.send(JSON.stringify({
    "m": {
      "ch": "Announcements#456",
      "t": "Sub"
    },
    "d":""
  }));
};
ws.onmessage = function(event) {
  console.log(event.data);
};
```


## The server

The server can be started with the following command:

```
mandela-server -c channel_config_file.txt
```

The server will read the channel configuration file and start listening for Websocket connections.

The channel configuration file is a text file with one channel per line.

The format of the channel configuration file is as follows:

```
{
  "label": "Announcements",
}
```

## How to use:


Build and run the binary mandela-server to start the server.

```
cargo build
./target/debug/mandela-server -c samples/channel_configs_files.txt
```

And then run the client:

```
cd client
npm install -g serve  ## or use any web server 
serve .
```

Open the browser at http://localhost:5000/


## License

MIT License
