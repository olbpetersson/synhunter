<template>
<div>
  <form novalidate @submit.stop.prevent="submit">
    <md-input-container>
      <label>{{label}}</label>
      <md-input v-model="value"></md-input>
    </md-input-container>
    <md-button type="submit" class="md-raised">Submit</md-button>
  </form>
</div>
</template>
<script>
export default {
  data() {
    return {
      value: null
    };
  },
  props: ['label'],
  created() {},
  methods: {
    submit() {
      // todo: send to server
    }
  }
};
</script>
<style>
</style>