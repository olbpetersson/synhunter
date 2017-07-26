<template>
  <md-layout md-column class="container">
    <md-layout v-for="(row, i) in parseInt(height)" class="row">
      <md-layout v-for="(tile, j) in parseInt(width)" class="tile" md-gutter>
        <md-card>
          <md-button @click="ping(i, j)">({{i}},{{j}})</md-button>
        </md-card>
      </md-layout>
    </md-layout>
  </md-layout>
</template>

<script>
  import Player from './Player'

  // const BASE_URL = 'ws://synhunter.mirkk.eu:1337';
  const BASE_URL = 'ws://54.171.223.125:1337';

  export default {
    data() {
      return {
        socket: null,
        currentPlayer: null
      };
    },
    props: ['width', 'height'],
    created() {
      console.log('Board initialized', this.width, this.height);
      this.socket = new WebSocket(BASE_URL);

      this.socket.onopen = () => {
        let payload = {
          jsonrpc: "2.0",
          method: 'game_subscribe',
          params: [],
          id: 1
        };
        console.log("Sending gamesubscribe", payload);
        this.socket.send(JSON.stringify(payload));

      };

      this.socket.onmessage = (e) => {
        let response = JSON.parse(e.data);
        if (response.method) {
          console.log("Subscribed to", response.method)
        } else {
          let uuid = response.result;
          console.log("Creating a player with uuid", uuid);
          this.currentPlayer = new Player(uuid, undefined, undefined);
        }
      };
      this.socket.onerror = (e) => console.log("Got a websocket error", e);
    },
    methods: {
      ping(i, j) {
        console.log('pinging...');
        let payload = {
          jsonrpc: "2.0",
          method: 'foo',
          params: [(i*parseInt(this.height)) + j],
          id: 0
        };
        this.socket.send(JSON.stringify(payload));
      }
    }
  };
</script>
<style scoped>
  .container {
    margin-bottom: 8px;
  }
  .row {
    padding-top: 8px;
  }
  .tile {
    min-width: 0;
  }
  .md-card {
    width: 100%;
    margin: 0 8px;
  }
  .md-card button {
    min-width: 0;
    margin: 0;
  }
</style>