<template>
  <md-card :style="{background}">
    <md-button v-show="claimedByTeam || !showWord" @click="click(tile.id)" :disabled="!myTeam || showWord"></md-button>
    <span v-show="isSelected && showWord">{{tile.word}}</span>
  </md-card>
</template>
<script>
  export default {
    props: ['tile', 'click', 'claimedByTeam', 'isSelected', "showWord", "myTeam"],
    computed: {
      background() {
        if(this.claimedByTeam) {
          return this.claimedByTeam.color;
        } else if (this.isSelected) {
          return 'yellow';
        }
        return 'url(/static/logo.png) no-repeat center/contain';
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