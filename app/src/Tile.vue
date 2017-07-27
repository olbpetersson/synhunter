<template>
  <md-card :style="{background}">
    <md-button v-show="!team" @click="click(tile.id)" :disabled="!enabled"></md-button>
    <span v-show="showWord">{{tile.word}}</span>
  </md-card>
</template>
<script>
  export default {
    props: ['tile', 'click', 'enabled', 'team', 'isSelected', "showWord"],
    computed: {
      background() {
        if(this.isSelected) {
          return 'yellow';
        }
        return this.team ? this.team.color : 'url(/static/logo.png) no-repeat center/contain';
      }
    }
  };
</script>
<style scoped>
  .md-button {
    min-width: 0;
    height: 100%;
    margin: 0;
    padding: 0;
  }
  span {
    padding-top: 16px;
    text-align: center;
  }
</style>