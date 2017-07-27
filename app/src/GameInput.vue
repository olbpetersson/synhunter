<template>
<div class="container">
  <form novalidate @submit.stop.prevent="submit">
    <md-input-container>
      <label>{{label}}</label>
      <md-input :disabled="!inputEnabled" v-model="value"></md-input>
    </md-input-container>
    <md-button :disabled="!inputEnabled" type="submit" class="md-raised">Submit {{inputEnabled}}</md-button>
  </form>
</div>
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
  props: ['submit']
};
</script>
<style scoped>
.container {
  padding: 8px;
}
</style>