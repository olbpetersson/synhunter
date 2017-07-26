<template>
  <md-layout>
    <md-layout md-column class="container" v-if="gameState">
      <md-layout v-for="(row, i) in parseInt(height)" key="i" class="row">
        <md-layout v-for="(tile, j) in parseInt(width)" key="j" class="tile" md-gutter>
          <tile id="a" word="a"/>
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

  export default {
    data() {
      return {
        currentPlayer: null,
        gameState: null,
      };
    },
    props: ['width', 'height', 'send'],
    created() {
      console.log('Board initialized', this.width, this.height);

      this.$on('createPlayer', (e) => this.currentPlayer = e)
      this.$on('game_state', state => (this.gameState = state))
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
        console.log("sending payload", payload);
        this.send(JSON.stringify(payload));
        //this.socket.send(JSON.stringify(payload));
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