<template>
  <md-card>
    <md-button @click="foo">{{word}}</md-button>
  </md-card>
</template>
<script>
  export default {
    data() {
      return {};
    },
    props: ['id', 'word'],
    methods: {
      foo() {
        console.log(this.id, this.word);
      }
    }
  };
</script>
<style scoped>
  /*.container {
    padding: 8px;
  }*/
</style>