import 'vue-material-css';

import Vue from 'vue';
import VueMaterial from 'vue-material';

import Board from 'Board.vue';
import GameInput from 'GameInput.vue';
import Player from './Player';
import JsonRpc from './JsonRpc'

Vue.use(VueMaterial);

const app = new Vue({
  el: '#app',
  components: {Board, GameInput},
  data() {
    return {
      socket: null
    }
  },
  created() {
    const BASE_URL = 'ws://54.171.223.125:1337';

    this.socket = new WebSocket(BASE_URL);

    this.socket.onopen = () => {
      let payload = new JsonRpc('game_subscribe', [], 1);
      console.log("Sending gamesubscribe", payload);
      this.send(payload);

    };

    this.socket.onmessage = (e) => {
      let response = JSON.parse(e.data);
      if (response.method) {
        console.log("Subscribed to", response);
        this.$emit("game_state", )
      } else {
        let uuid = response.result;
        console.log("Creating a player with uuid", uuid, response);
        this.$emit('createPlayer', new Player(uuid, undefined, undefined));
      }
    };
    this.socket.onerror = (e) => console.log("Got a websocket error", e);

    this.$on('send', (e) => {
      console.log('sending from index', e);
      this.socket.send(JSON.stringify(e));
    });
  },
  methods : {
    send(e) {
      this.socket.send(JSON.stringify(e));
    }
  }
});