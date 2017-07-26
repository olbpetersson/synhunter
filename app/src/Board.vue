<template>
  <md-layout md-column class="container">
    <md-layout v-for="(row, i) in parseInt(height)" class="row">
      <md-layout v-for="(tile, j) in parseInt(width)" class="tile" md-gutter>
        <md-card>
          <md-button @click="ping(i, j)">({{i}},{{j}})</md-button>
        </md-card>
      </md-layout>
    </md-layout>
  </md-layout>
</template>

<script>


  // const BASE_URL = 'ws://synhunter.mirkk.eu:1337';

  export default {
    data() {
      return {
        //socket: null,
        currentPlayer: null
      };
    },
    props: ['width', 'height'],
    created() {
      console.log('Board initialized', this.width, this.height);

      this.$on('createPlayer', (e) => this.currentPlayer = e)
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
        this.$broadcast('send', JSON.stringify(payload));

        //this.socket.send(JSON.stringify(payload));
      }
    }
  };
</script>
<style scoped>
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