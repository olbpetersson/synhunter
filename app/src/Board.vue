<template>
  <md-layout md-column>
    <div v-if="currentPlayer && currentPlayer.isLeader && !gameState.turn.tile">
      <md-layout md-align="center">
        <span class="md-headline">You are the leader!</span>
      </md-layout>
    </div>
    <md-layout md-column class="container" v-if="gameStateView && !gameState.turn.tile">
      <md-layout v-for="row in gameStateView" key="row" class="row">
        <md-layout v-for="tile in row" key="tile" class="tile" md-gutter>
          <tile :tile="tile" :click="chooseTile"></tile>
        </md-layout>
      </md-layout>
    </md-layout>
      <md-layout md-column class="container" v-if="currentPlayer && currentPlayer.isLeader && gameState.turn.tile ">
          {{gameState.turn.hints}}
          <md-layout v-for="hint in gameState.turn.hints" key="row" class="column">
              a hint!  {{hint}}
          </md-layout>
      </md-layout>
      -------
      <md-layout md-column class="container" v-if="currentPlayer && !currentPlayer.isLeader && gameState.turn.tile ">
          Give a synonym for: {{ findTileWord(gameState.turn.tile) }}
      </md-layout>
   <!-- <md-layout md-column md-align="center" md-vertical-align="center" v-else>
      <md-spinner md-indeterminate></md-spinner>
      <span class="md-title loading-text">Herp Derp...</span>
    </md-layout>-->
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
      },
      hasSelectedTile: function() {
        this.gameState ? this.gameState.turn.tile : undefined;
      }
    },
    props: ['width', 'height', 'send'],
    mounted() {
      this.$on('game_state', (state) => {

        this.$set(this, 'gameState', state);
        this.$set(this.gameState, 'turn', state.turn);

//        this.$set(this.gameState.turn, 'tile', state.turn.tile);
        this.gameStateView = {};
        for (let tile of state.tiles) {
          if (!this.gameStateView[tile.position[0]]) {
            this.gameStateView[tile.position[0]] = {};
          }
          this.gameStateView[tile.position[0]][tile.position[1]] = tile;
        }
        this.updatePlayerState();
        if(this.gameState.turn.hints){
          this.$set(this.gameState.turn, 'hints', state.turn.hints);
        }
      });
      this.$on('create_player', (uuid) => {
        this.currentPlayer.uuid = uuid;
        this.updatePlayerState();
      });
    },
    methods: {
      ping(i, j) {
        let payload = new JsonRpc('foo', 1, 0);
        this.send(payload);
      },
      chooseTile(uuid) {
        let payload = new JsonRpc('choose_tile', [uuid], 2);
        this.send(payload);
      },
      submitHint() {
        let bepa = "foobar";
        let payload = new JsonRpc('submit_hint', [bepa], 2);
        this.send(payload);
      },
      updatePlayerState() {
        if(this.currentPlayer && this.gameState){
          this.gameState.teams.forEach((team) => {
            team.players.forEach((uuid) => {
              if (uuid === this.currentPlayer.uuid) {
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
      },
      findTileWord(uuid) {
        return this.gameState.tiles.find((tile) => tile.id === uuid).word;
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