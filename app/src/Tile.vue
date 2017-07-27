<template>
  <md-card>
    <md-button @click="click(tile.id)" :disabled="!enabled">{{enabled ? 'enabled' : 'disabled'}}</md-button>
  </md-card>
</template>
<script>
  export default {
    props: ['tile', 'click', 'enabled']
  };
</script>
<style scoped>
  .md-card {
    background-image: url(/static/logo.png);
    background-position: center;
    background-size: contain;
    background-repeat: no-repeat;
  }
  .md-button {
    min-width: 0;
    height: 100%;
    margin: 0;
    padding: 0;
  }
  .card-logo {
    display: block;
    margin: 0 auto;
    vertical-align: middle;
  }
</style>