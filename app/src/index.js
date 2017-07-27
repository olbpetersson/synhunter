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
      board: null,
      gameInput: null,
      debug: false
    }
  },
  computed: {
    inputEnabled() {
      if (!this.board || !this.board.gameState) {
        return false;
      }
      return this.board.currentTurn.tile &&
             this.board.currentTurn.team === this.board.team.id;
    },
    inputSpyEnabled() {
      if (!this.board || !this.board.gameState) {
        return false;
      }
      return this.board.currentTurn.tile && !this.board.isLeader &&
             this.board.currentTurn.team !== this.board.team.id;
    },
    inputLabel() {
      if (!this.board || !this.board.gameState) {
        return false;
      }
      return this.board.isLeader ? 'Give an answer' : 'Give a synonym';
    },
    color() {
      return this.board && this.board.gameState ? this.board.team.color : 'transparent';
    }
  },
  mounted() {
    const BASE_URL = 'ws://54.171.223.125:1337';

    this.socket = new WebSocket(BASE_URL);
    this.board = this.$refs.board;
    this.gameInput = this.$refs.gameinput;
    console.log("created a gameInput", this.gameInput);

    this.socket.onopen = () => {
      let payload = new JsonRpc('game_subscribe', [], 1);
      this.send(payload);
    };

    this.socket.onmessage = (e) => {
      let response = JSON.parse(e.data);
      if (response.method) {
        let gameState = new GameBoard(response.params.result.board.teams,
                                      response.params.result.board.tiles,
                                      response.params.result.turns);
        console.log("Subscribed to game", gameState);
        this.board.$emit('game_state', gameState);
      } else if (response.result) {
        let uuid = response.result;
        console.log("Creating a player with uuid", uuid, response);
        this.board.$emit('create_player', uuid);
      }
    };
    this.socket.onerror = (e) => console.log("Got a websocket error", e);

    this.$on('send', (e) => {
      this.socket.send(JSON.stringify(e));
    });
  },
  methods : {
    send(e) {
      console.log("Sending via websocket: ", e);
      this.socket.send(JSON.stringify(e));
    },
    submit() {
      let method = this.board.isLeader ? "submit_answer" : "submit_hint";
      console.log("sending with method name and value", method, this.gameInput.value);
      let payload = new JsonRpc(method, [this.gameInput.value],999);
      this.send(payload);
    },
    resetGame() {
      let payload = new JsonRpc("reset_game", [], 666);
      this.send(payload);
    }
  }
});