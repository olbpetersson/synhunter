<template>
<form novalidate @submit.stop.prevent="submit">
  <md-layout class="container">
    <md-input-container :style="{color}">
      <label :style="{color}">{{label}}</label>
      <md-input :disabled="false && !inputEnabled" v-model="value" :style="{color}"></md-input>
    </md-input-container>
    <md-layout md-align="center">
      <md-button :disabled="false && !inputEnabled" type="submit" class="md-raised" :style="{color}">
        <md-icon>send</md-icon>
        Submit
      </md-button>
    </md-layout>
  </md-layout>
</form>
</template>
<script>
import JsonRpc from 'JsonRpc'
export default {
  data() {
    return {
      value: null,
      label: "Answer/Hint",
      inputEnabled: false
    };
  },
  props: ['submit', 'color']
};
</script>
<style scoped>
.container {
  padding: 8px;
}
.container .md-theme-default.md-input-container.md-input-focused:after {
  background-color: transparent;
}
</style>