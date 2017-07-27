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
      color: 'transparent'
    }
  },
  watch: {
    'board.isLeader': function(isLeader) {
      if (this.gameInput && this.gameInput.inputEnabled) {
        this.gameInput.label = isLeader ? 'Give an answer' : 'Give a synonym';
      }
    },
    'board.currentTurn': function(turn) {
      if(this.gameInput) {
        this.gameInput.inputEnabled = turn && turn.tile ? true : false;
      }
    },
    'board.player.team': function(color) {
      this.updateColor();
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
    updateColor() {
      if (!this.board.player || !this.board.gameState) {
        return;
      }
      const uuid = this.board.player.uuid;
      for (let team of this.board.gameState.teams) {
        if (team.leader === uuid) {
          this.color = team.color;
          return;
        }
        for (let player of team.players) {
          if (player === uuid) {
            this.color = team.color;
            return;
          }
        }
      }
    },
    resetGame() {
      let payload = new JsonRpc("reset_game", [], 666);
      this.send(payload);
    }
  }
});