<template>
  <md-layout>

    <button @click="chooseTile"> APA</button>
    <button @click="submitHint"> BEPA</button>

    <md-layout md-column class="container" v-if="gameStateView">
      <md-layout v-for="row in gameStateView" class="row">
        <md-layout v-for="tile in row" class="tile" md-gutter>
          <tile :tile="tile" :click="chooseTile"></tile>
        </md-layout>
      </md-layout>
    </md-layout>
    <md-layout md-column md-align="center" md-vertical-align="center" v-else>
      <md-spinner md-indeterminate></md-spinner>
      <span class="md-title loading-text">Herp Derp...</span>
    </md-layout>
    <md-layout v-if="currentPlayer && currentPlayer.isLeader">
        <strong>You are the leader!</strong>
        <div v-for="hint in gameState.turn.hints">
            <span>hint</span>
        </div>
    </md-layout>
  </md-layout>
</template>
<script>
  import Tile from 'Tile.vue';
  import JsonRpc from './JsonRpc';
  import Player from 'Player'
  // const BASE_URL = 'ws://synhunter.mirkk.eu:1337';

  export default {
    data() {
      return {
        currentPlayer: {},
        gameState: null,
        gameStateView: null
      };
    },
    props: ['width', 'height', 'send'],
    mounted() {
      console.log('Board initialized', this.width, this.height);

      this.$on('game_state', (state) => {
        console.log('updating our gamestate in the herpyderpyboard', state);
        this.gameState = state;
        this.gameStateView = {};
        for (let tile of state.tiles) {
          if (!this.gameStateView[tile.position[0]]) {
            this.gameStateView[tile.position[0]] = {};
          }
          this.gameStateView[tile.position[0]][tile.position[1]] = tile;
        }
        this.updatePlayerState(this);
      });
      this.$on('create_player', (uuid) => {
        this.currentPlayer.uuid = uuid;
        console.log("asdf player", this.currentPlayer.uuid, uuid);
        this.updatePlayerState(this);
      });
    },
    methods: {
      ping(i, j) {
        console.log('pinging...');
        let payload = new JsonRpc('foo', 1, 0);
        console.log("sending payload", payload);
        this.send(payload);
      },
      chooseTile(uuid) {
        let payload = new JsonRpc('choose_tile', [uuid], 2);
        console.log("sending choose tile", payload);
        this.send(payload);
      },
      submitHint() {
        let bepa = "foobar";
        let payload = new JsonRpc('submit_hint', [bepa], 2);
        console.log("sending submit hint", payload);
        this.send(payload);
      },
      updatePlayerState(component){
        if(component.currentPlayer && component.gameState){
          component.gameState.teams.forEach(function (team) {
            team.players.forEach(function (player) {
              if (player.id === component.currentPlayer.uuid) {
                component.currentPlayer.team = team.color;
              }
            });

            if (team.leader === component.currentPlayer.uuid) {
              component.currentPlayer.isLeader = true;
              component.currentPlayer.team = team.color;
            }
          });
        }
        console.log("currentPlayer", component.currentPlayer);
      }
    },
    components: {Tile}
  };
</script>
<style scoped>
  .loading-text {
    margin-top: 12px;
  }
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