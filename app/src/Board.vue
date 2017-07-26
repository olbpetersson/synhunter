<template>
  <md-layout md-column>
    <div v-if="currentPlayer && currentPlayer.isLeader">
      <md-layout md-align="center">
        <span class="md-headline">You are the leader!</span>
      </md-layout>
      <md-layout md-align="center" v-for="hint in gameState.turn.hints">
        <span>{{hint}}</span>
      </md-layout>
    </div>
    <md-layout md-column class="container" v-if="gameStateView">
      <md-layout v-for="row in gameStateView" key="row" class="row">
        <md-layout v-for="tile in row" key="tile" class="tile" md-gutter>
          <tile :tile="tile" :click="chooseTile"></tile>
        </md-layout>
      </md-layout>
    </md-layout>
    <md-layout md-column md-align="center" md-vertical-align="center" v-else>
      <md-spinner md-indeterminate></md-spinner>
      <span class="md-title loading-text">Herp Derp...</span>
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
    computed: {
      isLeader: function() {
        return this.currentPlayer && this.currentPlayer.isLeader;
      }
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
        this.updatePlayerState();
      });
      this.$on('create_player', (uuid) => {
        this.currentPlayer.uuid = uuid;
        console.log("asdf player", this.currentPlayer.uuid, uuid);
        this.updatePlayerState();
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
      updatePlayerState() {
        if(this.currentPlayer && this.gameState){
          this.gameState.teams.forEach((team) => {
            team.players.forEach((player) => {
              if (player.id === this.currentPlayer.uuid) {
                this.$set(this.currentPlayer, 'isLeader', false);
                this.$set(this.currentPlayer, 'team', team.color);
              }
            });

            if (team.leader === this.currentPlayer.uuid) {
              this.$set(this.currentPlayer, 'isLeader', true);
              this.$set(this.currentPlayer, 'team', team.color);
            }
          });
        }
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