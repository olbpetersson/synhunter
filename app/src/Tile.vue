<template>
  <md-card>
    <md-button class="foobar" @click="click(tile.id)" v-if="!tile.state">{{tile.word}}</md-button>
    <img class="card-logo" v-else src="/static/logo.png">
  </md-card>
</template>
<script>
  export default {
    data() {
      return {};
    },
    props: ['tile', 'click'],
    methods: {
      foo() {
        // console.log(this.id, this.word);
      }
    }
  };
</script>
<style scoped>
  /*.container {
    padding: 8px;
  }*/
  .foobar {
    min-width: 0;
    padding: 0;
  }
  .card-logo {
    display: block;
    margin: 0 auto;
    vertical-align: middle;
  }
</style>