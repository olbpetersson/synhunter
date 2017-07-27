<template>
<form novalidate @submit.stop.prevent="submit(), (value = '')">
  <md-layout class="container" v-show="enabled">
    <md-input-container :style="{color}">
      <label :style="{color}">{{label}}</label>
      <md-input v-model="value" :style="{color}"></md-input>
    </md-input-container>
    <md-layout md-align="center">
      <md-button type="submit" class="md-raised" :style="{color}">
        <md-icon>send</md-icon>
        Submit
      </md-button>
    </md-layout>
  </md-layout>
  <md-layout md-column v-show="!enabled">
    <md-layout md-align="center">
      <md-spinner md-indeterminate></md-spinner>
    </md-layout>
    <md-layout md-align="center">
      <span>Waiting...</span>
    </md-layout>
  </md-layout>
</form>
</template>
<script>
import JsonRpc from 'JsonRpc'
export default {
  data() {
    return {
      value: null
    };
  },
  props: ['submit', 'color', 'enabled', 'label']
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