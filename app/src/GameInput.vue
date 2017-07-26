<template>
<div class="container">
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
import JsonRpc from 'JsonRpc'
export default {
  data() {
    return {
      value: null
    };
  },
  props: ['label','send'],
  created() {},
  methods: {
    submit() {
      let payload = new JsonRpc('submit_answer', [this.value],999);
      console.log("sending submit answer", this.value);
      this.send(payload);
    }
  }
};
</script>
<style scoped>
.container {
  padding: 8px;
}
</style>