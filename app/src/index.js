import 'vue-material-css';

import Vue from 'vue';
import VueMaterial from 'vue-material';

import Board from 'Board.vue';
import GameInput from 'GameInput.vue';
import GameBoard from 'GameBoard'
import Player from 'Player';
import JsonRpc from 'JsonRpc'

Vue.use(VueMaterial);

const app = new Vue({
  el: '#app',
  components: {Board, GameInput},
  data() {
    return {
      socket: null,
      board: null
    }
  },
  mounted() {
    const BASE_URL = 'ws://54.171.223.125:1337';

    this.socket = new WebSocket(BASE_URL);
    this.board = this.$refs.board;
    this.gameInput = this.$refs.gameInput;

    console.log("created board", this.board, this.gameInput);

    this.socket.onopen = () => {
      let payload = new JsonRpc('game_subscribe', [], 1);
      console.log("Sending gamesubscribe", payload);
      this.send(payload);

    };

    this.socket.onmessage = (e) => {
      let response = JSON.parse(e.data);
      console.log("RESPONSUSUSUSUSU", response);
      if (response.method) {
        let gameState = new GameBoard(response.params.result.turn, response.params.result.board.teams, response.params.result.board.tiles);
        console.log("Subscribed to asdf", gameState);
        this.board.$emit('game_state', gameState);
      } else if (response.result) {
        let uuid = response.result;
        console.log("Creating a player with uuid", uuid, response);
        this.board.$emit('create_player', uuid);
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
    },
  }
});