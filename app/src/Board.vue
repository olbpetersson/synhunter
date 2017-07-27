<template>
  <md-layout md-column>

    <div v-show="debug">
      <md-layout md-column>
        <md-layout>isLeader: {{isLeader}}</md-layout>
        <template v-if="gameState">
          <span>Current team</span>
          <md-layout>id={{team.id}}</md-layout>
          <md-layout>color={{team.color}}</md-layout>
          <md-layout>leader={{team.leader}}</md-layout>
          <span>Current turn</span>
          <md-layout>team: {{currentTurn.team}}</md-layout>
          <md-layout>hints: {{currentTurn.hints}}</md-layout>
          <md-layout>tile: {{currentTurn.tile}}</md-layout>
        </template>
      </md-layout>
    </div>

    <md-layout md-column v-if="player && gameStateView">
      <div v-show="isLeader || currentTurn.team === team.id">
        <md-layout md-column>
          <md-layout md-align="center" v-show="isLeader">
            <span class="md-headline">You are the leader!</span>
          </md-layout>
          <!-- hints -->
          <md-layout md-column class="container" v-show="currentTurn.team === team.id && isLeader">
            <md-layout v-for="hint in currentTurn.hints" key="hint">
              <span>Hint: {{hint}}</span>
            </md-layout>
          </md-layout>

          <md-layout md-align="center" v-show=" currentTurn.team === team.id">
            <span class="md-headline">Your teams turn!</span>
          </md-layout>
        </md-layout>
      </div>
      
      <!-- <md-layout v-show="!currentTurn.tile || currentTurn.team !== team.id"> -->

      

      <!-- game board -->
      <md-layout md-column class="container">
        <md-layout v-for="row in gameStateView" key="row" class="row">
          <md-layout v-for="tile in row" key="tile" class="tile" md-gutter>
            <tile
                :tile="tile"
                :click="chooseTile"
                :claimedByTeam="findTeam(tile.state)"
                :isSelected="currentTurn.tile === tile.id"
                :myTeam="currentTurn.team === team.id"
                :showWord="!isLeader"></tile>
          </md-layout>
        </md-layout>
      </md-layout>

      <!-- </md-layout>
      <md-layout v-show="currentTurn.tile && currentTurn.team === team"> -->
        
      <!-- !isLeader: show input -->
      <!-- <md-layout md-column class="container" v-show="!isLeader">
          Give a synonym for: {{ findTileWord(currentTurn.tile) }}
      </md-layout> -->
      
      <!-- </md-layout> -->

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
        team: null,
        player: {},
        gameState: null,
        gameStateView: null
      };
    },
    computed: {
      currentTurn() {
        return this.gameState ? this.gameState.turns[this.gameState.turns.length - 1] : null;
      },
      isLeader() {
        return this.player ? this.player.isLeader : false;
      },
      hasSelectedTile() {
        const turn = this.currentTurn;
        return  turn ? turn.tile : false;
      }
    },
    props: ['width', 'height', 'send', 'debug'],
    mounted() {
      this.$on('game_state', (state) => {

        this.gameState = state;
        if (this.currentTurn.spyhint &&
            !this.currentTurn.hints.find(hint => hint === this.currentTurn.spyhint)) {
          let index = Math.floor(Math.random() * this.currentTurn.hints.length);
          this.currentTurn.hints.splice(index, 0, this.currentTurn.spyhint);
        }
        this.gameStateView = {};
        for (let tile of state.tiles) {
          if (!this.gameStateView[tile.position[1]]) {
            this.gameStateView[tile.position[1]] = {};
          }
          this.gameStateView[tile.position[1]][tile.position[0]] = tile;
        }
        this.updatePlayerState();
      });
      this.$on('create_player', (uuid) => {
        this.player.uuid = uuid;
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
        if(this.player && this.gameState){
          this.gameState.teams.forEach((team) => {
            team.players.forEach((uuid) => {
              if (uuid === this.player.uuid) {
                this.$set(this.player, 'isLeader', false);
                this.$set(this.player, 'team', team.color);
                this.team = team;
              }
            });
            if (team.leader === this.player.uuid) {
              this.$set(this.player, 'isLeader', true);
              this.$set(this.player, 'team', team.color);
              this.team = team;
            }
          });
        }
      },
      findTeam(id) {
        return this.gameState.teams.find(team => team.id === id);
      },
      findTileWord(uuid) {
        let tile = this.gameState.tiles.find((tile) => tile.id === uuid);
        return tile ? tile.word : null;
      },
      isSelectedTile(uuid){
        let tile = this.findTile(uuid);
        return this.currentTurn ? this.currentTurn.tile && this.currentTurn.tile === uuid : false;
      },
      findTile(uuid){
        let tile = this.gameState.tiles.find((tile) => tile.id === uuid);
        return tile;
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