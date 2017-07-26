<template>
<div>
  <md-layout v-for="(row, i) in parseInt(height)" class="row">
    <md-layout v-for="(tile, j) in parseInt(width)" md-gutter md-align="center">
      <md-card>
        <md-card-actions>
          <md-button @click="ping(i, j)">Ping</md-button>
        </md-card-actions>
        <md-card-content>
          {{i}} {{j}}
        </md-card-content>
      </md-card>
    </md-layout>
  </md-layout>
</div>
</template>
<script>
// const BASE_URL = 'ws://synhunter.mirkk.eu:1337';
const BASE_URL = 'ws://54.171.223.125:1337';

export default {
  data() {
    return {
      socket: null
    };
  },
  props: ['width', 'height'],
  created() {
    console.log('Board initialized', this.width, this.height);
    this.socket = new WebSocket(BASE_URL);

    // this.socket.onopen = () => this.socket.send('Ping');
    this.socket.onmessage = (e) => console.log('Server:', e);
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
<style>
.row {
  padding-top: 12px;
}
</style>