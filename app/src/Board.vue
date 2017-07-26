<template>
  <md-layout>

      <button @click="chooseTile"> APA</button>
      <button @click="submitHint"> BEPA</button>

    <md-layout md-column class="container" v-if="gameState">
      <md-layout v-for="(row, i) in parseInt(height)" key="i" class="row">
        <md-layout v-for="(tile, j) in parseInt(width)" key="j" class="tile" md-gutter>
          <tile id="a" word="a"/>
        </md-layout>
      </md-layout>
    </md-layout>
    <md-layout md-column md-align="center" md-vertical-align="center">
      <md-spinner md-indeterminate></md-spinner>
      <span class="md-title loading-text">Herp Derp...</span>
    </md-layout>
  </md-layout>
</template>
<script>
  import Tile from 'Tile.vue';
import JsonRpc from './JsonRpc';

  // const BASE_URL = 'ws://synhunter.mirkk.eu:1337';

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
        let payload = new JsonRpc('foo', 1, 0);
        console.log("sending payload", payload);
        this.send(payload);
        //this.socket.send(payload);
      },
      chooseTile() {
        let apa = "4da2dae4-3982-482d-8e3d-42fd983ddaeb";
        let payload = new JsonRpc('choose_tile', [apa], 2);
        console.log("sending choose tile", payload);
        this.send(payload);
      },
      submitHint() {
        let bepa = "foobar";
        let payload = new JsonRpc('submit_hint', [bepa], 2);
        console.log("sending submit hint", payload);
        this.send(payload);
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