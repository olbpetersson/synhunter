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
      value: null,
      label: null
    };
  },
  props: ['send', 'isLeader'],
  mounted() {
    console.log("created gameinput", this.isLeader());
    this.label= this.isLeader() ? "Answer" : "Hint";
  },
  methods: {
    submit() {
      let method = this.isLeader() ? "submit_answer" : "submit_hint";
      let payload = new JsonRpc(method, [this.value],999);
      console.log("sending submit answer", this.value);
      this.send(payload);
      console.log("is leader inna da gameinput", this.isLeader());
    }
  }
};
</script>
<style scoped>
.container {
  padding: 8px;
}
</style>